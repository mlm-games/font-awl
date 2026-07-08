use fontique::Blob;

use crate::Error;

mod bundled;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_os = "android")]
mod android;

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

#[cfg(target_os = "android")]
pub(crate) fn load_system_fonts(col: &mut fontique::Collection) -> Result<Vec<Blob<u8>>, Error> {
    android::load_system_fonts(col)
}

#[cfg(not(target_os = "android"))]
pub(crate) fn load_system_fonts(_col: &mut fontique::Collection) -> Result<Vec<Blob<u8>>, Error> {
    Ok(Vec::new())
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn load_web_fonts(col: &mut fontique::Collection) -> Result<Vec<Blob<u8>>, Error> {
    web::load_local_fonts(col).await
}
