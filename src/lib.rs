mod utils;

use shared;
use shared::line;
use shared::{WallType, Wall};
use wasm_bindgen::prelude::*;
use nalgebra::{Point2, Vector2};
use ncollide2d::query::Ray;
use ncollide2d::shape::Ball;

use std::f32::consts::PI;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

// use rand::random;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, ImageBitmap};

struct State {
    config: shared::Config,
    buffer: Vec<u8>,
}

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

fn withOptState<F: FnOnce(&mut Option<State>)>(f: F) {
    f(&mut STATE.lock().unwrap())
}

fn setState(state: State) {
    withOptState(|wrapper| *wrapper = Some(state))
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn try_log<F: FnOnce() -> Result<(), JsValue>>(f: F) {
    match f() {
        Ok(()) => (),
        Err(err) => log!("{:?}", err)
    }
}

fn withState<F: FnOnce(&mut State)>(f: F) {
    match STATE.lock().unwrap().as_mut() {
        None => (),
        Some(mut state) => f(&mut state)
    }
}

fn tryWithState<F: FnOnce(&mut State) -> Result<(), JsValue>>(f: F) {
    withState(|state| try_log(|| f(state)))
}

use nalgebra as na;
use nalgebra::geometry::{Isometry2, Rotation2, Translation2};


use wasm_bindgen::JsCast;


fn on_message(evt: web_sys::MessageEvent) -> Result<(), JsValue> {
    let document = web_sys::window().expect("window").document().expect("Document");
    let canvas = document.get_element_by_id("drawing").expect("get Canvas");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let ctx = canvas.get_context("2d").expect("context").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let uarr = js_sys::Uint8ClampedArray::from(evt.data());
    tryWithState(|state| {
        uarr.copy_to(&mut state.buffer);

        let mut clamped = Clamped(state.buffer.clone());
        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(clamped.as_mut_slice()), state.config.width as u32, state.config.height as u32)?;

        ctx.put_image_data(&data, 0.0, 0.0)?;
        ctx.set_stroke_style(&JsValue::from_str("green"));

        for wall in state.config.walls.iter() {
            wall.kind.draw(&ctx)
        };
        Ok(())
    });

    Ok(())

}



#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    _real: f64,
    _imaginary: f64,
) -> Result<(), JsValue> {


    let cx = (width / 2) as line::float;
    let cy = (height / 2) as line::float;

    let mut walls = vec![

        // ncollide2d::shape::Segment::new(Point2::new(100.0, 100.0), Point2::new(101.0, 400.0)),
        // ncollide2d::shape::Segment::new(Point2::new(550.0, 100.0), Point2::new(551.0, 500.0)),
        // ncollide2d::shape::Segment::new(Point2::new(100.0, 100.0), Point2::new(350.0, 101.0)),
        // ncollide2d::shape::Segment::new(Point2::new(100.0, 550.0), Point2::new(500.0, 561.0)),

        // Wall::Circle(
        //     Ball::new(50.0),
        //     Point2::new(cx, cy - 150.0),
        //     -std::f32::consts::PI,
        //     std::f32::consts::PI
        // ),
        // Wall::Circle(
        //     Ball::new(50.0),
        //     Point2::new(cx + 250.0, cy),
        //     -std::f32::consts::PI / 2.0,
        //     std::f32::consts::PI / 2.0
        // ),
        // Wall::Circle(
        //     Ball::new(50.0),
        //     Point2::new(cx, cy + 250.0),
        //     0.0,
        //     std::f32::consts::PI
        // ),

    ];

    let count = 5;

    let radius = 100.0;
    let by = line::PI * 2.0 / (count as line::float);

    for i in 0..count {
        let theta = i as line::float * by;

        let r0 = 100.0;
        let r1 = 250.0;
        let td = by / 2.0;

        let index = 0.6;

        // walls.push(Wall::transparent(WallType::Line(ncollide2d::shape::Segment::new(
        //     Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
        //     Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
        // )), 1.1));

        walls.push(Wall::transparent(WallType::Line(ncollide2d::shape::Segment::new(
            Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
            Point2::new(cx + (theta + td).cos() * r0, cy + (theta + td).sin() * r0),
        )), index));

        walls.push(Wall::transparent(WallType::Line(ncollide2d::shape::Segment::new(
            Point2::new(cx + theta.cos() * r1, cy + theta.sin() * r1),
            Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
        )), 1.0 / index));

        walls.push(Wall::transparent(WallType::Circle(
            Ball::new(r0 / 5.0),
            Point2::new(
                cx + (theta).cos() * r0 / 2.0,
                cy + (theta).sin() * r0 / 2.0,
            ),
            // -PI,
            // PI,
            theta + PI / 2.0,
            theta - PI / 2.0,
        ), 0.8));

        // walls.push(Wall::mirror(WallType::Circle(
        //     Ball::new(r0 / 5.0),
        //     Point2::new(
        //         cx + (theta).cos() * r0 / 2.0,
        //         cy + (theta).sin() * r0 / 2.0,
        //     ),
        //     theta - PI / 2.0,
        //     theta + PI / 2.0,
        // )));

        // walls.push(Wall::mirror(WallType::Circle(
        //     Ball::new(radius),
        //     Point2::new(
        //         cx + (theta).cos() * (radius - 20.0),
        //         cy + (theta).sin() * (radius - 20.0),
        //     ),
        //     0.0 + theta,
        //     PI + theta
        //     // angle_norm(theta),
        //     // angle_norm(theta + 0.1),
        //     // angle_norm(theta - std::f32::consts::PI * 4.0 / 5.0),
        //     // angle_norm(theta - std::f32::consts::PI * 3.0 / 5.0),
        // )))
    }

    let config = shared::Config::new(walls, width as usize, height as usize);
    let cloned = config.clone();

    setState(State {
        config: cloned,
        buffer: vec![0_u8;(width * height * 4) as usize],
    });

    // let mut data = shared::zen_photon(&config);
    // let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    // ctx.put_image_data(&data, 0.0, 0.0)?;

    let worker = web_sys::Worker::new("../worker/dist/bundle.js")?;
    let f = Closure::wrap(Box::new(|evt: web_sys::MessageEvent| try_log(|| on_message(evt))) as Box<FnMut(web_sys::MessageEvent)>);
    worker.set_onmessage(Some(f.as_ref().unchecked_ref()));
    f.forget();

    log!("Sending a message to the working");
    worker.post_message(&JsValue::from_serde(&config).unwrap())?;

            ctx.set_stroke_style(&JsValue::from_str("green"));
            for wall in config.walls.iter() {
                wall.kind.draw(&ctx);
            }

    // Ok(f.as_ref().unchecked_ref())
    Ok(())
}