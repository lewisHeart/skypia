use dioxus::prelude::*;

pub fn play_sound(sound_type: &str) {
    let js_code = match sound_type {
        "online" => r#"
            try {
                let ctx = new (window.AudioContext || window.webkitAudioContext)();
                let playTone = (freq, start, duration, volume) => {
                    let osc = ctx.createOscillator();
                    let gain = ctx.createGain();
                    osc.type = 'sine';
                    osc.frequency.setValueAtTime(freq, ctx.currentTime + start);
                    gain.gain.setValueAtTime(volume, ctx.currentTime + start);
                    gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + start + duration);
                    osc.connect(gain);
                    gain.connect(ctx.destination);
                    osc.start(ctx.currentTime + start);
                    osc.stop(ctx.currentTime + start + duration);
                };
                playTone(660, 0, 0.08, 0.12);
                playTone(880, 0.08, 0.16, 0.12);
            } catch (e) {
                console.error("Audio error:", e);
            }
        "#,
        "message" => r#"
            try {
                let ctx = new (window.AudioContext || window.webkitAudioContext)();
                let playTone = (freq, start, duration, volume) => {
                    let osc = ctx.createOscillator();
                    let gain = ctx.createGain();
                    osc.type = 'sine';
                    osc.frequency.setValueAtTime(freq, ctx.currentTime + start);
                    gain.gain.setValueAtTime(volume, ctx.currentTime + start);
                    gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + start + duration);
                    osc.connect(gain);
                    gain.connect(ctx.destination);
                    osc.start(ctx.currentTime + start);
                    osc.stop(ctx.currentTime + start + duration);
                };
                playTone(1020, 0, 0.25, 0.15);
                playTone(1360, 0.02, 0.20, 0.08);
            } catch (e) {
                console.error("Audio error:", e);
            }
        "#,
        "nudge" => r#"
            try {
                let ctx = new (window.AudioContext || window.webkitAudioContext)();
                let t = ctx.currentTime;
                let osc = ctx.createOscillator();
                let gain = ctx.createGain();
                
                osc.type = 'sawtooth';
                osc.frequency.setValueAtTime(80, t);
                
                // Modulate frequency to create vibration effect
                for (let i = 0; i < 24; i++) {
                    osc.frequency.setValueAtTime(i % 2 === 0 ? 70 : 95, t + i * 0.015);
                }
                
                gain.gain.setValueAtTime(0.25, t);
                gain.gain.linearRampToValueAtTime(0.001, t + 0.40);
                
                osc.connect(gain);
                gain.connect(ctx.destination);
                osc.start(t);
                osc.stop(t + 0.40);
            } catch (e) {
                console.error("Audio error:", e);
            }
        "#,
        _ => "",
    };

    if !js_code.is_empty() {
        // Execute javascript fire-and-forget
        let _ = document::eval(js_code);
    }
}
