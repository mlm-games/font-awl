use fontique::{Blob, Collection, FontInfoOverride};
use js_sys::{Function, Promise, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::Error;

pub(crate) async fn load_local_fonts(collection: &mut Collection) -> Result<(), Error> {
    let window = web_sys::window().ok_or_else(|| {
        Error::NotSupported("no window global on WASM")
    })?;

    let query_fn = Reflect::get(&window, &JsValue::from_str("queryLocalFonts"))
        .map_err(|_| Error::Web("queryLocalFonts not available in this browser".into()))?;

    if query_fn.is_undefined() || query_fn.is_null() {
        return Err(Error::Web("queryLocalFonts not available in this browser".into()));
    }

    let promise = Reflect::apply(
        &query_fn.dyn_into::<Function>().map_err(|_| {
            Error::Web("queryLocalFonts is not a function".into())
        })?,
        &window,
        &js_sys::Array::new(),
    )
    .map_err(|e| Error::Web(format!("queryLocalFonts() threw: {e:?}")))?;

    let fonts_array = JsFuture::from(
        promise
            .dyn_into::<Promise>()
            .map_err(|_| Error::Web("queryLocalFonts did not return a Promise".into()))?,
    )
    .await
    .map_err(|e| Error::Web(format!("queryLocalFonts() rejected: {e:?}")))?;

    let fonts = js_sys::Array::from(&fonts_array);

    for i in 0..fonts.length() {
        let font = fonts.get(i);

        let family = Reflect::get(&font, &JsValue::from_str("family"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();

        let blob_method = Reflect::get(&font, &JsValue::from_str("blob"))
            .map_err(|_| Error::Web("FontData has no blob() method".into()))?
            .dyn_into::<Function>()
            .map_err(|_| Error::Web("blob is not a function".into()))?;

        let blob_promise = blob_method
            .call0(&font)
            .map_err(|e| Error::Web(format!("blob() threw: {e:?}")))?;

        let blob = JsFuture::from(
            blob_promise
                .dyn_into::<Promise>()
                .map_err(|_| Error::Web("blob() did not return a Promise".into()))?,
        )
        .await
        .map_err(|e| Error::Web(format!("blob() rejected: {e:?}")))?;

        let ab_method = Reflect::get(&blob, &JsValue::from_str("arrayBuffer"))
            .map_err(|_| Error::Web("Blob has no arrayBuffer() method".into()))?
            .dyn_into::<Function>()
            .map_err(|_| Error::Web("arrayBuffer is not a function".into()))?;

        let ab_promise = ab_method
            .call0(&blob)
            .map_err(|e| Error::Web(format!("arrayBuffer() threw: {e:?}")))?;

        let array_buffer = JsFuture::from(
            ab_promise
                .dyn_into::<Promise>()
                .map_err(|_| Error::Web("arrayBuffer() did not return a Promise".into()))?,
        )
        .await
        .map_err(|e| Error::Web(format!("arrayBuffer() rejected: {e:?}")))?;

        let uint8 = Uint8Array::new(&array_buffer);
        let data: Vec<u8> = uint8.to_vec();

        let info = FontInfoOverride {
            family_name: if family.is_empty() { None } else { Some(family.as_str()) },
            width: None,
            style: None,
            weight: None,
            axes: None,
        };

        let blob: Blob<u8> = data.into();
        collection.register_fonts(blob, Some(info));
    }

    Ok(())
}
