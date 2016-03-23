#[cfg(target_os="windows")]
fn main() {
    println!("cargo:rustc-link-search={}\\lib", env!("CARGO_MANIFEST_DIR"));
}
