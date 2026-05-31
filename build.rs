use std::env::var;
fn main() {
    if
        var("CARGO_FEATURE_REPL").is_ok() &&
        var("CARGO_FEATURE_STD").is_ok()
    {
        println!("cargo:rustc-cfg=__bytes");
    }
}