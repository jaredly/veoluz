use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
// use web_sys::ImageData;
use crate::state::State;
use line::float;
use shared::line;
use shared::Wall;

use nalgebra::{Point2, Vector2};

#[derive(Clone, Copy, Debug)]
pub enum Handle {
    Handle(usize),
    Move(Vector2<float>),
}

#[derive(Clone, Copy, Debug)]
pub enum AddKindName {
    Light,
    Circle,
    Line,
    Parabola,
}

#[derive(Clone, Debug)]
pub enum Selection {
    Wall(usize, Option<(Handle, Point2<float>)>),
    Light(usize, bool),
    Adding(AddKindName),
    Pan {
        grab: Point2<float>,
        center: Point2<float>,
    },
    Multiple(Vec<usize>, Option<(Vec<Vector2<float>>, Point2<float>)>),
}

// #[derive(Clone)]
pub struct UiState {
    pub selection: Option<Selection>,
    pub show_lasers: bool,
    pub mouse_over: bool,
    pub hovered: Option<(usize, Handle)>,
    pub last_mouse_pos: Point2<float>,
}

lazy_static! {
    static ref STATE: std::sync::Mutex<UiState> = std::sync::Mutex::new(UiState {
        selection: None,
        show_lasers: false,
        mouse_over: false,
        hovered: None,
        last_mouse_pos: Point2::new(0.0, 0.0)
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
    let walls = state.config.all_walls();
    for _i in 0..10 {
        // log!("Ray: {:?}", ray);
        match shared::calculate::find_collision(&walls, &ray) {
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
                    shared::calculate::bounce_ray(&mut ray, toi, properties, left_side, normal);

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

    let (zoom, dx, dy) = state.config.transform();
    state.ctx.save();
    state.ctx.translate(dx as f64, dy as f64)?;
    state.ctx.scale(zoom as f64, zoom as f64)?;

    let dashes = js_sys::Array::new();
    dashes.push(&JsValue::from(2.0f64));
    dashes.push(&JsValue::from(3.0f64));
    state.ctx.set_line_dash(&dashes)?;
    state.ctx.set_line_width(1.0);
    state.ctx.set_stroke_style(&JsValue::from_str("#aaa"));

    let extras = state.config.extra_walls();
    for wall in extras {
        crate::draw::draw(&wall.kind, &state.ctx, false);
    }

    let dashes = js_sys::Array::new();
    state.ctx.set_line_dash(&dashes)?;

    for (i, wall) in state.config.main_walls().iter().enumerate() {
        let w = match &ui.selection {
            Some(Selection::Wall(wid, _)) if *wid == i => 3.0,
            Some(Selection::Multiple(walls, _)) if walls.contains(&i) => 3.0,
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
        crate::draw::draw(&wall.kind, &state.ctx, true);
        state.ctx.set_line_width(1.0);
        crate::draw::draw_handles(
            &wall.kind,
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
                _ => match ui.selection {
                    Some(Selection::Wall(wid, current_handle)) if wid == i => {
                        match current_handle {
                            Some((Handle::Handle(i), _)) => Some(i),
                            _ => None,
                        }
                    }
                    _ => None,
                },
            },
        )?;
    }

    if ui.show_lasers {
        let count = 30;
        for light in state.config.lights.iter() {
            for i in 0..count {
                draw_laser(&state, i as f32 / count as f32, &light)?;
            }
        }
    }
    state.ctx.restore();

    Ok(())
}

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

fn find_wall_hover(
    walls: &[Wall],
    pos: &Point2<shared::line::float>,
) -> Option<(usize, Vector2<float>)> {
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

pub fn set_location_hash(val: &str) {
    location.set_hash(val);
}

pub fn deserialize_bincode(encoded: &[u8]) -> Result<shared::Config, bincode::Error> {
    bincode::deserialize::<shared::Config>(&encoded)
        .or_else(|_| bincode::deserialize::<shared::v3::Config>(&encoded).map(shared::from_v3))
        .or_else(|_| {
            bincode::deserialize::<shared::v2::Config>(&encoded)
                .map(shared::v3::from_v2)
                .map(shared::from_v3)
        })
        .or_else(|_| {
            bincode::deserialize::<shared::v1::Config>(&encoded)
                .map(shared::v2::from_v1)
                .map(shared::v3::from_v2)
                .map(shared::from_v3)
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

pub fn get_button(id: &str) -> Result<web_sys::HtmlButtonElement, JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let button = document
        .get_element_by_id(id)
        .expect(&("get button ".to_owned() + id.clone()))
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    Ok(button)
}

pub fn get_element(id: &str) -> Result<web_sys::HtmlElement, JsValue> {
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

pub fn get_input(id: &str) -> Result<web_sys::HtmlInputElement, JsValue> {
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

pub fn set_text(id: &'static str, text: String) -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let element = document
        .get_element_by_id(id)
        .expect("get input")
        .dyn_into::<web_sys::HtmlElement>()?;
    element.set_inner_text(&text);
    Ok(())
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

pub fn setup_checkbox<F: FnMut(bool) + 'static>(
    name: &'static str,
    update: F,
) -> Result<(), JsValue> {
    let node = get_input(name)?;
    let rc = std::sync::Arc::new(std::cell::RefCell::new(update));
    let other = rc.clone();

    use std::ops::DerefMut;

    listen!(node, "change", web_sys::InputEvent, move |_evt| {
        let input = get_input(name).expect("No input");
        other.borrow_mut().deref_mut()(input.checked());
    });
    Ok(())
}

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

pub fn draw(ui: &UiState, state: &crate::state::State) -> Result<(), JsValue> {
    draw_image(state)?;
    if ui.mouse_over {
        draw_walls(state, ui, ui.hovered.clone())?;
    }
    Ok(())
}

pub fn reset(config: &shared::Config, ui: &mut UiState) -> Result<(), JsValue> {
    ui.selection = None;
    hide_wall_ui()?;
    old_ui::reset_config(config)
}

#[wasm_bindgen]
extern "C" {
    type WheelEvent;

    #[wasm_bindgen(method, getter, js_name = deltaY)]
    fn delta_y(this: &WheelEvent) -> f64;

    #[wasm_bindgen(method, js_name = preventDefault)]
    fn prevent_default(this: &WheelEvent);
}

fn update_cursor(ui: &UiState) -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let canvas = document
        .get_element_by_id("drawing")
        .expect("get Canvas")
        .dyn_into::<web_sys::HtmlElement>()?;
    let cursor = match (ui.hovered, &ui.selection) {
        (_, Some(Selection::Adding(_))) => "crosshair",
        (_, Some(Selection::Wall(_, Some(_)))) | (Some(_), _) => "pointer",
        (_, Some(Selection::Multiple(_, Some(_)))) => "drag",
        (_, Some(Selection::Pan { .. })) => "pointer",
        _ => "default",
    };
    // log!("cursor {} - {:?} - {:?}", cursor, ui.hovered, ui.selection);
    canvas.style().set_property("cursor", cursor)
}

use crate::old_ui;
use crate::old_ui::hide_wall_ui;

pub fn init(config: &shared::Config) -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("Document");
    let canvas = document
        .get_element_by_id("drawing")
        .expect("get Canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(config.rendering.width as u32);
    canvas.set_height(config.rendering.height as u32);

    old_ui::setup_button()?;
    old_ui::setup_wall_ui()?;
    old_ui::reset_config(config)?;

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

    listen!(canvas, "wheel", WheelEvent, move |evt: WheelEvent| {
        crate::state::try_with(|state| {
            state.config.rendering.zoom =
                (evt.delta_y() as f32 * -0.01 + state.config.rendering.zoom).max(0.0);
            evt.prevent_default();
            state.async_render(true)
        })
    });

    listen!(canvas, "mousedown", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| {
                state.maybe_save_history();
                let pos = mouse_pos(&state.config.rendering, &evt);
                use std::ops::Deref;
                evt.deref().prevent_default();

                if let Some(Selection::Adding(kind)) = &mut ui.selection {
                    state.config.walls.push(Wall::new(
                        match kind {
                            AddKindName::Light => unimplemented!(),
                            AddKindName::Line => shared::WallType::line(
                                pos.clone(),
                                pos
                            ),
                            AddKindName::Circle => shared::WallType::circle(
                                pos.clone(), 5.0, -shared::line::PI, shared::line::PI),
                            AddKindName::Parabola => shared::WallType::parabola(
                                pos.clone(), Vector2::new(0.0, 5.0), -50.0, 50.0)
                        }
                    ));
                    ui.selection = Some(Selection::Wall(state.config.walls.len() - 1, Some((Handle::Handle(0), pos))));
                    update_cursor(ui)?;
                    return draw(ui, state)
                }

                match find_collision(&state.config.walls, &pos) {
                    None => {
                        ui.selection = Some(Selection::Pan {
                            grab: pos,
                            center: state.config.rendering.center,
                        });
                        ui.hovered = None;
                        hide_wall_ui()?;
                    }
                    Some((wid, id)) => {
                        if evt.shift_key() {
                            let mut walls = match &ui.selection {
                                Some(Selection::Multiple(walls, _)) => walls.clone(),
                                Some(Selection::Wall(wid, _)) => vec![*wid],
                                _ => vec![],
                            };
                            // TODO allow removing
                            walls.push(wid);
                            let pdiffs = walls
                                .iter()
                                .map(|wid| state.config.walls[*wid].kind.point_base() - pos)
                                .collect();
                            ui.selection = Some(Selection::Multiple(walls, Some((pdiffs, pos))));
                            hide_wall_ui()?;
                        } else if let Some(Selection::Multiple(walls, _)) = &ui.selection {
                            if walls.contains(&wid) {
                                let mut walls = walls.clone();
                                walls.push(wid);
                                let pdiffs = walls
                                    .iter()
                                    .map(|wid| state.config.walls[*wid].kind.point_base() - pos)
                                    .collect();
                                ui.selection =
                                    Some(Selection::Multiple(walls, Some((pdiffs, pos))));
                                hide_wall_ui()?;
                            } else {
                                ui.selection = Some(Selection::Wall(wid, Some((id, pos))));
                                ui.hovered = None;
                            }
                            old_ui::show_wall_ui(wid, &state.config.walls[wid])?;
                        } else {
                            ui.selection = Some(Selection::Wall(wid, Some((id, pos))));
                            ui.hovered = None;
                            old_ui::show_wall_ui(wid, &state.config.walls[wid])?;
                        }
                    }
                };
                update_cursor(ui)?;
                draw(ui, state)
            })
        })
    });

    listen!(
        web_sys::window().unwrap(),
        "keydown",
        web_sys::KeyboardEvent,
        move |evt: web_sys::KeyboardEvent| {
            if evt.key() == "Meta" {
                use_ui(|ui| {
                    match &mut ui.selection {
                        Some(Selection::Wall(_, Some((_, orig)))) => {
                            *orig = ui.last_mouse_pos.clone();
                        }
                        _ => (),
                    };
                })
            }
            // log!("Key {}", evt.key());
        }
    );

    listen!(canvas, "mousemove", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            use_ui(|ui| -> Result<(), JsValue> {
                let mut pos = mouse_pos(&state.config.rendering, &evt);
                ui.last_mouse_pos = pos.clone();
                match &mut ui.selection {
                    Some(Selection::Wall(wid, Some((Handle::Move(pdiff), original_point)))) => {
                        if evt.meta_key() {
                            pos = *original_point + (pos - *original_point) / 10.0;
                        }
                        state.config.walls[*wid].kind.set_point_base(pos + *pdiff);
                        state.async_render(true)
                    }
                    Some(Selection::Wall(wid, Some((Handle::Handle(id), original_point)))) => {
                        if evt.meta_key() {
                            pos = *original_point + (pos - *original_point) / 10.0;
                        }
                        state.config.walls[*wid].kind.move_handle(*id, &pos);
                        state.async_render(true)
                    }
                    Some(Selection::Pan { grab, center }) => {
                        let pos = pos + (*center - state.config.rendering.center);
                        state.config.rendering.center = *center - (pos - *grab);
                        state.async_render(true)
                    }
                    Some(Selection::Multiple(wids, Some((pdiffs, original_point)))) => {
                        if evt.meta_key() {
                            pos = *original_point + (pos - *original_point) / 10.0;
                        }
                        for (wid, pdiff) in wids.iter().zip(pdiffs.clone()) {
                            state.config.walls[*wid].kind.set_point_base(pos + pdiff);
                        }
                        state.async_render(true)
                    }
                    _ => {
                        match find_collision(
                            &state.config.walls,
                            &mouse_pos(&state.config.rendering, &evt),
                        ) {
                            Some((wid, id)) => ui.hovered = Some((wid, id)),
                            None => ui.hovered = None,
                        }
                        Ok(())
                    }
                }?;
                use std::ops::Deref;
                evt.deref().prevent_default();
                update_cursor(ui)?;
                draw(ui, state)
            })?;
            Ok(())
        })
    });

    listen!(canvas, "mouseup", web_sys::MouseEvent, move |_evt| {
        try_state_ui(|state, ui| {
            match &ui.selection {
                Some(Selection::Wall(wid, Some((id, _)))) => {
                    ui.hovered = Some((*wid, *id));
                    ui.selection = Some(Selection::Wall(*wid, None));
                    state.async_render(false)?;
                }
                Some(Selection::Pan { .. }) => {
                    ui.selection = None;
                    state.async_render(false)?;
                }
                Some(Selection::Multiple(wids, _)) => {
                    ui.selection = Some(Selection::Multiple(wids.clone(), None));
                    state.async_render(false)?;
                }
                _ => (),
            };
            state.maybe_save_history();

            update_cursor(ui)?;
            Ok(())
        })
    });

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok(ctx)
}
