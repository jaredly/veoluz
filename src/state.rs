use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub struct State {
    pub config: shared::Config,
    pub buffer: Vec<u32>,
}

impl From<shared::Config> for State {
    fn from(config: shared::Config) -> Self {
        State {
            buffer: vec![0_u32; config.width * config.height],
            config,
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

pub fn withOptState<F: FnOnce(&mut Option<State>)>(f: F) {
    f(&mut STATE.lock().unwrap())
}

pub fn setState(state: State) {
    withOptState(|wrapper| *wrapper = Some(state))
}

pub fn with<R, F: FnOnce(&mut State) -> R>(f: F) -> R {
    match STATE.lock().unwrap().as_mut() {
        Some(mut state) => f(&mut state),
        None => {
            log!("!!! Error: tried to handle state, but no state found");
            panic!("No state found, must set state first")
        }
    }
}

pub fn try_with<F: FnOnce(&mut State) -> Result<(), wasm_bindgen::prelude::JsValue>>(f: F) {
    with(|state| crate::utils::try_log(|| f(state)))
}
