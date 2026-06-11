use dioxus::prelude::*;
fn main() {
    let a = asset!("/assets/favicon.ico");
    println!("{}", a.to_string());
}
