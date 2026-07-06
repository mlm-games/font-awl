use fontique::Collection;

pub(crate) fn register_defaults(_col: &mut Collection) -> Vec<Vec<u8>> {
    #[allow(unused_mut)]
    let mut out = Vec::new();
    #[cfg(feature = "basic")]
    out.extend(register_basic(_col));
    #[cfg(feature = "emoji")]
    out.extend(register_emoji(_col));
    #[cfg(feature = "cjk")]
    out.extend(register_cjk(_col));
    #[cfg(feature = "monospace")]
    out.extend(register_monospace(_col));
    out
}

#[cfg(feature = "basic")]
fn register_basic(col: &mut Collection) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_opensans)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/OpenSans-Regular.ttf"
        ));
        let bytes = raw.to_vec();
        let blob: fontique::Blob<u8> = bytes.clone().into();
        col.register_fonts(blob, None);
        out.push(bytes);
    }
    #[cfg(font_awl_symbols2)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoSansSymbols2-Regular.ttf"
        ));
        let bytes = raw.to_vec();
        let blob: fontique::Blob<u8> = bytes.clone().into();
        col.register_fonts(blob, None);
        out.push(bytes);
    }
    out
}

#[cfg(feature = "emoji")]
fn register_emoji(col: &mut Collection) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_emoji)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoColorEmoji-Regular.ttf"
        ));
        let bytes = raw.to_vec();
        let blob: fontique::Blob<u8> = bytes.clone().into();
        col.register_fonts(blob, None);
        out.push(bytes);
    }
    out
}

#[cfg(feature = "cjk")]
fn register_cjk(col: &mut Collection) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_cjk)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoSansCJK-Regular.ttc"
        ));
        let bytes = raw.to_vec();
        let blob: fontique::Blob<u8> = bytes.clone().into();
        col.register_fonts(blob, None);
        out.push(bytes);
    }
    out
}

#[cfg(feature = "monospace")]
fn register_monospace(col: &mut Collection) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_mono)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/JetBrainsMono-Regular.ttf"
        ));
        let bytes = raw.to_vec();
        let blob: fontique::Blob<u8> = bytes.clone().into();
        col.register_fonts(blob, None);
        out.push(bytes);
    }
    out
}
