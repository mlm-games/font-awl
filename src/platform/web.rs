use fontique::{Blob, Collection, FontInfoOverride};
use js_sys::{Function, Promise, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::Error;

async fn call_method_and_await(
    obj: &JsValue,
    method_name: &str,
) -> Result<JsValue, Error> {
    let method = Reflect::get(obj, &JsValue::from_str(method_name))
        .map_err(|_| Error::Web(format!("{method_name} not available")))?;
    let promise = method
        .dyn_into::<Function>()
        .map_err(|_| Error::Web(format!("{method_name} is not a function")))?
        .call0(obj)
        .map_err(|e| Error::Web(format!("{method_name}() threw: {e:?}")))?;
    JsFuture::from(
        promise
            .dyn_into::<Promise>()
            .map_err(|_| Error::Web(format!("{method_name} did not return a Promise")))?,
    )
    .await
    .map_err(|e| Error::Web(format!("{method_name}() rejected: {e:?}")))
}

pub(crate) async fn load_local_fonts(collection: &mut Collection) -> Result<Vec<Blob<u8>>, Error> {
    let mut pending: Vec<(Blob<u8>, String)> = Vec::new();
    let window = web_sys::window()
        .ok_or_else(|| Error::NotSupported("no window global on WASM"))?;

    let query_fn = Reflect::get(&window, &JsValue::from_str("queryLocalFonts"))
        .map_err(|_| Error::Web("queryLocalFonts not supported in this browser".into()))?;

    if query_fn.is_undefined() || query_fn.is_null() {
        return Err(Error::Web("queryLocalFonts not supported in this browser".into()));
    }

    let fonts_array = call_method_and_await(&window, "queryLocalFonts").await?;
    let fonts = js_sys::Array::from(&fonts_array);

    for i in 0..fonts.length() {
        let font = fonts.get(i);

        let family = Reflect::get(&font, &JsValue::from_str("family"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();

        let blob_value = call_method_and_await(&font, "blob").await?;

        let array_buffer = call_method_and_await(&blob_value, "arrayBuffer").await?;

        let uint8 = Uint8Array::new(&array_buffer);
        pending.push((uint8.to_vec().into(), family));
    }

    let mut font_data = Vec::with_capacity(pending.len());
    for (blob, family) in &pending {
        let info = FontInfoOverride {
            family_name: if family.is_empty() { None } else { Some(family.as_str()) },
            width: None,
            style: None,
            weight: None,
            axes: None,
        };

        collection.register_fonts(blob.clone(), Some(info));
        font_data.push(blob.clone());
    }

    Ok(font_data)
}
