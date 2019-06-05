#[macro_use]
extern crate lazy_static;

use nalgebra::{Point2, Vector2};
use ncollide2d::query::Ray;
use ncollide2d::shape::Ball;
use shared;
use shared::line;
use shared::{Wall, WallType};
use wasm_bindgen::prelude::*;

use std::f32::consts::PI;

// use rand::random;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageBitmap, ImageData};

use nalgebra as na;
use nalgebra::geometry::{Isometry2, Rotation2, Translation2};

use wasm_bindgen::JsCast;

#[macro_use]
mod utils;
mod state;
mod scenes;

fn on_message(evt: web_sys::MessageEvent) -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let canvas = document.get_element_by_id("drawing").expect("get Canvas");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let uarr = js_sys::Uint8ClampedArray::from(evt.data());

    state::with(|state| -> Result<(), JsValue> {
        uarr.copy_to(&mut state.buffer);

        let mut clamped = Clamped(state.buffer.clone());
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(clamped.as_mut_slice()),
            state.config.width as u32,
            state.config.height as u32,
        )?;

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
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    _real: f64,
    _imaginary: f64,
) -> Result<(), JsValue> {
    let config = scenes::apple();

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
