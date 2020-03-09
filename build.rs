fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-O3")
        .flag("-larmadillo")
        .file("src/arma_ffi.cpp")
        .compile("arma");

    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=armadillo");
}
