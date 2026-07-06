fn main() {
    println!("cargo::rustc-check-cfg=cfg(font_awl_opensans)");
    println!("cargo::rustc-check-cfg=cfg(font_awl_symbols2)");
    println!("cargo::rustc-check-cfg=cfg(font_awl_emoji)");
    println!("cargo::rustc-check-cfg=cfg(font_awl_cjk)");
    println!("cargo::rustc-check-cfg=cfg(font_awl_mono)");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let font_dir = std::path::Path::new(&manifest_dir).join("fonts");

    #[rustfmt::skip]
    let checks = [
        ("OpenSans-Regular.ttf",         "font_awl_opensans",   "basic"),
        ("NotoSansSymbols2-Regular.ttf", "font_awl_symbols2",   "basic"),
        ("NotoColorEmoji-Regular.ttf",   "font_awl_emoji",      "emoji"),
        ("NotoSansCJK-Regular.ttc",      "font_awl_cjk",        "cjk"),
        ("JetBrainsMono-Regular.ttf",    "font_awl_mono",       "monospace"),
    ];

    for (file, cfg_flag, feature) in &checks {
        let path = font_dir.join(file);
        if path.exists() {
            println!("cargo:rustc-cfg={cfg_flag}");
        } else {
            let feat_var = format!("CARGO_FEATURE_{}", feature.to_uppercase());
            if std::env::var(&feat_var).is_ok() {
                println!("cargo:warning=font-awl: missing '{file}' needed by '{feature}' feature");
            }
        }
    }
}
