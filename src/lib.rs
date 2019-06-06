#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::ImageData;

#[macro_use]
mod utils;
mod scenes;
mod state;
mod ui;

fn parse_worker_message(
    evt: web_sys::MessageEvent,
) -> Result<(usize, js_sys::Uint32Array), JsValue> {
    let obj = evt.data();
    let id = js_sys::Reflect::get(&obj, &"id".into())?
        .as_f64()
        .expect("should be a number") as usize;
    let buffer = js_sys::Reflect::get(&obj, &"buffer".into())?;
    let buff = js_sys::ArrayBuffer::from(buffer);
    let uarr = js_sys::Uint32Array::new_with_byte_offset(&buff.dyn_into::<JsValue>()?, 0);
    Ok((id, uarr))
}

fn on_message(evt: web_sys::MessageEvent) -> Result<(), JsValue> {
    let (id, uarr) = parse_worker_message(evt)?;

    state::with(|state| state.handle_render(id, uarr))?;

    Ok(())
}

fn make_worker() -> Result<web_sys::Worker, JsValue> {
    let worker = web_sys::Worker::new("../worker/dist/bundle.js")?;
    let f = Closure::wrap(
        Box::new(|evt: web_sys::MessageEvent| utils::try_log(|| on_message(evt)))
            as Box<FnMut(web_sys::MessageEvent)>,
    );
    worker.set_onmessage(Some(f.as_ref().unchecked_ref()));
    f.forget();

    Ok(worker)
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let config = scenes::apple();

    state::setState(config.into());

    state::try_with(|state| {
        state.workers.push(make_worker()?);
        log!("Initial render!");
        state.async_render(false);
        Ok(())
    });

    Ok(())
}
