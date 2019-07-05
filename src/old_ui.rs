use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
// use web_sys::ImageData;
use crate::state;

use nalgebra::{Point2};


use crate::ui::*;

macro_rules! listen {
    ($base:expr, $name:expr, $evt: ty, $body:expr) => {
        let c = Closure::wrap(Box::new($body) as Box<FnMut($evt)>);
        $base.add_event_listener_with_callback($name, c.as_ref().unchecked_ref())?;
        c.forget();
    };
}

pub fn setup_button() -> Result<(), JsValue> {
    listen!(
        get_button("share")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            crate::state::try_with(|state| {
                // let res = serde_json::to_string(&state.config).unwrap();
                // location.set_hash(&res);
                let encoded = bincode::serialize(&state.config).unwrap();
                let zipped = miniz_oxide::deflate::compress_to_vec(&encoded, 10);
                log!("Sharing {} vs {}", encoded.len(), zipped.len());

                let b64 = base64::encode(&zipped);
                set_location_hash(&b64);
                Ok(())
            })
        }
    );

    listen!(
        get_button("lasers")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            state::try_with(|state| {
                state.ui.show_lasers = !state.ui.show_lasers;
                let button = get_button("lasers")?;
                button.set_inner_html(if state.ui.show_lasers {
                    "hide lasers"
                } else {
                    "show lasers"
                });
                state.ui.mouse_over = state.ui.show_lasers;
                draw(state)?;
                Ok(())
            })
        }
    );

    Ok(())
}

pub fn setup_wall_ui() -> Result<(), JsValue> {
    listen!(
        get_button("resize")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            state::try_with(|state| {
                let wi = get_input("width")?;
                let hi = get_input("height")?;
                state.config.rendering.width = wi.value_as_number() as usize;
                state.config.rendering.height = hi.value_as_number() as usize;
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
                state.clear();
                state.reset_buffer();
                state.maybe_save_history();
                state.async_render(false)
            })
        }
    );

    listen!(
        get_button("undo")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| state::try_with(|state| state.undo())
    );

    listen!(
        get_button("redo")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| state::try_with(|state| state.redo())
    );

    Ok(())
}

pub fn reset_config(config: &shared::Config) -> Result<(), JsValue> {
    get_input("width")?.set_value_as_number(config.rendering.width as f64);
    get_input("height")?.set_value_as_number(config.rendering.height as f64);
    Ok(())
}
