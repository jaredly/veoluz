use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
// use web_sys::ImageData;
use crate::state::State;
use shared::{Config, Wall, WallType};

#[derive(Clone)]
pub struct UiState {
    selected_wall: usize,
    current_handle: Option<usize>,
    show_lasers: bool,
}

lazy_static! {
    static ref STATE: std::sync::Mutex<UiState> = std::sync::Mutex::new(UiState {
        selected_wall: 0,
        current_handle: None,
        show_lasers: true,
    });
}

pub fn use_ui<R, F: FnOnce(&mut UiState) -> R>(f: F) -> R {
    f(&mut STATE.lock().unwrap())
}

fn draw_image(state: &State) -> Result<(), JsValue> {
    state.ctx.put_image_data(&state.image_data, 0.0, 0.0)
}

fn draw_laser(
    state: &State,
    vector: nalgebra::Vector2<shared::line::float>,
) -> Result<(), JsValue> {
    let mut ray = ncollide2d::query::Ray::new(state.config.light_source, vector);
    for _i in 0..10 {
        // log!("Ray: {:?}", ray);
        match shared::find_collision(&state.config.walls, &ray) {
            None => {
                state.ctx.set_stroke_style(&"red".into());
                state.ctx.begin_path();
                state.ctx.move_to(ray.origin.x as f64, ray.origin.y as f64);
                let p = ray.point_at(600.0);
                state.ctx.line_to(p.x as f64, p.y as f64);
                state.ctx.stroke();
                break;
            }
            Some((toi, properties, left_side, normal)) => {
                let (new_origin, stop) =
                    shared::bounce_ray(&mut ray, toi, properties, left_side, normal);

                state.ctx.set_stroke_style(&"red".into());
                state.ctx.begin_path();
                state.ctx.move_to(ray.origin.x as f64, ray.origin.y as f64);
                state.ctx.line_to(new_origin.x as f64, new_origin.y as f64);
                state.ctx.stroke();

                let n = normal.normalize();
                state
                    .ctx
                    .set_stroke_style(&(if left_side { "blue" } else { "orange" }).into());
                state.ctx.begin_path();
                state.ctx.move_to(
                    (new_origin.x - n.x * 5.0) as f64,
                    (new_origin.y - n.y * 5.0) as f64,
                );
                state.ctx.line_to(
                    (new_origin.x + n.x * 15.0) as f64,
                    (new_origin.y + n.y * 15.0) as f64,
                );
                state.ctx.stroke();

                ray.origin = new_origin;
                if stop {
                    break;
                }
            }
        }
    }
    Ok(())
}

use nalgebra::{Point2, Vector2};

fn vector_dir(dir: f32) -> Vector2<f32> {
    Vector2::new(dir.cos(), dir.sin())
}

fn draw_walls(state: &State, ui: &UiState, hover: Option<(usize, usize)>) -> Result<(), JsValue> {
    state.ctx.set_fill_style(&JsValue::from_str("#aaa"));

    for (i, wall) in state.config.walls.iter().enumerate() {
        state
            .ctx
            .set_line_width(if ui.selected_wall == i { 3.0 } else { 1.0 });
        if wall.properties.reflect == 1.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("yellow"));
        } else if wall.properties.absorb == 1.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("red"));
        } else if wall.properties.reflect == 0.0 && wall.properties.absorb == 0.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("green"));
        } else {
            state.ctx.set_stroke_style(&JsValue::from_str("blue"));
        }
        wall.kind.draw(&state.ctx);
        state.ctx.set_line_width(1.0);
        wall.kind.draw_handles(
            &state.ctx,
            5.0,
            match hover {
                None => {
                    if ui.selected_wall == i {
                        ui.current_handle
                    } else {
                        None
                    }
                }
                Some((wid, id)) => {
                    if wid == i {
                        Some(id)
                    } else {
                        None
                    }
                }
            },
        )?;
    }

    if ui.show_lasers {
        let count = 30;
        for i in 0..count {
            draw_laser(
                &state,
                vector_dir(std::f32::consts::PI * 2.0 / count as f32 * i as f32),
            )?;
        }
    }

    Ok(())
}

macro_rules! listen {
    ($base:expr, $name:expr, $evt: ty, $body:expr) => {
        let c = Closure::wrap(Box::new($body) as Box<FnMut($evt)>);
        $base.add_event_listener_with_callback($name, c.as_ref().unchecked_ref())?;
        c.forget();
    };
}

fn mouse_pos(evt: &web_sys::MouseEvent) -> Point2<f32> {
    let ui: &web_sys::Event = evt.as_ref();
    let m = ui.target().unwrap();
    let target: &web_sys::Element = m.dyn_ref::<web_sys::Element>().unwrap();
    let rect = target.get_bounding_client_rect();
    Point2::new(
        evt.x() as f32 - rect.x() as f32,
        evt.y() as f32 - rect.y() as f32,
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

#[wasm_bindgen]
extern "C" {
    type Location;
    static location: Location;

    #[wasm_bindgen(method, getter, structural)]
    fn hash(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    fn set_hash(this: &Location, val: &str);
}

pub fn get_url_config() -> Option<shared::Config> {
    let hash = location.hash();
    if hash.len() == 0 {
        return None;
    }
    let hash: String = hash[1..].into();
    base64::decode(&hash)
        .ok()
        .and_then(|encoded| bincode::deserialize(&encoded).ok())
}

pub fn setup_button() -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let button = document
        .get_element_by_id("render")
        .expect("get button")
        .dyn_into::<web_sys::HtmlButtonElement>()?;

    listen!(button, "click", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            state.async_render(true)?;
            Ok(())
        })
    });

    let button = document
        .get_element_by_id("share")
        .expect("get share button")
        .dyn_into::<web_sys::HtmlButtonElement>()?;

    listen!(button, "click", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            // let res = serde_json::to_string(&state.config).unwrap();
            // location.set_hash(&res);
            let encoded = bincode::serialize(&state.config).unwrap();
            let b64 = base64::encode(&encoded);
            location.set_hash(&b64);
            Ok(())
        })
    });

    Ok(())
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

    setup_button()?;

    listen!(canvas, "mouseenter", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            draw_image(state)?;
            use_ui(|ui| draw_walls(state, ui, None))?;
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
                match find_collision(&state.config.walls, &mouse_pos(&evt)) {
                    None => (),
                    Some((wid, id)) => {
                        ui.selected_wall = wid;
                        ui.current_handle = Some(id);
                    }
                }
                draw_image(state)?;
                draw_walls(state, ui, None);
                Ok(())
            })
        })
    });

    listen!(canvas, "mousemove", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| -> Result<(), JsValue> {
                let hover = match ui.current_handle {
                    None => match find_collision(&state.config.walls, &mouse_pos(&evt)) {
                        None => None,
                        Some((wid, id)) => Some((wid, id))
                    },
                    Some(id) => {
                        state.config.walls[ui.selected_wall]
                            .kind
                            .move_handle(id, &mouse_pos(&evt));
                        state.async_render(true);
                        Some((ui.selected_wall, id))
                    }
                };
                draw_image(state)?;
                draw_walls(state, ui, hover)?;
                Ok(())
            })?;
            Ok(())
        })
    });

    listen!(canvas, "mouseup", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                match ui.current_handle {
                    None => (),
                    Some(_) => {
                        state.async_render(false);
                    }
                };
                ui.current_handle = None;
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
