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

        let colored = shared::colorize(&state.config, bright);
        log!("colored array with length {}", colored.len());

        // let uarr = js_sys::Uint8ClampedArray::from(uarr.dyn_into::<JsValue>()?);
        // log!("8 array with length {}", uarr.length());

        // uarr.copy_to(&mut state.buffer);
        // uarr.copy_to(&mut state.buffer);

        let mut clamped = Clamped(colored.clone());
        // let mut clamped = Clamped(state.buffer.clone());
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(clamped.as_mut_slice()),
            state.config.width as u32,
            state.config.height as u32,
        )?;

        let ctx = ui::ctx()?;
        ctx.put_image_data(&data, 0.0, 0.0)?;
        ctx.set_stroke_style(&JsValue::from_str("green"));

        for wall in state.config.walls.iter() {
            wall.kind.draw(&ctx)
        }
        Ok(())
    })?;

    Ok(())
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let config = scenes::apple();

    ui::init(&config)?;
    let ctx = ui::ctx()?;

    // const canvas = document.getElementById('drawing');
    // const ctx = canvas.getContext('2d');
    // canvas.width = 1024
    // canvas.height = 576

    state::setState(config.into());

    // let mut data = shared::zen_photon(&config);
    // let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    // ctx.put_image_data(&data, 0.0, 0.0)?;

    let worker = web_sys::Worker::new("../worker/dist/bundle.js")?;
    let f = Closure::wrap(
        Box::new(|evt: web_sys::MessageEvent| utils::try_log(|| on_message(evt)))
            as Box<FnMut(web_sys::MessageEvent)>,
    );
    worker.set_onmessage(Some(f.as_ref().unchecked_ref()));
    f.forget();

    state::try_with(|state| {
        log!("Sending a message to the worker");
        worker.post_message(&JsValue::from_serde(&state.config).unwrap())?;
        ctx.set_stroke_style(&JsValue::from_str("green"));
        for wall in state.config.walls.iter() {
            wall.kind.draw(&ctx);
        }
        Ok(())
    });

    // Ok(f.as_ref().unchecked_ref())
    Ok(())
}
