use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
// use web_sys::ImageData;
use crate::state::State;
use line::float;
use shared::line;
use shared::Wall;

use nalgebra::{Point2, Vector2};


use crate::ui::*;

macro_rules! listen {
    ($base:expr, $name:expr, $evt: ty, $body:expr) => {
        let c = Closure::wrap(Box::new($body) as Box<FnMut($evt)>);
        $base.add_event_listener_with_callback($name, c.as_ref().unchecked_ref())?;
        c.forget();
    };
}

fn mouse_pos(rendering: &shared::Rendering, evt: &web_sys::MouseEvent) -> Point2<f32> {
    let ui: &web_sys::Event = evt.as_ref();
    let m = ui.target().unwrap();
    let target: &web_sys::Element = m.dyn_ref::<web_sys::Element>().unwrap();
    let rect = target.get_bounding_client_rect();
    rendering.inverse_transform_point(&Point2::new(
        evt.x() as f32 - rect.x() as f32,
        evt.y() as f32 - rect.y() as f32,
    ))
}


pub fn setup_button() -> Result<(), JsValue> {
    listen!(
        get_button("add_line")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            use_ui(|ui| {
                ui.selection = Some(Selection::Adding(AddKindName::Line));
            })
        }
    );

    listen!(
        get_button("add_parabola")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            use_ui(|ui| {
                ui.selection = Some(Selection::Adding(AddKindName::Parabola));
            })
        }
    );

    listen!(
        get_button("add_arc")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            use_ui(|ui| {
                ui.selection = Some(Selection::Adding(AddKindName::Circle));
            })
        }
    );

    listen!(
        get_button("remove")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            try_state_ui(|state, ui| {
                // TODO support removing multiple walls probably
                if let Some(Selection::Wall(wid, _)) = ui.selection {
                    ui.selection = None;
                    hide_wall_ui()?;
                    state.config.walls.remove(wid);
                    state.async_render(false)?;
                }
                Ok(())
            })
        }
    );

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
        get_button("json")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            crate::state::try_with(|state| {
                let res = serde_json::to_string_pretty(&state.config).unwrap();
                get_element("textarea")?
                    .dyn_into::<web_sys::HtmlTextAreaElement>()?
                    .set_value(&res);
                Ok(())
            })
        }
    );

    listen!(
        get_button("lasers")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            try_state_ui(|state, ui| {
                ui.show_lasers = !ui.show_lasers;
                let button = get_button("lasers")?;
                button.set_inner_html(if ui.show_lasers {
                    "hide lasers"
                } else {
                    "show lasers"
                });
                ui.mouse_over = ui.show_lasers;
                draw(ui, state)?;
                Ok(())
            })
        }
    );

    Ok(())
}

pub fn setup_wall_ui() -> Result<(), JsValue> {
    setup_input("reflect", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some(Selection::Wall(wid, _)) = ui.selection {
                state.config.walls[wid].properties.reflect = value;
            }
            state.async_render(!finished)?;
            if (finished) {
                state.maybe_save_history()
            }
            Ok(())
        })
    })?;

    setup_input("absorb", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some(Selection::Wall(wid, _)) = ui.selection {
                state.config.walls[wid].properties.absorb = value;
            }
            state.async_render(!finished)?;
            if (finished) {
                state.maybe_save_history()
            }
            Ok(())
        })
    })?;

    setup_input("roughness", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some(Selection::Wall(wid, _)) = ui.selection {
                state.config.walls[wid].properties.roughness = value;
            }
            state.async_render(!finished)?;
            if (finished) {
                state.maybe_save_history()
            }
            Ok(())
        })
    })?;

    setup_input("refraction", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some(Selection::Wall(wid, _)) = ui.selection {
                state.config.walls[wid].properties.refraction = value;
            }
            state.async_render(!finished)?;
            if (finished) {
                state.maybe_save_history()
            }
            Ok(())
        })
    })?;

    setup_checkbox("reflection", |value| {
        try_state_ui(|state, _ui| {
            state.config.transform.reflection = value;
            state.async_render(false)?;
            state.maybe_save_history();
            Ok(())
        })
    })?;

    setup_input("rotation", |value, finished| {
        try_state_ui(|state, _ui| {
            state.config.transform.rotational_symmetry = value as u8;
            state.async_render(!finished)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("zoom", |value, finished| {
        try_state_ui(|state, _ui| {
            state.config.rendering.zoom = value as f32;
            set_text("zoom-text", format!("{}", value))?;
            state.async_render(!finished)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    listen!(
        get_button("resize")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            try_state_ui(|state, _ui| {
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

    setup_input("expose-low", |value, finished| {
        try_state_ui(|state, ui| {
            state.config.rendering.exposure.min = value as f32;
            if value as f32 > state.config.rendering.exposure.max - 0.01 {
                state.config.rendering.exposure.max = value as f32 + 0.01;
                get_input("expose-high")?
                    .set_value_as_number(state.config.rendering.exposure.max as f64);
            }
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("expose-high", |value, finished| {
        try_state_ui(|state, ui| {
            state.config.rendering.exposure.max = value as f32;
            if (value as f32) < state.config.rendering.exposure.min + 0.01 {
                state.config.rendering.exposure.min = value as f32 - 0.01;
                get_input("expose-low")?
                    .set_value_as_number(state.config.rendering.exposure.min as f64);
            }
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("b-r", |value, finished| {
        try_state_ui(|state, ui| {
            match &mut state.config.rendering.coloration {
                shared::Coloration::HueRange { .. } => (),
                shared::Coloration::Rgb { background, .. } => {
                    background.0 = value as u8;
                }
            };
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("b-g", |value, finished| {
        try_state_ui(|state, ui| {
            match &mut state.config.rendering.coloration {
                shared::Coloration::HueRange { .. } => (),
                shared::Coloration::Rgb { background, .. } => {
                    background.1 = value as u8;
                }
            };
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("b-b", |value, finished| {
        try_state_ui(|state, ui| {
            match &mut state.config.rendering.coloration {
                shared::Coloration::HueRange { .. } => (),
                shared::Coloration::Rgb { background, .. } => {
                    background.2 = value as u8;
                }
            };
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("f-r", |value, finished| {
        try_state_ui(|state, ui| {
            match &mut state.config.rendering.coloration {
                shared::Coloration::HueRange { .. } => (),
                shared::Coloration::Rgb { highlight, .. } => {
                    highlight.0 = value as u8;
                }
            };
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("f-g", |value, finished| {
        try_state_ui(|state, ui| {
            match &mut state.config.rendering.coloration {
                shared::Coloration::HueRange { .. } => (),
                shared::Coloration::Rgb { highlight, .. } => {
                    highlight.1 = value as u8;
                }
            };
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    setup_input("f-b", |value, finished| {
        try_state_ui(|state, ui| {
            match &mut state.config.rendering.coloration {
                shared::Coloration::HueRange { .. } => (),
                shared::Coloration::Rgb { highlight, .. } => {
                    highlight.2 = value as u8;
                }
            };
            state.reexpose(ui)?;
            if (finished) {
                state.maybe_save_history();
            }
            Ok(())
        })
    })?;

    listen!(
        get_button("expose-fourth")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            try_state_ui(|state, ui| {
                state.config.rendering.exposure.curve = shared::Curve::FourthRoot;
                state.reexpose(ui)?;
                state.maybe_save_history();
                Ok(())
            })
        }
    );

    listen!(
        get_button("expose-square")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            try_state_ui(|state, ui| {
                state.config.rendering.exposure.curve = shared::Curve::SquareRoot;
                state.reexpose(ui)?;
                state.maybe_save_history();
                Ok(())
            })
        }
    );

    listen!(
        get_button("expose-linear")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| {
            try_state_ui(|state, ui| {
                state.config.rendering.exposure.curve = shared::Curve::Linear;
                state.reexpose(ui)?;
                state.maybe_save_history();
                Ok(())
            })
        }
    );

    listen!(
        get_button("undo")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| try_state_ui(|state, _ui| state.undo())
    );

    listen!(
        get_button("redo")?,
        "click",
        web_sys::MouseEvent,
        move |_evt| try_state_ui(|state, _ui| state.redo())
    );

    Ok(())
}

pub fn hide_wall_ui() -> Result<(), JsValue> {
    get_element("wall_ui")?
        .style()
        .set_property("display", "none")
}

pub fn show_wall_ui(_idx: usize, wall: &Wall) -> Result<(), JsValue> {
    get_element("wall_ui")?
        .style()
        .set_property("display", "block")?;
    get_input("reflect")?.set_value_as_number(wall.properties.reflect as f64);
    get_input("absorb")?.set_value_as_number(wall.properties.absorb as f64);
    get_input("roughness")?.set_value_as_number(wall.properties.roughness as f64);
    get_input("refraction")?.set_value_as_number(wall.properties.refraction as f64);

    Ok(())
}

pub fn reset_config(config: &shared::Config) -> Result<(), JsValue> {
    match config.rendering.coloration {
        shared::Coloration::HueRange { .. } => (),
        shared::Coloration::Rgb {
            highlight,
            background,
        } => {
            get_input("f-r")?.set_value_as_number(highlight.0 as f64);
            get_input("f-g")?.set_value_as_number(highlight.1 as f64);
            get_input("f-b")?.set_value_as_number(highlight.2 as f64);
            get_input("b-r")?.set_value_as_number(background.0 as f64);
            get_input("b-g")?.set_value_as_number(background.1 as f64);
            get_input("b-b")?.set_value_as_number(background.2 as f64);
        }
    };
    get_input("rotation")?.set_value_as_number(config.transform.rotational_symmetry as f64);
    get_input("reflection")?.set_checked(config.transform.reflection);
    get_input("zoom")?.set_value_as_number(config.rendering.zoom as f64);
    get_input("width")?.set_value_as_number(config.rendering.width as f64);
    get_input("height")?.set_value_as_number(config.rendering.height as f64);
    set_text("zoom-text", format!("{}", config.rendering.zoom))?;
    get_input("expose-low")?.set_value_as_number(config.rendering.exposure.min as f64);
    get_input("expose-high")?.set_value_as_number(config.rendering.exposure.max as f64);
    Ok(())
}
