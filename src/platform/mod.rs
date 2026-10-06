use fontique::Blob;

use crate::Error;

mod bundled;

#[cfg(all(target_os = "android", feature = "system"))]
mod android;
#[cfg(all(target_arch = "wasm32", feature = "local-fonts"))]
mod web;

#[cfg(any(target_arch = "wasm32", target_os = "android"))]
pub(crate) fn system_fonts_at_init() -> bool {
    false
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
pub(crate) fn system_fonts_at_init() -> bool {
    cfg!(feature = "system")
}

pub(crate) fn register_default_fonts(col: &mut fontique::Collection) -> Vec<Blob<u8>> {
    bundled::register_defaults(col)
}

#[cfg(all(target_os = "android", feature = "system"))]
pub(crate) fn load_system_fonts(col: &mut fontique::Collection) -> Result<Vec<Blob<u8>>, Error> {
    android::load_system_fonts(col)
}

#[cfg(not(all(target_os = "android", feature = "system")))]
pub(crate) fn load_system_fonts(_col: &mut fontique::Collection) -> Result<Vec<Blob<u8>>, Error> {
    Ok(Vec::new())
}

#[cfg(all(target_arch = "wasm32", feature = "local-fonts"))]
pub(crate) async fn load_web_fonts(col: &mut fontique::Collection) -> Result<Vec<Blob<u8>>, Error> {
    web::load_local_fonts(col).await
}
