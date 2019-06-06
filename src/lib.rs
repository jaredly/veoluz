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

fn on_message(evt: web_sys::MessageEvent) -> Result<(), JsValue> {
    let buff = js_sys::ArrayBuffer::from(evt.data());
    let uarr = js_sys::Uint32Array::new_with_byte_offset(&buff.dyn_into::<JsValue>()?, 0);

    state::with(|state| -> Result<(), JsValue> {
        let mut bright = vec![0_u32;state.config.width * state.config.height];
        uarr.copy_to(&mut bright);
        for i in 0..bright.len() {
            state.buffer[i] += bright[i];
        }

        let colored = shared::colorize(&state.config, &state.buffer);

        let mut clamped = Clamped(colored.clone());
        // let mut clamped = Clamped(state.buffer.clone());
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(clamped.as_mut_slice()),
            state.config.width as u32,
            state.config.height as u32,
        )?;
        state.image_data = data;

        state.ctx.put_image_data(&state.image_data, 0.0, 0.0)?;
        state.ctx.set_stroke_style(&JsValue::from_str("green"));

        for wall in state.config.walls.iter() {
            wall.kind.draw(&state.ctx)
        }
        Ok(())
    })?;

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

    let worker = make_worker()?;

    state::setState(config.into());

    state::try_with(|state| {
        log!("Sending a message to the worker");
        // TODO here I need to clean out the state.buffer probably
        worker.post_message(&JsValue::from_serde(&state.config).unwrap())?;
        state.ctx.set_stroke_style(&JsValue::from_str("green"));
        for wall in state.config.walls.iter() {
            wall.kind.draw(&state.ctx);
        }
        Ok(())
    });

    Ok(())
}
