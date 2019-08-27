use wasm_bindgen::prelude::*;
// use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
// use web_sys::ImageData;
use crate::state;
use crate::state::State;
use line::float;
use shared::line;
use shared::Wall;

use nalgebra::{Point2, Vector2};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Handle {
    Handle(usize),
    Move(Vector2<float>),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AddKindName {
    Light,
    Circle,
    Line,
    Parabola,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiState {
    pub selection: Option<Selection>,
    pub show_lasers: bool,
    pub mouse_over: bool,
    pub hovered: Option<(usize, Handle)>,
    pub last_mouse_pos: Point2<float>,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            selection: None,
            show_lasers: false,
            mouse_over: false,
            hovered: None,
            last_mouse_pos: Point2::new(0.0, 0.0),
        }
    }
}

// lazy_static! {
//     static ref STATE: std::sync::Mutex<UiState> = std::sync::Mutex::new(UiState {
//         selection: None,
//         show_lasers: false,
//         show_hist: false,
//         mouse_over: false,
//         hovered: None,
//         last_mouse_pos: Point2::new(0.0, 0.0)
//     });
// }

// pub fn use_ui<R, F: FnOnce(&mut UiState) -> R>(f: F) -> R {
//     f(&mut STATE.lock().unwrap())
// }

// pub fn state_ui<R, F: FnOnce(&mut crate::state::State, &mut UiState) -> R>(f: F) -> R {
//     crate::state::with(|state| use_ui(|ui| f(state, ui)))
// }

// pub fn try_state_ui<F: FnOnce(&mut crate::state::State, &mut UiState) -> Result<(), JsValue>>(
//     f: F,
// ) {
//     crate::state::try_with(|state| use_ui(|ui| f(state, ui)))
// }

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

fn draw_walls(state: &State, ui: &UiState, hover: Option<(usize, Handle)>) -> Result<(), JsValue> {
    state.ctx.set_fill_style(&JsValue::from_str("#aaa"));

    let (zoom, dx, dy) = state.config.transform();

    let dashes = js_sys::Array::new();
    dashes.push(&JsValue::from(2.0f64));
    dashes.push(&JsValue::from(3.0f64));
    state.ctx.set_line_dash(&dashes)?;
    state.ctx.set_line_width(1.0);
    state.ctx.set_stroke_style(&JsValue::from_str("#aaa"));

    let hovered_wall = if let Some(Selection::Wall(wid, Some(_))) = ui.selection {
        Some(wid)
    } else if let Some((wid, _)) = hover {
        Some(wid)
    } else {
        None
    };

    if let Some(wid) = hovered_wall {
        if let Some(wall) = state.config.walls.get(wid) {
            let mut extras = vec![];
            shared::extra_walls(vec![wall.clone()], &mut extras, &state.config);
            for wall in extras {
                let mut kind = wall.kind.clone();
                kind.scale(zoom);
                kind.translate(&Vector2::new(dx, dy));
                crate::draw::draw(&kind, &state.ctx, false);
            }
        }
    }

    let dashes = js_sys::Array::new();
    state.ctx.set_line_dash(&dashes)?;

    for (i, wall) in state.config.main_walls().iter().enumerate() {
        let (selected, just_sel) = match &ui.selection {
            Some(Selection::Wall(wid, _)) if *wid == i => (true, true),
            Some(Selection::Multiple(walls, _)) if walls.contains(&i) => (true, true),
            _ => match hover {
                Some((wid, _)) if wid == i => (true, false),
                _ => (false, false),
            },
        };

        let mut kind = wall.kind.clone();
        kind.scale(zoom);
        kind.translate(&Vector2::new(dx, dy));

        if selected {
            state.ctx.set_line_width(5.0);
            state.ctx.set_stroke_style(&JsValue::from_str("#fff"));
            crate::draw::draw(&kind, &state.ctx, true);
        } else if wall.hide {
            continue;
        }
        state.ctx.set_line_width(1.0);
        if wall.properties.reflect == 1.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("#faf"));
        } else if wall.properties.absorb == 1.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("#faa"));
        } else if wall.properties.reflect == 0.0 && wall.properties.absorb == 0.0 {
            state.ctx.set_stroke_style(&JsValue::from_str("#afa"));
        } else {
            state.ctx.set_stroke_style(&JsValue::from_str("#aaf"));
        }
        crate::draw::draw(&kind, &state.ctx, true);
        crate::draw::draw_handles(
            &kind,
            &state.ctx,
            5.0,
            just_sel,
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
        state.ctx.save();
        state.ctx.translate(dx as f64, dy as f64)?;
        state.ctx.scale(zoom as f64, zoom as f64)?;
        let count = 30;
        for light in state.config.all_lights().iter() {
            for i in 0..count {
                draw_laser(&state, i as f32 / count as f32, &light)?;
            }
        }
        state.ctx.restore();
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
    zoom: f32,
    walls: &[Wall],
    pos: &Point2<shared::line::float>,
) -> Option<(usize, Vector2<float>)> {
    let mut close = None;
    for (wid, wall) in walls.iter().enumerate() {
        if wall.hide {
            continue;
        }
        let dist = wall.kind.point_dist(pos);
        if dist < 15.0 / zoom {
            match close {
                Some((_, d, _)) if d < dist => (),
                _ => close = Some((wid, dist, wall.kind.point_base() - pos)),
            }
        }
    }
    return close.map(|(wid, _, pdiff)| (wid, pdiff));
}

fn find_collision(
    zoom: f32,
    walls: &[Wall],
    pos: &Point2<shared::line::float>,
    selected_wall: Option<usize>,
) -> Option<(usize, Handle)> {
    for (wid, wall) in walls.iter().enumerate() {
        if wall.hide {
            continue;
        }
        match wall
            .kind
            .check_handle(pos, 5.0 / zoom, selected_wall == Some(wid))
        {
            None => (),
            Some(id) => return Some((wid, Handle::Handle(id))),
        }
    }
    return find_wall_hover(zoom, walls, pos).map(|(wid, pdiff)| (wid, Handle::Move(pdiff)));
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
        .or_else(|_| bincode::deserialize::<shared::v4::Config>(&encoded).map(shared::from_v4))
        .or_else(|_| {
            bincode::deserialize::<shared::v3::Config>(&encoded)
                .map(shared::v4::from_v3)
                .map(shared::from_v4)
        })
        .or_else(|_| {
            bincode::deserialize::<shared::v2::Config>(&encoded)
                .map(shared::v3::from_v2)
                .map(shared::v4::from_v3)
                .map(shared::from_v4)
        })
        .or_else(|_| {
            bincode::deserialize::<shared::v1::Config>(&encoded)
                .map(shared::v2::from_v1)
                .map(shared::v3::from_v2)
                .map(shared::v4::from_v3)
                .map(shared::from_v4)
        })
}

pub fn parse_url_config(hash: &str) -> Option<shared::Config> {
    base64::decode(&hash)
        .ok()
        .and_then(|zipped| miniz_oxide::inflate::decompress_to_vec(&zipped).ok())
        .and_then(|encoded| deserialize_bincode(&encoded).ok())
}

pub fn get_url_config() -> Option<shared::Config> {
    let hash = location.hash();
    if hash.len() == 0 {
        return None;
    }
    let hash: String = hash[1..].into();
    parse_url_config(&hash)
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

// struct Input<F: FnMut(f32, bool) + 'static> {
//     name: &'static str,
//     cb: F,
// }

// impl<F: FnMut(f32, bool) + 'static> Input<F> {
//     fn new(name: &'static str, cb: F) -> Self {
//         Input {name, cb}
//     }
// }

pub fn draw_histogram(canvas: &web_sys::HtmlCanvasElement, state: &crate::state::State) {
    // let _ = shared::Timer::new("histogram");
    let (min, max) = match state.config.rendering.exposure.limits {
        None => return,
        Some((min, max)) => (min, max),
    };
    let min = min as f64;
    let max = (max as f64).max(min + 0.01);
    let dx = max - min;

    let full_width = canvas.width() as f64;

    // let full_width = state.config.rendering.width as f64;
    let x0 = full_width * min;
    let x1 = full_width * max;
    let bin_count = (200.0 * dx) as usize;
    let w = (x1 - x0) / bin_count as f64;
    let histogram = shared::render::histogram(&state.config, &state.buffer, bin_count);

    let height = canvas.height() as f64;

    // let height = state.config.rendering.height as f64 / 3.0;

    // log!("Ok {} x {}", height, full_width);

    let max = *histogram.iter().max().unwrap() as f64;

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.clear_rect(0.0, 0.0, full_width, height);

    ctx.set_fill_style(&"#f00".into());
    for (i, count) in histogram.iter().enumerate() {
        let count = *count;
        if count < 10 {
            continue;
        }
        let h = (count as f64 / max).sqrt() * height;
        ctx.fill_rect(x0 + i as f64 * w, height as f64 - h, w, h);
    }
}

pub fn draw(state: &crate::state::State) -> Result<(), JsValue> {
    draw_image(state)?;
    if state.ui.mouse_over {
        draw_walls(state, &state.ui, state.ui.hovered.clone())?;
    }
    if let Some(canvas) = &state.hist_canvas {
        draw_histogram(&canvas, state);
    }
    Ok(())
}

pub fn reset(config: &shared::Config, ui: &mut UiState) -> Result<(), JsValue> {
    ui.selection = None;
    // hide_wall_ui()?;
    Ok(())
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
        (_, Some(Selection::Wall(_, Some((Handle::Move(_), _)))))
        | (Some((_, Handle::Move(_))), _) => "grab",
        (_, Some(Selection::Wall(_, Some(_)))) | (Some(_), _) => "pointer",
        (_, Some(Selection::Multiple(_, Some(_)))) => "grab",
        (_, Some(Selection::Pan { .. })) => "all-scroll",
        _ => "default",
    };
    // log!("cursor {} - {:?} - {:?}", cursor, ui.hovered, ui.selection);
    canvas.style().set_property("cursor", cursor)
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
    canvas.set_width(config.rendering.width as u32);
    canvas.set_height(config.rendering.height as u32);

    // listen!(canvas, "mouseenter", web_sys::MouseEvent, move |_evt| {
    //     crate::state::try_with(|state| {
    //         state.ui.mouse_over = true;
    //         draw(state)
    //     })
    // });

    listen!(canvas, "mouseleave", web_sys::MouseEvent, move |_evt| {
        crate::state::try_with(|state| {
            match &state.hide_timeout {
                None => (),
                Some(tid) => crate::state::clear_timeout(tid),
            }
            state.ui.mouse_over = false;
            draw(state)
        })
    });

    listen!(canvas, "wheel", WheelEvent, move |evt: WheelEvent| {
        crate::state::try_with(|state| {
            state.config.rendering.zoom =
                (evt.delta_y() as f32 * -0.01 + state.config.rendering.zoom).max(0.1);
            evt.prevent_default();
            state.async_render(true)
        })
    });

    listen!(canvas, "mousedown", web_sys::MouseEvent, move |evt| {
        crate::state::try_with(|state| {
            state.maybe_save_history();
            let pos = mouse_pos(&state.config.rendering, &evt);
            use std::ops::Deref;
            evt.deref().prevent_default();

            if let Some(Selection::Adding(kind)) = &mut state.ui.selection {
                state.config.walls.push(Wall::new(match kind {
                    AddKindName::Light => unimplemented!(),
                    AddKindName::Line => {
                        shared::WallType::line(pos.clone() + Vector2::new(1.0, 1.0), pos)
                    }
                    AddKindName::Circle => shared::WallType::circle(
                        pos.clone(),
                        5.0,
                        -shared::line::PI,
                        shared::line::PI,
                    ),
                    AddKindName::Parabola => shared::WallType::parabola(
                        pos.clone(),
                        Vector2::new(0.0, -50.0),
                        50.0,
                        -50.0,
                    ),
                }));
                state.ui.selection = Some(Selection::Wall(
                    state.config.walls.len() - 1,
                    Some((Handle::Handle(0), pos)),
                ));
                update_cursor(&state.ui)?;
                return draw(state);
            }

            let selected_wall = match state.ui.selection {
                Some(Selection::Wall(wid, _)) => Some(wid),
                _ => None,
            };

            match find_collision(
                state.config.rendering.zoom,
                &state.config.walls,
                &pos,
                selected_wall,
            ) {
                None => {
                    state.ui.selection = Some(Selection::Pan {
                        grab: pos,
                        center: state.config.rendering.center,
                    });
                    state.ui.hovered = None;
                    // hide_wall_ui()?;
                }
                Some((wid, id)) => {
                    if evt.shift_key() {
                        let mut walls = match &state.ui.selection {
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
                        state.ui.selection = Some(Selection::Multiple(walls, Some((pdiffs, pos))));
                    // hide_wall_ui()?;
                    } else if let Some(Selection::Multiple(walls, _)) = &state.ui.selection {
                        if walls.contains(&wid) {
                            let mut walls = walls.clone();
                            walls.push(wid);
                            let pdiffs = walls
                                .iter()
                                .map(|wid| state.config.walls[*wid].kind.point_base() - pos)
                                .collect();
                            state.ui.selection =
                                Some(Selection::Multiple(walls, Some((pdiffs, pos))));
                        // hide_wall_ui()?;
                        } else {
                            state.ui.selection = Some(Selection::Wall(wid, Some((id, pos))));
                            state.ui.hovered = None;
                        }
                    // old_ui::show_wall_ui(wid, &state.config.walls[wid])?;
                    } else {
                        state.ui.selection = Some(Selection::Wall(wid, Some((id, pos))));
                        state.ui.hovered = None;
                        // old_ui::show_wall_ui(wid, &state.config.walls[wid])?;
                    }
                }
            };
            update_cursor(&state.ui)?;
            state.send_on_change();
            draw(state)
        })
    });

    listen!(
        web_sys::window().unwrap(),
        "keydown",
        web_sys::KeyboardEvent,
        move |evt: web_sys::KeyboardEvent| {
            if evt.key() == "Meta" {
                state::with(|state| {
                    match &mut state.ui.selection {
                        Some(Selection::Wall(_, Some((_, orig)))) => {
                            *orig = state.ui.last_mouse_pos.clone();
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
            match &state.hide_timeout {
                None => (),
                Some(tid) => crate::state::clear_timeout(tid),
            }
            state.hide_timeout = Some(crate::state::set_timeout(
                move || {
                    crate::state::try_with(|state| {
                        state.ui.mouse_over = false;
                        draw(state)
                    })
                },
                600.0,
            ));
            if !state.ui.mouse_over {
                state.ui.mouse_over = true;
                // draw(state)?;
            }

            // web_sys::
            // use_ui(|ui| -> Result<(), JsValue> {
            let mut pos = mouse_pos(&state.config.rendering, &evt);
            state.ui.last_mouse_pos = pos.clone();
            match &mut state.ui.selection {
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
                    let selected_wall = match state.ui.selection {
                        Some(Selection::Wall(wid, _)) => Some(wid),
                        _ => None,
                    };
                    match find_collision(
                        state.config.rendering.zoom,
                        &state.config.walls,
                        &mouse_pos(&state.config.rendering, &evt),
                        selected_wall,
                    ) {
                        Some((wid, id)) => state.ui.hovered = Some((wid, id)),
                        None => state.ui.hovered = None,
                    }
                    Ok(())
                }
            }?;
            use std::ops::Deref;
            evt.deref().prevent_default();
            update_cursor(&state.ui)?;
            draw(state)?;
            // })?;
            Ok(())
        })
    });

    listen!(canvas, "mouseup", web_sys::MouseEvent, move |_evt| {
        state::try_with(|state| {
            match &state.ui.selection {
                Some(Selection::Wall(wid, Some((id, _)))) => {
                    // state.ui.hovered = None;
                    state.ui.hovered = Some((*wid, Handle::Move(nalgebra::zero())));
                    state.ui.selection = Some(Selection::Wall(*wid, None));
                    state.async_render(false)?;
                }
                Some(Selection::Pan { .. }) => {
                    state.ui.selection = None;
                    state.async_render(false)?;
                }
                Some(Selection::Multiple(wids, _)) => {
                    state.ui.selection = Some(Selection::Multiple(wids.clone(), None));
                    state.async_render(false)?;
                }
                _ => (),
            };
            state.maybe_save_history();

            update_cursor(&state.ui)?;
            state.send_on_change();
            Ok(())
        })
    });

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok(ctx)
}
