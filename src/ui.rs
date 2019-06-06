use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
// use web_sys::ImageData;
use crate::state::State;
use shared::{Config, Wall, WallType};

type UiState = (usize, usize);

lazy_static! {
    static ref STATE: std::sync::Mutex<Option<UiState>> = std::sync::Mutex::new(None);
}

pub fn use_ui<R, F: FnOnce(&mut Option<UiState>) -> R>(f: F) -> R {
    f(&mut STATE.lock().unwrap())
}

fn draw_image(state: &State) -> Result<(), JsValue> {
    state.ctx.put_image_data(&state.image_data, 0.0, 0.0)
}

fn draw_walls(state: &State, ui: &Option<(usize, usize)>) {
    state.ctx.set_stroke_style(&JsValue::from_str("green"));
    state.ctx.set_fill_style(&JsValue::from_str("#7fa"));

    for (i, wall) in state.config.walls.iter().enumerate() {
        wall.kind.draw(&state.ctx);
        wall.kind.draw_handles(
            &state.ctx,
            5.0,
            match ui {
                Some((wid, id)) if *wid == i => Some(*id),
                _ => None,
            },
        );
    }
}

macro_rules! listen {
    ($base:expr, $name:expr, $evt: ty, $body:expr) => {
        let c = Closure::wrap(Box::new($body) as Box<FnMut($evt)>);
        $base.add_event_listener_with_callback($name, c.as_ref().unchecked_ref())?;
        c.forget();
    };
}

use nalgebra::Point2;

fn mouse_pos(evt: &web_sys::MouseEvent) -> Point2<f32> {
    let ui: &web_sys::Event = evt.as_ref();
    let m = ui.target().unwrap();
    let target: &web_sys::Element = m.dyn_ref::<web_sys::Element>().unwrap();
    let rect = target.get_bounding_client_rect();
    Point2::new(
        (evt.x() as f32 - rect.x() as f32),
        (evt.y() as f32 - rect.y() as f32),
    )
}

fn find_collision(walls: &[Wall], pos: &Point2<shared::line::float>) -> Option<(usize, usize)> {
    for (wid, wall) in walls.iter().enumerate() {
        match wall.kind.check_handle(pos, 5.0) {
            None => (),
            Some(id) => return Some((wid, id)),
        }
    }
    return None;
}

pub fn init(config: &shared::Config) -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let canvas = document
        .get_element_by_id("drawing")
        .expect("get Canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(config.width as u32);
    canvas.set_height(config.height as u32);

    listen!(canvas, "mouseenter", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            draw_image(state)?;
            draw_walls(state, &None);
            Ok(())
        })
    });

    listen!(canvas, "mouseleave", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            draw_image(state)?;
            Ok(())
        })
    });

    listen!(canvas, "mousedown", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                *ui = find_collision(&state.config.walls, &mouse_pos(&evt));
                draw_image(state)?;
                draw_walls(state, ui);
                Ok(())
            })
        })
    });

    listen!(canvas, "mousemove", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| -> Result<(), JsValue> {
                let hover = match ui {
                    None => find_collision(&state.config.walls, &mouse_pos(&evt)),
                    Some((wid, id)) => {
                        state.config.walls[*wid]
                            .kind
                            .move_handle(*id, &mouse_pos(&evt));
                        state.async_render(true);
                        Some((*wid, *id))
                    }
                };
                draw_image(state)?;
                draw_walls(state, &hover);
                Ok(())
            })?;
            Ok(())
        })
    });

    listen!(canvas, "mouseup", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                match ui {
                    None => (),
                    Some(_) => {
                        state.async_render(false);
                    }
                }
                *ui = None;
            });
            Ok(())
        })
    });

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok(ctx)
}
