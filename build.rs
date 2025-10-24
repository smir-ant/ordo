fn main() {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo::rerun-if-changed=i18n");
    
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations("i18n");

    slint_build::compile_with_config("ui/main.slint", config).unwrap();

}
