use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

fn on_message(f: &Closure<FnMut(web_sys::MessageEvent)>) {
    let glob: JsValue = js_sys::global().into();
    let scope: web_sys::DedicatedWorkerGlobalScope = glob.into();
    scope.set_onmessage(Some(f.as_ref().unchecked_ref()))
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
pub fn run() -> Result<IntervalHandle, JsValue> {
    set_panic_hook();

    let cb = Closure::wrap(Box::new(move |evt: web_sys::MessageEvent| {
        log!("Got a message!");
        log!("Data: {}", evt.data().as_string().expect("Expected data to be a string"))
    }) as Box<FnMut(web_sys::MessageEvent)>);

    on_message(&cb);

    // let window = web_sys::window().expect("should have a Window");
    // let document = window.document().expect("should have a Document");

    // let p: web_sys::Node = document.create_element("p")?.into();
    // p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));

    // let body = document.body().expect("should have a body");
    // let body: &web_sys::Node = body.as_ref();
    // body.append_child(&p)?;

    Ok(IntervalHandle { _closure: cb })
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
