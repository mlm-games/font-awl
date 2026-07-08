use fontique::{Blob, Collection};

pub(crate) fn register_defaults(_col: &mut Collection) -> Vec<Blob<u8>> {
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
#[allow(unused_variables, unused_mut)]
fn register_basic(col: &mut Collection) -> Vec<Blob<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_opensans)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/OpenSans-Regular.ttf"
        ));
        let blob: Blob<u8> = raw.to_vec().into();
        let families = col.register_fonts(blob.clone(), None);
        col.set_generic_families(
            fontique::GenericFamily::SansSerif,
            families.iter().map(|(fid, _)| *fid),
        );
        out.push(blob);
    }
    #[cfg(font_awl_symbols2)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoSansSymbols2-Regular.ttf"
        ));
        let blob: Blob<u8> = raw.to_vec().into();
        col.register_fonts(blob.clone(), None);
        out.push(blob);
    }
    out
}

#[cfg(feature = "emoji")]
#[allow(unused_variables, unused_mut)]
fn register_emoji(col: &mut Collection) -> Vec<Blob<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_emoji)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoColorEmoji-Regular.ttf"
        ));
        let blob: Blob<u8> = raw.to_vec().into();
        let families = col.register_fonts(blob.clone(), None);
        col.set_generic_families(
            fontique::GenericFamily::Emoji,
            families.iter().map(|(fid, _)| *fid),
        );
        out.push(blob);
    }
    out
}

#[cfg(feature = "cjk")]
#[allow(unused_variables, unused_mut)]
fn register_cjk(col: &mut Collection) -> Vec<Blob<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_cjk)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/NotoSansCJK-Regular.ttc"
        ));
        let blob: Blob<u8> = raw.to_vec().into();
        let families = col.register_fonts(blob.clone(), None);
        col.set_generic_families(
            fontique::GenericFamily::SansSerif,
            families.iter().map(|(fid, _)| *fid),
        );
        out.push(blob);
    }
    out
}

#[cfg(feature = "monospace")]
#[allow(unused_variables, unused_mut)]
fn register_monospace(col: &mut Collection) -> Vec<Blob<u8>> {
    let mut out = Vec::new();
    #[cfg(font_awl_mono)]
    {
        let raw = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/JetBrainsMono-Regular.ttf"
        ));
        let blob: Blob<u8> = raw.to_vec().into();
        let families = col.register_fonts(blob.clone(), None);
        col.set_generic_families(
            fontique::GenericFamily::Monospace,
            families.iter().map(|(fid, _)| *fid),
        );
        out.push(blob);
    }
    out
}
