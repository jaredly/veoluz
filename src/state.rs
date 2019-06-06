use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub struct State {
    pub render_id: usize,
    pub ctx: CanvasRenderingContext2d,
    pub image_data: web_sys::ImageData,
    pub config: shared::Config,
    pub buffer: Vec<u32>,
    pub workers: Vec<web_sys::Worker>,
}

// umm I dunno if this is cheating or something
// I mean bad things could happen if I accessed the ctx
// from different threads
// but given that wasm doesn't yet have threads, it's probably fine.
unsafe impl Send for State {}

impl From<shared::Config> for State {
    fn from(config: shared::Config) -> Self {
        State {
            render_id: 0,
            ctx: crate::ui::init(&config).expect("Unable to setup canvas"),
            image_data: web_sys::ImageData::new_with_sw(config.width as u32, config.height as u32).expect("Can't make an imagedata"),
            buffer: vec![0_u32; config.width * config.height],
            workers: vec![],
            config,
        }
    }
}
impl State {
    pub fn reset_buffer(&mut self) {
        self.buffer = vec![0_u32; self.config.width * self.config.height];
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
