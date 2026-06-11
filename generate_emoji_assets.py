import os

anim_files = [f for f in os.listdir('public/emojis_anim_gif') if f.endswith('.gif')]
static_files = [f for f in os.listdir('public/emojis') if f.endswith('.svg') or f.endswith('.png')]

with open('src/emoji_assets.rs', 'w') as f:
    f.write('use dioxus::prelude::*;\n\n')
    f.write('pub fn get_emoji_anim_asset(name: &str) -> String {\n')
    f.write('    match name {\n')
    for file in sorted(anim_files):
        name = os.path.splitext(file)[0]
        f.write(f'        "{name}" => asset!("/assets/emojis_anim_gif/{file}").to_string(),\n')
    f.write('        _ => "".to_string(),\n')
    f.write('    }\n')
    f.write('}\n\n')

    f.write('pub fn get_emoji_static_asset(name: &str) -> String {\n')
    f.write('    match name {\n')
    for file in sorted(static_files):
        name = os.path.splitext(file)[0]
        f.write(f'        "{name}" => asset!("/assets/emojis/{file}").to_string(),\n')
    f.write('        _ => "".to_string(),\n')
    f.write('    }\n')
    f.write('}\n')
