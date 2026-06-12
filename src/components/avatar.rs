use dioxus::prelude::*;
use std::sync::{Mutex, LazyLock};
use std::collections::HashMap;
use base64::Engine;

pub static AVATAR_CACHE: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

pub fn invalidate_avatar_cache(url: &str) {
    let mut cache = AVATAR_CACHE.lock().unwrap();
    cache.remove(url);
    let base_url = url.split('?').next().unwrap_or(url);
    cache.retain(|k, _| !k.starts_with(base_url));
}

#[component]
pub fn Avatar(url: Option<String>, size: usize) -> Element {
    let final_url = match url {
        Some(ref u) if u.starts_with("http") => u.to_string(),
        Some(ref u)
            if u.starts_with("/assets/")
                || u.starts_with("assets/")
                || u.starts_with("/_assets/")
                || u.starts_with("_assets/")
                || u.starts_with("dioxus-asset://") =>
        {
            u.to_string()
        }
        Some(ref u) if !u.is_empty() => format!("{}{}", crate::services::api::SERVER_BASE_URL, u),
        _ => "".to_string(),
    };

    // Criamos um sinal reativo para a URL final para que o use_resource reaja às suas mudanças
    let mut url_signal = use_signal(|| final_url.clone());
    if url_signal() != final_url {
        url_signal.set(final_url.clone());
    }

    // Obter do cache de forma síncrona
    let cached = if final_url.is_empty() {
        None
    } else {
        AVATAR_CACHE.lock().unwrap().get(&final_url).cloned()
    };

    // Recurso reativo para fazer o fetch assíncrono em segundo plano se não estiver em cache
    let avatar_resource = use_resource(move || {
        let url = url_signal();
        async move {
            if url.is_empty() {
                return None;
            }
            // Verifica no cache global antes de fazer o fetch
            let cached_in_resource = AVATAR_CACHE.lock().unwrap().get(&url).cloned();
            if let Some(c) = cached_in_resource {
                return Some(c);
            }
            if !url.starts_with("http") {
                return Some(url);
            }
            match reqwest::get(&url).await {
                Ok(resp) => {
                    if let Ok(bytes) = resp.bytes().await {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                        let mime = if url.contains(".gif") {
                            "image/gif"
                        } else if url.contains(".png") {
                            "image/png"
                        } else {
                            "image/jpeg"
                        };
                        let data_uri = format!("data:{};base64,{}", mime, b64);
                        AVATAR_CACHE.lock().unwrap().insert(url.clone(), data_uri.clone());
                        Some(data_uri)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
    });

    let display_url = if let Some(ref c) = cached {
        c.clone()
    } else if let Some(Some(ref res_url)) = *avatar_resource.value().read() {
        res_url.clone()
    } else {
        final_url.clone()
    };

    let is_loading = cached.is_none() && final_url.starts_with("http") && avatar_resource.value().read().is_none();
    
    // Se o fetch assíncrono terminou e retornou None, consideramos falha e exibimos o fallback.
    let show_fallback = display_url.is_empty() || (!is_loading && cached.is_none() && final_url.starts_with("http") && avatar_resource.value().read().as_ref().map(|x| x.is_none()).unwrap_or(true));

    if !show_fallback {
        rsx! {
            img {
                src: "{display_url}",
                width: "{size}px",
                height: "{size}px",
                class: "rounded-[4px] object-cover flex-shrink-0 border border-slate-350 shadow-inner",
                alt: "Avatar"
            }
        }
    } else {
        rsx! {
            svg {
                view_box: "0 0 100 100",
                width: "{size}px",
                height: "{size}px",
                class: "rounded-[4px] flex-shrink-0 border border-slate-300 shadow-sm",
                defs {
                    linearGradient { id: "msnGrad", x1: "0%", y1: "0%", x2: "100%", y2: "100%",
                        stop { offset: "0%", stop_color: "#e6f2ff" }
                        stop { offset: "100%", stop_color: "#bcd6f7" }
                    }
                }
                rect { width: "100", height: "100", rx: "4", fill: "url(#msnGrad)" }
                // Boneco clássico do MSN azul/verde
                circle { cx: "44", cy: "38", r: "13", fill: "#3b82f6" }
                path { d: "M20 76 C20 58, 68 58, 68 76 Z", fill: "#3b82f6" }
                circle { cx: "66", cy: "48", r: "10", fill: "#22c55e" }
                path { d: "M48 76 C48 64, 84 64, 84 76 Z", fill: "#22c55e" }
            }
        }
    }
}

pub fn render_avatar(url_opt: Option<&str>, size_px: usize) -> Element {
    rsx! {
        Avatar {
            url: url_opt.map(|s| s.to_string()),
            size: size_px,
        }
    }
}
