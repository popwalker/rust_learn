fn main() {
    // 告诉rustc需要link bz2
    println!("cargo:rustc-link-lib=bz2");
    // 告诉cargo 当 wrapper.h变化时重新运行
    println!("cargo:rerun-if-changed=wrapper.h");

    // 配置bindgen，并生成Bindings结构
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // 生成Rust代码
    bindings.write_to_file("src/bindings.rs").expect("Failed to write bindings");
}