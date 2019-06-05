use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, ImageBitmap};

#[wasm_bindgen]
extern "C" {
    type DedicatedWorkerGlobalScope;
    #[wasm_bindgen(js_namespace = global)]
    pub fn createImageBitmap(data: &JsValue) -> js_sys::Promise;
}

fn global() -> web_sys::DedicatedWorkerGlobalScope {
    let glob: JsValue = js_sys::global().into();
    // let scope: web_sys::DedicatedWorkerGlobalScope =
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
// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn process(config: JsValue) -> Result<Clamped<Vec<u8>>, JsValue> {
    set_panic_hook();

            let config: shared::Config = config.into_serde().expect("Invalid data");
            let mut data = shared::zen_photon(&config);
            log!("Creating a bitmap {}x{}", config.width, config.height);


    Ok(Clamped(data))
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
