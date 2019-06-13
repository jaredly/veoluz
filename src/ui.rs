use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
// use web_sys::ImageData;
use crate::state::State;
use shared::line;
use shared::{Config, Wall, WallType};

use nalgebra::{Point2, Vector2};

#[derive(Clone, Copy)]
pub enum Handle {
    Handle(usize),
    Move(Vector2<line::float>),
}

// #[derive(Clone)]
pub struct UiState {
    selected_wall: Option<(usize, Option<Handle>)>,
    show_lasers: bool,
    mouse_over: bool,
    hovered: Option<(usize, Handle)>,
}

lazy_static! {
    static ref STATE: std::sync::Mutex<UiState> = std::sync::Mutex::new(UiState {
        selected_wall: None,
        show_lasers: false,
        mouse_over: false,
        hovered: None,
    });
}

pub fn use_ui<R, F: FnOnce(&mut UiState) -> R>(f: F) -> R {
    f(&mut STATE.lock().unwrap())
}

pub fn state_ui<R, F: FnOnce(&mut crate::state::State, &mut UiState) -> R>(f: F) -> R {
    crate::state::with(|state| use_ui(|ui| f(state, ui)))
}

pub fn try_state_ui<F: FnOnce(&mut crate::state::State, &mut UiState) -> Result<(), JsValue>>(
    f: F,
) {
    crate::state::try_with(|state| use_ui(|ui| f(state, ui)))
}

fn draw_image(state: &State) -> Result<(), JsValue> {
    state.ctx.put_image_data(&state.image_data, 0.0, 0.0)
}

fn draw_laser(
    state: &State,
    direction: line::float,
    light: &shared::LightSource,
) -> Result<(), JsValue> {
    let mut ray = light.kind.spawn(direction);
    let walls = shared::all_walls(&state.config);
    for _i in 0..10 {
        // log!("Ray: {:?}", ray);
        match shared::find_collision(&walls, &ray) {
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

fn vector_dir(dir: f32) -> Vector2<f32> {
    Vector2::new(dir.cos(), dir.sin())
}

fn draw_walls(state: &State, ui: &UiState, hover: Option<(usize, Handle)>) -> Result<(), JsValue> {
    state.ctx.set_fill_style(&JsValue::from_str("#aaa"));

    let mut extras = vec![];
    shared::extra_walls(&mut extras, &state.config);
    for wall in extras {
        state.ctx.set_line_width(1.0);
        state.ctx.set_stroke_style(&JsValue::from_str("#aaa"));
        crate::draw::draw(&wall.kind, &state.ctx);
    }

    for (i, wall) in state.config.walls.iter().enumerate() {
        let w = match ui.selected_wall {
            Some((wid, _)) if wid == i => 3.0,
            _ => match hover {
                Some((wid, _)) if wid == i => 2.0,
                _ => 1.0,
            },
        };
        state.ctx.set_line_width(w);
        if wall.properties.reflect == 1.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("yellow"));
        } else if wall.properties.absorb == 1.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("red"));
        } else if wall.properties.reflect == 0.0 && wall.properties.absorb == 0.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("green"));
        } else {
            state.ctx.set_stroke_style(&JsValue::from_str("blue"));
        }
        crate::draw::draw(&wall.kind, &state.ctx);
        state.ctx.set_line_width(1.0);
        crate::draw::draw_handles(&wall.kind, 
            &state.ctx,
            5.0,
            match hover {
                Some((wid, Handle::Handle(id))) => {
                    if wid == i {
                        Some(id)
                    } else {
                        None
                    }
                }
                _ => match ui.selected_wall {
                    Some((wid, current_handle)) if wid == i => match current_handle {
                        Some(Handle::Handle(i)) => Some(i),
                        _ => None,
                    },
                    _ => None,
                },
            },
        )?;
    }

    if ui.show_lasers {
        let count = 30;
        for light in state.config.lights.iter() {
            for i in 0..count {
                draw_laser(
                    &state,
                    i as f32/ count as f32,
                    &light
                )?;
            }
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

fn find_wall_hover(
    walls: &[Wall],
    pos: &Point2<shared::line::float>,
) -> Option<(usize, Vector2<line::float>)> {
    let mut close = None;
    for (wid, wall) in walls.iter().enumerate() {
        let dist = wall.kind.point_dist(pos);
        if dist < 15.0 {
            match close {
                Some((_, d, _)) if d < dist => (),
                _ => close = Some((wid, dist, wall.kind.point_base() - pos)),
            }
        }
    }
    return close.map(|(wid, _, pdiff)| (wid, pdiff));
}

fn find_collision(walls: &[Wall], pos: &Point2<shared::line::float>) -> Option<(usize, Handle)> {
    for (wid, wall) in walls.iter().enumerate() {
        match wall.kind.check_handle(pos, 5.0) {
            None => (),
            Some(id) => return Some((wid, Handle::Handle(id))),
        }
    }
    return find_wall_hover(walls, pos).map(|(wid, pdiff)| (wid, Handle::Move(pdiff)));
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

pub fn deserialize_bincode(encoded: &[u8]) -> Result<shared::Config, bincode::Error> {
    bincode::deserialize::<shared::Config>(&encoded).or_else(|_| {
        bincode::deserialize::<shared::v1::Config>(&encoded).map(shared::from_v1)
    })
}

pub fn get_url_config() -> Option<shared::Config> {
    let hash = location.hash();
    if hash.len() == 0 {
        return None;
    }
    let hash: String = hash[1..].into();
    base64::decode(&hash)
        .ok()
        .and_then(|zipped| miniz_oxide::inflate::decompress_to_vec(&zipped).ok())
        .and_then(|encoded| deserialize_bincode(&encoded).ok())
}

fn get_button(id: &str) -> Result<web_sys::HtmlButtonElement, JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let button = document
        .get_element_by_id(id)
        .expect("get button")
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    Ok(button)
}

fn get_element(id: &str) -> Result<web_sys::HtmlElement, JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let input = document
        .get_element_by_id(id)
        .expect("get input")
        .dyn_into::<web_sys::HtmlElement>()?;
    Ok(input)
}

fn get_input(id: &str) -> Result<web_sys::HtmlInputElement, JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let input = document
        .get_element_by_id(id)
        .expect("get input")
        .dyn_into::<web_sys::HtmlInputElement>()?;
    Ok(input)
}

// struct Input<F: FnMut(f32, bool) + 'static> {
//     name: &'static str,
//     cb: F,
// }

// impl<F: FnMut(f32, bool) + 'static> Input<F> {
//     fn new(name: &'static str, cb: F) -> Self {
//         Input {name, cb}
//     }
// }

pub fn setup_input<F: FnMut(f32, bool) + 'static>(
    name: &'static str,
    update: F,
) -> Result<(), JsValue> {
    let node = get_input(name)?;
    let rc = std::sync::Arc::new(std::cell::RefCell::new(update));
    let other = rc.clone();

    use std::ops::DerefMut;

    listen!(node, "input", web_sys::InputEvent, move |_evt| {
        let input = get_input(name).expect("No input");
        rc.borrow_mut().deref_mut()(input.value_as_number() as f32, false);
    });

    listen!(node, "change", web_sys::InputEvent, move |_evt| {
        let input = get_input(name).expect("No input");
        other.borrow_mut().deref_mut()(input.value_as_number() as f32, true);
    });
    Ok(())
}

pub fn setup_button() -> Result<(), JsValue> {

    listen!(get_button("add_line")?, "click", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            state.config.walls.push(
                shared::Wall::new(
                    shared::WallType::basic_line(state.config.width, state.config.height)
                )
            );
            state.async_render(false)?;
            Ok(())
        })
    });

    listen!(get_button("add_parabola")?, "click", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            state.config.walls.push(
                shared::Wall::new(
                    shared::WallType::basic_parabola(state.config.width, state.config.height)
                )
            );
            state.async_render(false)?;
            Ok(())
        })
    });

    listen!(get_button("add_arc")?, "click", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            state.config.walls.push(
                shared::Wall::new(
                    shared::WallType::basic_circle(state.config.width, state.config.height)
                )
            );
            state.async_render(false)?;
            Ok(())
        })
    });

    listen!(get_button("remove")?, "click", web_sys::MouseEvent, move |_evt| {
        try_state_ui(|state, ui| {
            if let Some((wid, _)) = ui.selected_wall {
                ui.selected_wall = None;
                state.config.walls.remove(wid);
            }
            state.async_render(false)?;
            Ok(())
        })
    });

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
                location.set_hash(&b64);
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
                get_element("textarea")?.dyn_into::<web_sys::HtmlTextAreaElement>()?.set_value(&res);
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

pub fn draw(ui: &UiState, state: &crate::state::State) -> Result<(), JsValue> {
    draw_image(state)?;
    if ui.mouse_over {
        draw_walls(state, ui, ui.hovered.clone())?;
    }
    Ok(())
}

fn setup_wall_ui() -> Result<(), JsValue> {
    setup_input("reflect", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some((wid, _)) = ui.selected_wall {
                state.config.walls[wid].properties.reflect = value;
            }
            state.async_render(!finished)?;
            Ok(())
        })
    })?;

    setup_input("absorb", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some((wid, _)) = ui.selected_wall {
                state.config.walls[wid].properties.absorb = value;
            }
            state.async_render(!finished)?;
            Ok(())
        })
    })?;

    setup_input("roughness", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some((wid, _)) = ui.selected_wall {
                state.config.walls[wid].properties.roughness = value;
            }
            state.async_render(!finished)?;
            Ok(())
        })
    })?;

    setup_input("refraction", |value, finished| {
        try_state_ui(|state, ui| {
            if let Some((wid, _)) = ui.selected_wall {
                state.config.walls[wid].properties.refraction = value;
            }
            state.async_render(!finished)?;
            Ok(())
        })
    })?;

    setup_input("rotation", |value, finished| {
        try_state_ui(|state, ui| {
            state.config.reflection = value as u8;
            state.async_render(!finished)?;
            Ok(())
        })
    })?;

    setup_input("expose-low", |value, finished| {
        try_state_ui(|state, ui| {
            state.config.exposure.min = value as f32;
            if value as f32 > state.config.exposure.max - 0.01 {
                state.config.exposure.max = value as f32 + 0.01;
                get_input("expose-high")?.set_value_as_number(state.config.exposure.max as f64);
            }
            state.reexpose()?;
            Ok(())
        })
    })?;

    setup_input("expose-high", |value, finished| {
        try_state_ui(|state, ui| {
            state.config.exposure.max = value as f32;
            if (value as f32) < state.config.exposure.min + 0.01 {
                state.config.exposure.min = value as f32 - 0.01;
                get_input("expose-low")?.set_value_as_number(state.config.exposure.min as f64);
            }
            state.reexpose()?;
            Ok(())
        })
    })?;

    listen!(get_button("expose-fourth")?, "click", web_sys::MouseEvent, move |_evt| {
        try_state_ui(|state, ui| {
            state.config.exposure.curve = shared::Curve::FourthRoot;
            state.reexpose()?;
            Ok(())
        })
    });

    listen!(get_button("expose-square")?, "click", web_sys::MouseEvent, move |_evt| {
        try_state_ui(|state, ui| {
            state.config.exposure.curve = shared::Curve::SquareRoot;
            state.reexpose()?;
            Ok(())
        })
    });

    listen!(get_button("expose-linear")?, "click", web_sys::MouseEvent, move |_evt| {
        try_state_ui(|state, ui| {
            state.config.exposure.curve = shared::Curve::Linear;
            state.reexpose()?;
            Ok(())
        })
    });

    Ok(())
}

fn hide_wall_ui() -> Result<(), JsValue> {
    get_element("wall_ui")?
        .style()
        .set_property("display", "none")
}

fn show_wall_ui(idx: usize, wall: &Wall) -> Result<(), JsValue> {
    get_element("wall_ui")?
        .style()
        .set_property("display", "block")?;
    get_input("reflect")?.set_value_as_number(wall.properties.reflect as f64);
    get_input("absorb")?.set_value_as_number(wall.properties.absorb as f64);
    get_input("roughness")?.set_value_as_number(wall.properties.roughness as f64);
    get_input("refraction")?.set_value_as_number(wall.properties.refraction as f64);

    Ok(())
}

pub fn reset(config: &shared::Config) -> Result<(), JsValue> {
    get_input("rotation")?.set_value_as_number(config.reflection as f64);
    get_input("expose-low")?.set_value_as_number(config.exposure.min as f64);
    get_input("expose-high")?.set_value_as_number(config.exposure.max as f64);
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
    setup_wall_ui()?;
    reset(config)?;

    listen!(canvas, "mouseenter", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                ui.mouse_over = true;
                draw(ui, state)
            })
        })
    });

    listen!(canvas, "mouseleave", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                ui.mouse_over = false;
                draw(ui, state)
            })
        })
    });

    listen!(canvas, "mousedown", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                match find_collision(&state.config.walls, &mouse_pos(&evt)) {
                    None => {
                        ui.selected_wall = None;
                        ui.hovered = None;
                        hide_wall_ui();
                        Ok(())
                    }
                    Some((wid, id)) => {
                        ui.selected_wall = Some((wid, Some(id)));
                        ui.hovered = None;
                        show_wall_ui(wid, &state.config.walls[wid])
                    }
                }?;
                draw(ui, state)
            })
        })
    });

    listen!(canvas, "mousemove", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| -> Result<(), JsValue> {
                match ui.selected_wall {
                    Some((wid, Some(Handle::Move(pdiff)))) => {
                        let pos = mouse_pos(&evt);
                        state.config.walls[wid].kind.set_point_base(pos + pdiff);
                        state.async_render(true)
                    }
                    Some((wid, Some(Handle::Handle(id)))) => {
                        state.config.walls[wid]
                            .kind
                            .move_handle(id, &mouse_pos(&evt));
                        state.async_render(true)
                    }
                    _ => {
                        match find_collision(&state.config.walls, &mouse_pos(&evt)) {
                            Some((wid, id)) => ui.hovered = Some((wid, id)),
                            None => ui.hovered = None,
                        }
                        Ok(())
                    }
                }?;
                let document = web_sys::window()
                    .expect("window")
                    .document()
                    .expect("Document");
                let canvas = document
                    .get_element_by_id("drawing")
                    .expect("get Canvas")
                    .dyn_into::<web_sys::HtmlElement>()?;
                canvas.style().set_property(
                    "cursor",
                    match (ui.hovered, ui.selected_wall) {
                        (_, Some((_, Some(_)))) | (Some(_), _) => "pointer",
                        _ => "default",
                    },
                )?;
                draw(ui, state)
            })?;
            Ok(())
        })
    });

    listen!(canvas, "mouseup", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                match ui.selected_wall {
                    Some((wid, Some(id))) => {
                        ui.hovered = Some((wid, id));
                        ui.selected_wall = Some((wid, None));
                        state.async_render(false);
                    }
                    _ => (),
                };
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
