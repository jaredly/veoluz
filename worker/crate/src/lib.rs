use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageBitmap, ImageData};

fn global() -> web_sys::DedicatedWorkerGlobalScope {
    let glob: JsValue = js_sys::global().into();
    glob.into()
}

fn on_message(f: &Closure<FnMut(web_sys::MessageEvent)>) {
    global().set_onmessage(Some(f.as_ref().unchecked_ref()))
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct IntervalHandle {
    _closure: Closure<FnMut(web_sys::MessageEvent)>,
}

fn to_le(v: &mut [u32]) -> &[u8] {
    for b in v.iter_mut() {
        *b = b.to_le()
    }
    unsafe { v.align_to().1 }
}

#[wasm_bindgen]
pub struct Response {
    data: Clamped<Vec<u8>>,
    rays: usize,
}

#[wasm_bindgen]
impl Response {
    pub fn rays(&self) -> usize {
        self.rays
    }
    pub fn data(self) -> Clamped<Vec<u8>> {
        self.data
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn process(message: JsValue) -> Result<Response, JsValue> {
    set_panic_hook();

    let message: shared::messaging::Message = message.into_serde().expect("Invalid message");
    let (mut data, rays) = shared::calculate::timed(&message.config, message.count, 100.0);
    // log!("Creating a bitmap {}x{}, bright size {}", config.width, config.height, data.len());

    Ok(Response {
        data: Clamped(to_le(&mut data).to_vec()),
        rays,
    })
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
