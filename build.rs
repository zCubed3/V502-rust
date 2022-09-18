extern crate cmake;

fn main() {
    let dst = cmake::build("V502");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=V502");
}