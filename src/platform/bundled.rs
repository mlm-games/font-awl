use fontique::Collection;

pub(crate) fn register_defaults(_col: &mut Collection) {
    #[cfg(feature = "basic")]
    register_basic(col);
    #[cfg(feature = "emoji")]
    register_emoji(col);
    #[cfg(feature = "cjk")]
    register_cjk(col);
}

#[cfg(feature = "basic")]
fn register_basic(col: &mut Collection) {
    #[cfg(font_awl_basic)]
    {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoSans-Latin.ttf"
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
            "/fonts/NotoColorEmoji.ttf"
        ));
        let blob: fontique::Blob<u8> = data.to_vec().into();
        col.register_fonts(blob, None);
    }
}

#[cfg(feature = "cjk")]
fn register_cjk(col: &mut Collection) {
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
