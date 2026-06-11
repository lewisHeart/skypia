import os
from pathlib import Path

anim_dir = Path("assets/emojis_anim")
static_dir = Path("assets/emojis")

anim_files = [f.stem for f in anim_dir.glob("*.webp")]
static_files = [f.stem for f in static_dir.glob("*.svg")]

out = ["use dioxus::prelude::*;"]

out.append("pub fn get_emoji_anim_asset(name: &str) -> String {")
out.append("    match name {")
for name in sorted(anim_files):
    out.append(f'        "{name}" => asset!("/assets/emojis_anim/{name}.webp").to_string(),')
out.append('        _ => "".to_string(),')
out.append("    }")
out.append("}")

out.append("pub fn get_emoji_static_asset(name: &str) -> String {")
out.append("    match name {")
for name in sorted(static_files):
    out.append(f'        "{name}" => asset!("/assets/emojis/{name}.svg").to_string(),')
out.append('        _ => "".to_string(),')
out.append("    }")
out.append("}")

with open("src/emoji_assets.rs", "w") as f:
    f.write("\n".join(out) + "\n")
