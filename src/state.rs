use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    pub type TimeoutId;

    #[wasm_bindgen(js_name = "setTimeout")]
    pub fn set_timeout_inner(cb: &JsValue, timeout: f64) -> TimeoutId;

    #[wasm_bindgen(js_name = "clearTimeout")]
    pub fn clear_timeout(id: &TimeoutId);
}

pub fn set_timeout<T: FnOnce() + 'static>(cb: T, timeout: f64) -> TimeoutId {
    set_timeout_inner(&Closure::once_into_js(cb), timeout)
}

pub struct State {
    pub hide_timeout: Option<TimeoutId>,
    pub render_id: usize,
    pub last_rendered: usize,
    pub ctx: CanvasRenderingContext2d,
    pub image_data: web_sys::ImageData,
    pub config: shared::Config,
    pub history: Vec<shared::Config>,
    pub history_index: usize,
    pub last_rendered_config: Option<shared::Config>,
    pub buffer: Vec<u32>,
    pub ui: crate::ui::UiState,
    pub hist_canvas: Option<web_sys::HtmlCanvasElement>,
    pub on_change: js_sys::Function,
    pub workers: Vec<(web_sys::Worker, bool, Option<shared::messaging::Message>)>,
}

// umm I dunno if this is cheating or something
// I mean bad things could happen if I accessed the ctx
// from different threads
// but given that wasm doesn't yet have threads, it's probably fine.
unsafe impl Send for State {}

impl State {
    pub fn new(config: shared::Config, on_change: js_sys::Function) -> Self {
        State {
            hide_timeout: None,
            render_id: 0,
            hist_canvas: None,
            last_rendered: 0,
            ctx: crate::ui::init(&config).expect("Unable to setup canvas"),
            image_data: web_sys::ImageData::new_with_sw(
                config.rendering.width as u32,
                config.rendering.height as u32,
            )
            .expect("Can't make an imagedata"),
            buffer: vec![0_u32; config.rendering.width * config.rendering.height],
            workers: vec![],
            ui: Default::default(),
            history: vec![config.clone()],
            history_index: 0,
            last_rendered_config: None,
            on_change,
            config,
        }
    }
}

pub fn make_image_data(
    config: &shared::Config,
    bright: &[u32],
) -> Result<web_sys::ImageData, JsValue> {
    let colored = shared::colorize(config, bright);

    let mut clamped = wasm_bindgen::Clamped(colored.clone());
    // let mut clamped = Clamped(state.buffer.clone());
    let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(clamped.as_mut_slice()),
        config.rendering.width as u32,
        config.rendering.height as u32,
    )?;

    Ok(data)
}

impl State {
    pub fn reset_buffer(&mut self) {
        self.buffer = vec![0_u32; self.config.rendering.width * self.config.rendering.height];
        self.invalidate_past_renders();
    }

    pub fn add_worker(&mut self, worker: web_sys::Worker) {
        self.workers.push((worker, false, None))
    }

    pub fn invalidate_past_renders(&mut self) {
        self.render_id += 1;
        self.last_rendered = self.render_id;
    }

    pub fn undo(&mut self) -> Result<(), JsValue> {
        log!("Undo {} {}", self.history.len(), self.history_index);
        if self.history_index == 0 {
            if Some(&self.config) != self.history.last() {
                self.history.push(self.config.clone());
            }
        }
        self.history_index = (self.history_index + 1).min(self.history.len() - 1);
        if let Some(config) = self
            .history
            .get(self.history.len() - self.history_index - 1)
        {
            self.config = config.clone();
            self.async_render(false)?;
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), JsValue> {
        if self.history_index == 0 {
            log!("nothing to redo");
            return Ok(());
        }
        log!("redo");
        self.history_index = (self.history_index - 1).max(0);
        if let Some(config) = self
            .history
            .get(self.history.len() - self.history_index - 1)
        {
            self.config = config.clone();
            self.async_render(false)?;
        }
        Ok(())
    }

    pub fn maybe_save_history(&mut self) {
        log!("saving history");
        // If the lastest is the same
        if self.history_index == 0
            && self
                .history
                .last()
                .map_or(false, |last| *last == self.config)
        {
            return;
        }
        if self.history_index != 0
            && self
                .history
                .get(self.history.len() - self.history_index - 1)
                .map_or(false, |last| *last == self.config)
        {
            return;
        }

        // snip undone stuff
        if self.history_index != 0 {
            self.history = self.history[0..self.history.len() - self.history_index].to_vec();
            self.history_index = 0;
        }

        // if self.history.last().map_or(true, |last| *last != self.config) {
        self.history.push(self.config.clone());
        if self.history.len() > 500 {
            // trim to 500 len
            self.history = self.history[self.history.len() - 500..].to_vec();
        }
        // }
    }

    pub fn handle_render(
        &mut self,
        worker: usize,
        id: usize,
        array: js_sys::Uint32Array,
    ) -> Result<(), JsValue> {
        if id < self.last_rendered {
            let (worker, busy, queued) = &mut self.workers[worker];
            match queued {
                None => {
                    // log!("Finished a thread");
                    *busy = false
                }
                Some(message) => {
                    // log!("Sending a new config to render");
                    worker.post_message(&JsValue::from_serde(message).unwrap())?;
                    *queued = None
                }
            }
            // this is old data, disregard
            return Ok(());
        }
        if id > self.last_rendered {
            self.reset_buffer();
            self.last_rendered = id;
        }

        let mut bright = vec![0_u32; self.config.rendering.width * self.config.rendering.height];
        array.copy_to(&mut bright);
        for i in 0..bright.len() {
            self.buffer[i] += bright[i];
        }

        self.image_data = make_image_data(&self.config, &self.buffer)?;

        // crate::ui::use_ui(|ui| {
        //     crate::ui::draw(ui, &self)
        // });
        // self.ctx.put_image_data(&self.image_data, 0.0, 0.0)?;

        let (worker, busy, queued) = &mut self.workers[worker];
        match queued {
            None => {
                // log!("Finished a thread");
                *busy = false
            }
            Some(message) => {
                // log!("Sending a new config to render");
                worker.post_message(&JsValue::from_serde(message).unwrap())?;
                *queued = None
            }
        }

        Ok(())
    }

    pub fn debug_render(&mut self) -> Result<(), JsValue> {
        let brightness = shared::calculate::deterministic_calc(&self.config);
        self.image_data = make_image_data(&self.config, &brightness)?;

        self.ctx.put_image_data(&self.image_data, 0.0, 0.0)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.config.rendering.width as f64,
            self.config.rendering.height as f64,
        )
    }

    pub fn reexpose(&mut self) -> Result<(), JsValue> {
        self.image_data = make_image_data(&self.config, &self.buffer)?;

        // self.ctx.put_image_data(&self.image_data, 0.0, 0.0)?;
        // crate::ui::use_ui(|ui| {
        crate::ui::draw(&self);
        // });

        Ok(())
    }

    pub fn send_on_change(&self) {
        let _res = self.on_change.call2(
            &JsValue::null(),
            &JsValue::from_serde(&self.config).unwrap(),
            &JsValue::from_serde(&self.ui).unwrap(),
        );
    }

    pub fn async_render(&mut self, small: bool) -> Result<(), JsValue> {
        // log!("Async nreder folks");
        match &self.last_rendered_config {
            Some(config) => {
                if *config == self.config {
                    return Ok(());
                }
                let mut old_config_with_new_exposure = config.clone();
                old_config_with_new_exposure.rendering.exposure =
                    self.config.rendering.exposure.clone();
                old_config_with_new_exposure.rendering.coloration =
                    self.config.rendering.coloration.clone();
                // We've only changed settings that don't require recalculation
                if old_config_with_new_exposure == self.config {
                    self.last_rendered_config = Some(self.config.clone());
                    self.send_on_change();
                    self.reexpose();
                    return Ok(());
                } else {
                    log!("Not the same")
                    // log!("Not the same! {} vs {}", old_json, json)
                }
            }
            _ => (),
        }

        // log!("Render new config");
        // web_sys::console::log_1(&JsValue::from_serde(&self.config).unwrap());
        self.send_on_change();

        self.last_rendered_config = Some(self.config.clone());
        self.render_id += 1;

        let message = shared::messaging::Message {
            config: self.config.clone(),
            id: self.render_id,
            // count: if small { 10_000 } else { 500_000 },
            count: 200_000,
        };
        if self.workers.is_empty() {
            return self.debug_render();
        }

        for (worker, busy, queued) in self.workers.iter_mut() {
            if *busy {
                // log!("Queueing up for a worker");
                *queued = Some(message.clone())
            } else {
                *busy = true;
                // log!("Sending a new config to render");
                worker.post_message(&JsValue::from_serde(&message).unwrap())?;
            }
        }
        Ok(())
    }
}

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

// pub fn withOptState<F: FnOnce(&mut Option<State>)>(f: F) {
//     f(&mut STATE.lock().unwrap())
// }

// pub fn setState(state: State) {
//     withOptState(|wrapper| *wrapper = Some(state))
// }

pub fn has_state() -> bool {
    match STATE.lock().unwrap().as_mut() {
        Some(_) => true,
        None => false,
    }
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

pub fn maybe_with<F: FnOnce(&mut State)>(f: F) {
    match STATE.lock().unwrap().as_mut() {
        Some(mut state) => f(&mut state),
        None => (),
    }
}

pub fn try_with<F: FnOnce(&mut State) -> Result<(), wasm_bindgen::prelude::JsValue>>(f: F) {
    with(|state| crate::utils::try_log(|| f(state)))
}
