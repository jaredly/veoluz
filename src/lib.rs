#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate yew;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;

#[macro_use]
mod utils;
mod draw;
mod old_ui;
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

// #{wasm_bindgen}
// pub fn

#[wasm_bindgen]
pub fn set_active_wall(idx: usize) {
    let _ = ui::state_ui(|state, ui| {
        ui.selection = Some(ui::Selection::Wall(idx, None));
        ui::draw(ui, state)
    });
}

#[wasm_bindgen]
pub fn hover_wall(idx: usize) {
    let _ = ui::state_ui(|state, ui| {
        ui.hovered = Some((idx, ui::Handle::Move(nalgebra::zero())));
        ui::draw(ui, state)
    });
}

#[wasm_bindgen]
pub fn show_ui() {
    let _ = ui::state_ui(|state, ui| {
        ui.mouse_over = true;
        ui::draw(ui, state)
    });
}

#[wasm_bindgen]
pub fn hide_ui() {
    let _ = ui::state_ui(|state, ui| {
        ui.mouse_over = false;
        ui::draw(ui, state)
    });
}

#[wasm_bindgen]
pub fn show_hist() {
    let _ = ui::state_ui(|state, ui| {
        ui.show_hist = true;
        ui::draw(ui, state)
    });
}

#[wasm_bindgen]
pub fn hide_hist() {
    let _ = ui::state_ui(|state, ui| {
        ui.show_hist = false;
        ui::draw(ui, state)
    });
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

fn update_config(config: shared::Config, reset: bool, checkpoint: bool) {
    ui::try_state_ui(|state, ui| {
        if reset {
            state.invalidate_past_renders();
            ui::reset(&config, ui)?;
        }
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
        if reset {
            state.clear();
        }
        if reset || checkpoint {
            state.maybe_save_history();
        }
        state.async_render(false)
    })
}

#[wasm_bindgen]
pub fn update(config: &JsValue, checkpoint: bool) {
    if let Ok(config) = deserialize_jsvalue(config) {
        update_config(config, false, checkpoint)
    } else {
        println!("Bad config")
    }
}

#[wasm_bindgen]
pub fn restore(config: &JsValue) {
    if let Ok(config) = deserialize_jsvalue(config) {
        update_config(config, true, true)
    } else {
        println!("Bad config")
    }
}

#[wasm_bindgen]
pub fn parse_url_config(hash: &str) -> JsValue {
    ui::parse_url_config(hash).map_or(JsValue::null(), |config| {
        JsValue::from_serde(&config).unwrap()
    })
}

#[wasm_bindgen]
pub fn serialize_url_config(config: &JsValue) -> String {
    config
        .into_serde::<shared::Config>()
        .map(|config| {
            let encoded = bincode::serialize(&config).unwrap();
            let zipped = miniz_oxide::deflate::compress_to_vec(&encoded, 10);
            log!("Sharing {} vs {}", encoded.len(), zipped.len());

            let b64 = base64::encode(&zipped);
            b64
        })
        .ok()
        .unwrap_or("".into())
}

#[wasm_bindgen]
pub fn blank_config() -> JsValue {
    let width = 1024;
    let height = 576;
    JsValue::from_serde(&shared::Config::new(vec![], width, height)).unwrap()
    // scenes::playground()
    // scenes::parabola_test()
    // scenes::apple()
    // scenes::refraction_test()
    // scenes::circle_row()
    // scenes::refract2()
}

pub fn initial_config() -> shared::Config {
    match ui::get_url_config() {
        None => scenes::apple(),
        Some(config) => config,
    }
}

#[wasm_bindgen]
pub fn initial() -> JsValue {
    JsValue::from_serde(&initial_config()).unwrap()
}

#[wasm_bindgen]
pub fn setup(config: &JsValue, on_change: &js_sys::Function) {
    if let Ok(config) = deserialize_jsvalue(config) {
        if state::has_state() {
            update_config(config, true, true);
            state::with(|state| {
                state.on_change = on_change.to_owned();
            })
        } else {
            state::setState(state::State::new(config, on_change.to_owned()));
            state::try_with(|state| {
                state.add_worker(make_worker(0)?);
                state.async_render(false)?;
                Ok(())
            });
        }
    } else {
        panic!("Invalid config provided")
    }
}

#[wasm_bindgen]
pub fn run(on_change: &js_sys::Function) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    state::setState(state::State::new(initial_config(), on_change.to_owned()));

    state::try_with(|state| {
        state.add_worker(make_worker(0)?);
        state.async_render(false)?;

        // state.debug_render()?;

        Ok(())
    });

    Ok(())
}
