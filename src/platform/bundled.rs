use fontique::Collection;

pub(crate) fn register_defaults(_col: &mut Collection) {
    #[cfg(feature = "basic")]
    register_basic(_col);
    #[cfg(feature = "emoji")]
    register_emoji(_col);
    #[cfg(feature = "cjk")]
    register_cjk(_col);
}

#[cfg(feature = "basic")]
fn register_basic(col: &mut Collection) {
    #[cfg(font_awl_opensans)]
    {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/OpenSans-Regular.ttf"
        ));
        let blob: fontique::Blob<u8> = data.to_vec().into();
        col.register_fonts(blob, None);
    }
    #[cfg(font_awl_symbols2)]
    {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoSansSymbols2-Regular.ttf"
        ));
        let blob: fontique::Blob<u8> = data.to_vec().into();
        col.register_fonts(blob, None);
    }
}

#[cfg(feature = "emoji")]
fn register_emoji(col: &mut Collection) {
    #[cfg(font_awl_emoji)]
    {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoColorEmoji-Regular.ttf"
        ));
        let blob: fontique::Blob<u8> = data.to_vec().into();
        col.register_fonts(blob, None);
    }
}

#[cfg(feature = "cjk")]
fn register_cjk(_col: &mut Collection) {
    #[cfg(font_awl_cjk)]
    {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoSansCJK-Regular.ttc"
        ));
        let blob: fontique::Blob<u8> = data.to_vec().into();
        col.register_fonts(blob, None);
    }
}
