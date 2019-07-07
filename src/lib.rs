#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate yew;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;

#[macro_use]
mod utils;
mod draw;
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
        ui::draw(state)
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
    let _ = state::with(|state| {
        state.ui.selection = Some(ui::Selection::Wall(idx, None));
        ui::draw(state);
        state.send_on_change();
    });
}

#[wasm_bindgen]
pub fn set_active_light(idx: usize) {
    let _ = state::with(|state| {
        state.ui.selection = Some(ui::Selection::Light(idx, false));
        ui::draw(state);
        state.send_on_change();
    });
}

#[wasm_bindgen]
pub fn hover_wall(idx: usize) {
    state::maybe_with(|state| {
        state.ui.hovered = Some((idx, ui::Handle::Move(nalgebra::zero())));
        ui::draw(state);
        state.send_on_change();
    });
}

#[wasm_bindgen]
pub fn show_ui() {
    state::maybe_with(|state| {
        state.ui.mouse_over = true;
        ui::draw(state);
    });
}

#[wasm_bindgen]
pub fn hide_ui() {
    state::maybe_with(|state| {
        state.ui.mouse_over = false;
        ui::draw(state);
    });
}

#[wasm_bindgen]
pub fn show_hist(canvas: web_sys::HtmlCanvasElement) {
    state::maybe_with(|state| {
        state.hist_canvas = Some(canvas);
        ui::draw(state);
    });
}

#[wasm_bindgen]
pub fn hide_hist() {
    state::maybe_with(|state| {
        state.hist_canvas = None;
        ui::draw(state);
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
    state::try_with(|state| {
        if reset {
            state.invalidate_past_renders();
            ui::reset(&config, &mut state.ui)?;
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
        state.last_rendered_config = None;
        state.async_render(false)
    })
}

#[wasm_bindgen]
pub fn update_ui(ui: &JsValue) {
    if let Ok(ui) = ui.into_serde() {
        state::try_with(|state| {
            state.ui = ui;
            ui::draw(state)
        });
    } else {
        println!("Bad config")
    }
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
pub fn restore(config: &JsValue) -> JsValue {
    if let Ok(mut config) = deserialize_jsvalue(config) {
        config.rendering.width = 1024;
        config.rendering.height = 576;
        let js = JsValue::from_serde(&config).unwrap();
        update_config(config, true, true);
        js
    } else {
        println!("Bad config");
        JsValue::null()
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
pub fn undo() -> Result<JsValue, JsValue> {
    state::with(|state| {
        state.undo()?;
        Ok(JsValue::from_serde(&state.config).unwrap())
    })
}

#[wasm_bindgen]
pub fn redo() -> Result<JsValue, JsValue> {
    state::with(|state| {
        state.redo()?;
        Ok(JsValue::from_serde(&state.config).unwrap())
    })
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
