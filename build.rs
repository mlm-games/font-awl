fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let font_dir = std::path::Path::new(&manifest_dir).join("fonts");

    #[rustfmt::skip]
    let checks = [
        ("NotoSans-Latin.ttf", "font_awl_basic", "basic"),
        ("NotoColorEmoji.ttf", "font_awl_emoji", "emoji"),
        ("NotoSansCJK-Regular.ttc", "font_awl_cjk", "cjk"),
    ];

    for (file, cfg_flag, feature) in &checks {
        let path = font_dir.join(file);
        if path.exists() {
            println!("cargo:rustc-cfg={cfg_flag}");
        } else {
            let feat_var = format!("CARGO_FEATURE_{}", feature.to_uppercase());
            if std::env::var(&feat_var).is_ok() {
                println!("cargo:warning=font-awl: missing '{file}' needed by '{feature}' feature. Run scripts/download-fonts.sh");
            }
        }
    }
}
