use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::{farkle_solver::{FarkleSolver, MutableCache}, console_log, log};


#[wasm_bindgen]
pub async fn populate_solver(url: String, solver: &mut FarkleSolver) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    console_log!("url: {url}");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    console_log!("resp: {resp:?} OK? {}", resp.ok());
    assert!(resp.ok());

    let array_buffer = JsFuture::from(resp.array_buffer()?).await?;
    let data_arr = js_sys::Uint8Array
        ::new(&array_buffer)
        .to_vec();

    console_log!("data_arr len {}", data_arr.len());
    
    console_log!("deserialising...");
    let cache: MutableCache = bincode::deserialize(&data_arr).unwrap();

    console_log!("set cache");
    solver.set_cache(&cache);

    console_log!("cache size {}", cache.len());
    Ok(JsValue::default())
}