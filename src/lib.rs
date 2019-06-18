#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate yew;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;

#[macro_use]
mod utils;
mod draw;
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

fn on_message(wid: usize, evt: web_sys::MessageEvent) -> Result<(), JsValue> {
    let (id, uarr) = parse_worker_message(evt)?;

    state::with(|state| {
        state.handle_render(wid, id, uarr)?;
        ui::use_ui(|ui| ui::draw(ui, state))
    })?;

    Ok(())
}

fn make_worker(wid: usize) -> Result<web_sys::Worker, JsValue> {
    let worker = web_sys::Worker::new("./worker.js")?;
    let f = Closure::wrap(Box::new(move |evt: web_sys::MessageEvent| {
        utils::try_log(|| on_message(wid, evt))
    }) as Box<FnMut(web_sys::MessageEvent)>);
    worker.set_onmessage(Some(f.as_ref().unchecked_ref()));
    f.forget();

    Ok(worker)
}

#[wasm_bindgen]
pub fn save() -> JsValue {
    state::with(|state| JsValue::from_serde(&state.config).unwrap())
}

pub fn deserialize_jsvalue(encoded: &JsValue) -> Result<shared::Config, serde_json::Error> {
    encoded
        .into_serde::<shared::Config>()
        .or_else(|_| {
            encoded
                .into_serde::<shared::v3::Config>()
                .map(shared::from_v3)
        })
        .or_else(|_| {
            encoded
                .into_serde::<shared::v2::Config>()
                .map(shared::v3::from_v2)
                .map(shared::from_v3)
        })
        .or_else(|_| {
            encoded
                .into_serde::<shared::v1::Config>()
                .map(shared::v2::from_v1)
                .map(shared::v3::from_v2)
                .map(shared::from_v3)
        })
}

#[wasm_bindgen]
pub fn restore(config: &JsValue) {
    ui::try_state_ui(|state, ui| {
        if let Ok(config) = deserialize_jsvalue(config) {
            state.invalidate_past_renders();
            ui::reset(&config, ui)?;
            let size_changed = config.rendering.width != state.config.rendering.width
                || config.rendering.height != state.config.rendering.height;
            state.config = config;
            if size_changed {
                state
                    .ctx
                    .canvas()
                    .unwrap()
                    .set_width(state.config.rendering.width as u32);
                state
                    .ctx
                    .canvas()
                    .unwrap()
                    .set_height(state.config.rendering.height as u32);
                state.reset_buffer();
            }
            state.clear();
            state.async_render(false)
        } else {
            Ok(())
        }
    })
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    // let config = scenes::circle_row();
    // let config = scenes::refract2();
    let width = 1024;
    let height = 576;
    let config = shared::Config::new(vec![], width, height);
    // let config = scenes::playground();
    // let config = scenes::parabola_test();
    // let config = scenes::apple();
    // let config = scenes::refraction_test();

    let config = match ui::get_url_config() {
        None => config,
        Some(config) => config,
    };

    state::setState(config.into());

    state::try_with(|state| {
        state.add_worker(make_worker(0)?);
        state.async_render(false)?;

        // state.debug_render()?;

        Ok(())
    });

    Ok(())
}
