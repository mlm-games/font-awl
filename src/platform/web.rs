use fontique::{Blob, Collection, FontInfoOverride};
use js_sys::{Function, Promise, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::Error;

async fn call_method_and_await(obj: &JsValue, method_name: &str) -> Result<JsValue, Error> {
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

/// Whether the document's Permissions Policy grants `local-fonts`.
///
/// Embedders that do not pass `allow="local-fonts"` (any cross-origin frame)
/// make the browser log a Permissions Policy violation the moment
/// `queryLocalFonts` is touched, so the capability is checked before the call.
/// Browsers without `document.permissionsPolicy` report `true` and let the
/// call itself decide.
fn local_fonts_permitted() -> bool {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return false;
    };
    let Ok(policy) = Reflect::get(&document, &JsValue::from_str("permissionsPolicy")) else {
        return true;
    };
    let Ok(allows_feature) = Reflect::get(&policy, &JsValue::from_str("allowsFeature")) else {
        return true;
    };
    let Ok(function) = allows_feature.dyn_into::<Function>() else {
        return true;
    };
    function
        .call1(&policy, &JsValue::from_str("local-fonts"))
        .ok()
        .and_then(|allowed| allowed.as_bool())
        .unwrap_or(true)
}

/// `queryLocalFonts()` reports every font installed on the machine, which on a
/// desktop is far more data than a wasm module's linear memory. Each blob is
/// copied into that memory, so ingestion is bounded: fonts larger than
/// [`MAX_FONT_BYTES`] are skipped, and the walk stops once the registered total
/// reaches [`MAX_TOTAL_BYTES`]. Without those bounds a full font library
/// exhausts the module and aborts the instance.
const MAX_FONT_BYTES: usize = 16 * 1024 * 1024;
const MAX_TOTAL_BYTES: usize = 64 * 1024 * 1024;

pub(crate) async fn load_local_fonts(collection: &mut Collection) -> Result<Vec<Blob<u8>>, Error> {
    let window =
        web_sys::window().ok_or_else(|| Error::NotSupported("no window global on WASM"))?;

    if !local_fonts_permitted() {
        return Err(Error::NotSupported(
            "local-fonts not granted by Permissions Policy",
        ));
    }

    let query_fn = Reflect::get(&window, &JsValue::from_str("queryLocalFonts"))
        .map_err(|_| Error::Web("queryLocalFonts not supported in this browser".into()))?;

    if query_fn.is_undefined() || query_fn.is_null() {
        return Err(Error::Web(
            "queryLocalFonts not supported in this browser".into(),
        ));
    }

    let fonts_array = call_method_and_await(&window, "queryLocalFonts").await?;
    let fonts = js_sys::Array::from(&fonts_array);

    let mut font_data: Vec<Blob<u8>> = Vec::new();
    let mut registered_bytes = 0usize;
    let mut skipped_fonts = 0usize;
    let mut skipped_bytes = 0usize;

    for i in 0..fonts.length() {
        if registered_bytes >= MAX_TOTAL_BYTES {
            skipped_fonts += fonts.length() as usize - i as usize;
            break;
        }

        let font = fonts.get(i);

        let family = Reflect::get(&font, &JsValue::from_str("family"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();

        let blob_value = call_method_and_await(&font, "blob").await?;

        let array_buffer = call_method_and_await(&blob_value, "arrayBuffer").await?;

        let uint8 = Uint8Array::new(&array_buffer);
        let size = uint8.length() as usize;
        if size > MAX_FONT_BYTES || registered_bytes + size > MAX_TOTAL_BYTES {
            skipped_fonts += 1;
            skipped_bytes += size;
            continue;
        }

        let blob: Blob<u8> = uint8.to_vec().into();
        registered_bytes += size;

        let info = FontInfoOverride {
            family_name: if family.is_empty() {
                None
            } else {
                Some(family.as_str())
            },
            width: None,
            style: None,
            weight: None,
            axes: None,
        };

        collection.register_fonts(blob.clone(), Some(info));
        font_data.push(blob);
    }

    log::info!(
        "local-fonts: registered {} fonts ({registered_bytes} bytes), skipped {skipped_fonts} fonts ({skipped_bytes} bytes)",
        font_data.len()
    );

    Ok(font_data)
}
