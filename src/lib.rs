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
) -> Result<(usize, js_sys::Uint32Array, usize, f64), JsValue> {
    let obj = evt.data();
    let id = js_sys::Reflect::get(&obj, &"id".into())?
        .as_f64()
        .expect("should be a number") as usize;
    let total_rays = js_sys::Reflect::get(&obj, &"total_rays".into())?
        .as_f64()
        .expect("should be a number") as usize;
    let total_seconds = js_sys::Reflect::get(&obj, &"total_seconds".into())?
        .as_f64()
        .expect("should be a number");
    let buffer = js_sys::Reflect::get(&obj, &"buffer".into())?;
    let buff = js_sys::ArrayBuffer::from(buffer);
    let uarr = js_sys::Uint32Array::new_with_byte_offset(&buff.dyn_into::<JsValue>()?, 0);
    Ok((id, uarr, total_rays, total_seconds))
}

fn on_message(wid: usize, evt: web_sys::MessageEvent) -> Result<(), JsValue> {
    let (id, uarr, total_rays, total_seconds) = parse_worker_message(evt)?;

    if let Some(Ok(node)) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("total_rays"))
        .map(|node| node.dyn_into::<web_sys::HtmlElement>())
    {
        node.set_inner_text(&format!("{}", total_rays))
    }

    if let Some(Ok(node)) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("fps"))
        .map(|node| node.dyn_into::<web_sys::HtmlElement>())
    {
        node.set_inner_text(&format!(
            "{:.2}k",
            total_rays as f64 / total_seconds / 1000.0
        ))
    }

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
                .into_serde::<shared::v4::Config>()
                .map(shared::from_v4)
        })
        .or_else(|_| {
            encoded
                .into_serde::<shared::v3::Config>()
                .map(shared::v4::from_v3)
                .map(shared::from_v4)
        })
        .or_else(|_| {
            encoded
                .into_serde::<shared::v2::Config>()
                .map(shared::v3::from_v2)
                .map(shared::v4::from_v3)
                .map(shared::from_v4)
        })
        .or_else(|_| {
            encoded
                .into_serde::<shared::v1::Config>()
                .map(shared::v2::from_v1)
                .map(shared::v3::from_v2)
                .map(shared::v4::from_v3)
                .map(shared::from_v4)
        })
}

fn update_config(config: shared::Config, mut reset: bool, checkpoint: bool) {
    state::try_with(|state| {
        if config == state.config {
            reset = false;
        }
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
            state.last_rendered_config = None;
        }
        if reset || checkpoint {
            state.maybe_save_history();
        }
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
pub fn test_run(canvas: web_sys::HtmlCanvasElement) {
    // ok, here we go
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let cx = width / 2.0;
    let cy = height / 2.0;
    ctx.begin_path();
    ctx.ellipse(
        cx as f64,
        cy as f64,
        100.0,
        100.0,
        0.0,
        0.0,
        std::f32::consts::PI as f64 * 2.0,
    )
    .unwrap();
    ctx.set_fill_style(&"#f00".into());
    ctx.fill();

    let width = 100;
    let height = 100;
    let mut brightness_data = vec![0; width * height];
    let num = 800;
    for i in 0..num {
        let angle = i as f32 / num as f32 * std::f32::consts::PI * 2.0;
        let dx = angle.cos() * 60.0;
        let dy = angle.sin() * 60.0;

        shared::line_algos::wu(
            (50.0 + dx / 2.0, 50.0 + dy / 2.0),
            (50.0 + dx, 50.0 + dy),
            &mut brightness_data,
            width,
            height,
            255.0,
        );

        // shared::line::draw_line(
        //     (50.0 + dx / 2.0, 50.0 + dy / 2.0),
        //     (50.0 + dx, 50.0 + dy),
        //     &mut brightness_data,
        //     width,
        //     height,
        //     255.0,
        // );
    }
    let mut top = 0u32;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[y * width + x])
        }
    }
    let scale = 5;
    let mut colored = vec![0u8; width * height * 4 * scale * scale];
    for x in 0..width {
        for y in 0..height {
            let brightness = brightness_data[y * width + x];
            for x0 in 0..scale {
                for y0 in 0..scale {
                    let i = ((y * scale + y0) * width * scale + x * scale + x0) * 4;
                    colored[i + 0] = 255;
                    colored[i + 1] = 255;
                    colored[i + 2] = 255;
                    colored[i + 3] = (brightness as f32 / top as f32 * 255.0) as u8;
                }
            }
        }
    }

    let mut clamped = wasm_bindgen::Clamped(colored.clone());
    // let mut clamped = Clamped(state.buffer.clone());
    let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(clamped.as_mut_slice()),
        (width * scale) as u32,
        (height * scale) as u32,
    )
    .unwrap();
    ctx.put_image_data(&data, 0.0, 0.0).unwrap();
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
