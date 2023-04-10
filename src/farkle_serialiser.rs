use std::{
    fs::File,
    io::{BufReader, BufWriter, Error},
};

use crate::{
    farkle_solver::{DecideActionCache, FarkleSolver},
    farkle_solver_wasm::FarkleSolverWasm,
    utils::console_log,
};
use itertools::Itertools;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn populate_solver(
    url: String,
    solver: &mut FarkleSolverWasm,
) -> Result<JsValue, JsValue> {
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
    let data_arr = js_sys::Uint8Array::new(&array_buffer).to_vec();
    console_log!("data_arr len {}", data_arr.len());

    console_log!("deserialising...");
    let cache: DecideActionCache<2> = bincode::deserialize(&data_arr).unwrap();
    console_log!("is approx: {}", solver.get_is_approx());

    console_log!("set cache");
    solver.set_cache(&cache);

    console_log!("cache size {}", cache.len());
    Ok(JsValue::default())
}

pub fn populate_solver_from_file(solver: &mut FarkleSolver, file: String) -> Result<(), Error> {
    let f = File::open(file)?;
    let buf_reader = BufReader::new(f);
    let cache: DecideActionCache<2> = bincode::deserialize_from(buf_reader).unwrap();
    solver.set_cache(&cache);
    for k in solver.get_mutable_data().cache_decide_action.keys() {
        println!("{k} {:?}", solver.unpack_cache_key(*k));
    }
    Ok(())
}

pub fn write_solver(solver: &FarkleSolver<2>, path: &str) {
    println!("writing cache to file {path}");
    let mut f = BufWriter::new(File::create(path).unwrap());
    let mut cache = solver.get_mutable_data().cache_decide_action.clone();
    let keys = solver
        .get_mutable_data()
        .cache_decide_action
        .keys()
        .clone()
        .collect_vec();
    for key in &keys {
        let (held_score, _, _) = solver.unpack_cache_key(**key);
        if held_score != 0 {
            cache.remove(key);
        }
    }
    println!(
        "writing {} keys, {}%...",
        cache.len(),
        100.0 * (cache.len() as f64) / (solver.get_mutable_data().cache_decide_action.len() as f64)
    );
    bincode::serialize_into(&mut f, &cache).unwrap();
}
