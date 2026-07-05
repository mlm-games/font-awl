use std::ffi::CStr;

use fontique::{Blob, Collection};

use crate::Error;

pub(crate) fn load_system_fonts(collection: &mut Collection) -> Result<(), Error> {
    let iter = unsafe { ASystemFontIterator_open() };
    if iter.is_null() {
        return Err(Error::Android("ASystemFontIterator_open returned null".into()));
    }

    loop {
        let font = unsafe { ASystemFontIterator_next(iter) };
        if font.is_null() {
            break;
        }

        let path_ptr = unsafe { AFont_getFontFilePath(font) };
        if !path_ptr.is_null() {
            let path = unsafe { CStr::from_ptr(path_ptr) }
                .to_str()
                .map_err(|_| Error::Android("font path is not valid UTF-8".into()))?;

            if !path.is_empty() {
                let data = std::fs::read(path)
                    .map_err(|e| Error::platform("reading font file", e))?;
                let blob: Blob<u8> = data.into();
                collection.register_fonts(blob, None);
            }
        }

        unsafe { AFont_close(font) };
    }

    unsafe { ASystemFontIterator_close(iter) };
    Ok(())
}

#[link(name = "android")]
extern "C" {
    fn ASystemFontIterator_open() -> *mut std::ffi::c_void;
    fn ASystemFontIterator_next(it: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn ASystemFontIterator_close(it: *mut std::ffi::c_void);
    fn AFont_getFontFilePath(font: *mut std::ffi::c_void) -> *const std::os::raw::c_char;
    fn AFont_close(font: *mut std::ffi::c_void);
}
