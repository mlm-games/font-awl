use cfg_if::cfg_if;

use crate::Error;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod web;
        mod bundled;

        pub(crate) fn system_fonts_at_init() -> bool { false }

        pub(crate) fn load_system_fonts(_col: &mut fontique::Collection) -> Result<(), Error> {
            Ok(())
        }

        pub(crate) async fn load_web_fonts(col: &mut fontique::Collection) -> Result<(), Error> {
            web::load_local_fonts(col).await
        }

        pub(crate) fn register_default_fonts(col: &mut fontique::Collection) {
            bundled::register_defaults(col);
        }
    } else if #[cfg(target_os = "android")] {
        mod android;
        mod bundled;

        pub(crate) fn system_fonts_at_init() -> bool { false }

        pub(crate) fn load_system_fonts(col: &mut fontique::Collection) -> Result<(), Error> {
            android::load_system_fonts(col)
        }

        pub(crate) fn register_default_fonts(col: &mut fontique::Collection) {
            bundled::register_defaults(col);
        }
    } else {
        // Desktop: Linux, macOS, Windows
        mod bundled;

        pub(crate) fn system_fonts_at_init() -> bool {
            cfg!(feature = "system")
        }

        pub(crate) fn load_system_fonts(_col: &mut fontique::Collection) -> Result<(), Error> {
            Ok(())
        }

        pub(crate) fn register_default_fonts(col: &mut fontique::Collection) {
            bundled::register_defaults(col);
        }
    }
}
