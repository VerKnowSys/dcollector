// build.rs

fn main() {
    println!(
        "cargo:rustc-link-lib=dylib=pq\ncargo:rustc-link-search=native=/opt/homebrew/lib/\n"
    );
    println!(
        "cargo:rustc-link-lib=dylib=pq\ncargo:rustc-link-search=native=/Software/Postgresql13/lib/\n"
    );
}
