use std::ffi::CStr;

use fontique::{Blob, Collection};
use libloading::{Library, Symbol};

use crate::Error;

type OpenFn = unsafe extern "C" fn() -> *mut std::ffi::c_void;
type NextFn = unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void;
type CloseIterFn = unsafe extern "C" fn(*mut std::ffi::c_void);
type GetPathFn = unsafe extern "C" fn(*const std::ffi::c_void) -> *const std::os::raw::c_char;
type CloseFontFn = unsafe extern "C" fn(*mut std::ffi::c_void);

pub(crate) fn load_system_fonts(collection: &mut Collection) -> Result<Vec<Blob<u8>>, Error> {
    let lib = unsafe {
        Library::new("libandroid.so")
            .map_err(|_| Error::NotSupported("libandroid.so not found (pre-API 29?)"))?
    };

    let open: Symbol<OpenFn> = unsafe {
        lib.get(b"ASystemFontIterator_open")
            .map_err(|_| Error::NotSupported("ASystemFontIterator_open not available (pre-API 29?)"))?
    };
    let next: Symbol<NextFn> = unsafe {
        lib.get(b"ASystemFontIterator_next")
            .map_err(|_| Error::NotSupported("ASystemFontIterator_next not available"))?
    };
    let close_iter: Symbol<CloseIterFn> = unsafe {
        lib.get(b"ASystemFontIterator_close")
            .map_err(|_| Error::NotSupported("ASystemFontIterator_close not available"))?
    };
    let get_path: Symbol<GetPathFn> = unsafe {
        lib.get(b"AFont_getFontFilePath")
            .map_err(|_| Error::NotSupported("AFont_getFontFilePath not available"))?
    };
    let close_font: Symbol<CloseFontFn> = unsafe {
        lib.get(b"AFont_close")
            .map_err(|_| Error::NotSupported("AFont_close not available"))?
    };

    let mut font_data = Vec::new();
    let iter = unsafe { open() };
    if iter.is_null() {
        return Err(Error::Android("ASystemFontIterator_open returned null".into()));
    }

    loop {
        let font = unsafe { next(iter) };
        if font.is_null() {
            break;
        }

        let path_ptr = unsafe { get_path(font) };
        if !path_ptr.is_null() {
            let path = match unsafe { CStr::from_ptr(path_ptr) }.to_str() {
                Ok(p) => p,
                Err(_) => {
                    unsafe { close_font(font) };
                    continue;
                }
            };

            if !path.is_empty() {
                match std::fs::read(path) {
                    Ok(bytes) => {
                        let blob: Blob<u8> = bytes.into();
                        collection.register_fonts(blob.clone(), None);
                        font_data.push(blob);
                    }
                    Err(_) => {
                        // skip unreadable font — best effort
                    }
                }
            }
        }

        unsafe { close_font(font) };
    }

    unsafe { close_iter(iter) };
    Ok(font_data)
}
