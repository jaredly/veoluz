extern crate num_traits;
use num_traits::{Float, Signed};
use std::mem::swap;

pub type float = f32;
pub type int = i16;
pub type uint = u32;
pub const PI: float = std::f32::consts::PI;

#[inline]
fn draw(
    data: &mut [uint],
    width: usize,
    height: usize,
    full: float,
    x: int,
    y: int,
    amount: float,
) {
    // TODO get rid of these if blocks by ensuring we're in scope to start with
    if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
        let index = (y as usize) * width + x as usize;
        let brightness = (amount * full) as uint;
        data[index] += brightness;
    }
}

// // A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        // web_sys::console::log_1(&format!( $( $t )* ).into());
        ()
    };
}

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = performance)]
//     fn now() -> f64;
// }

pub fn draw_line(
    mut start: Point<float>,
    mut end: Point<float>,
    data: &mut [uint],
    width: usize,
    height: usize,
    full: float,
) {
    // let p = web_sys::window().unwrap().performance().unwrap();
    // let s = now();
    // let s = std::time::SystemTime::now();
    // let _ = crate::Timer::new("draw line");
    // -- The Setup part

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    let steep = dy.abs() > dx.abs();

    if steep {
        start = (start.1, start.0);
        end = (end.1, end.0);
    }

    if start.0 > end.0 {
        swap(&mut start, &mut end);
    }
    // log!("Line xs = {} - {}", start.0, end.0);

    let gradient = (end.1 - start.1) / (end.0 - start.0);

    // if gradient == 0.0 {
    //     gradient = 1.0;
    // }

    // -- the algorithm

    // TODO once I clamp the poisitions to be in bounds, I can use unsigned things for x & y instead of signed...

    let mut y = start.1;

    /*
    for x in start.0.round() as int..end.0.round() as int {
        let fpart = y.fract();
        let yy = y as int;

        // Get the point
        let point = if steep { (yy, x) } else { (x, yy) };

        // TODO get rid of these if blocks by ensuring we're in scope to start with
        if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int {
            draw(data, width, full, point.0, point.1, 1.0 - fpart)
        }

        if fpart > 0.0 {
            let point = if steep { (yy + 1, x) } else { (x, yy + 1) };
            if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int {
                draw(data, width, full, point.0, point.1, fpart)
            }
        }
        y += gradient;
    }
    */

    if steep {
        let x0part = start.0.fract();
        if x0part != 0.0 {
            let total = 1.0 - x0part;
            let ypart = y.fract();
            draw(
                data,
                width,
                height,
                full,
                y as int,
                start.0 as int,
                total * (1.0 - ypart),
            );
            if ypart > 0.0 {
                draw(
                    data,
                    width,
                    height,
                    full,
                    y as int + 1,
                    start.0 as int,
                    total * ypart,
                );
            }
        }

        for x in start.0.ceil() as int..end.0.floor() as int {
            let fpart = y.fract();
            let yi = y as int;

            draw(data, width, height, full, yi, x, 1.0 - fpart);

            if fpart > 0.0 {
                draw(data, width, height, full, yi + 1, x, fpart)
            }
            y += gradient;
        }

        let x0part = end.0.fract();
        if x0part != 0.0 {
            let total = x0part;
            let ypart = y.fract();
            let last = end.0.floor() as int;
            draw(
                data,
                width,
                height,
                full,
                y as int,
                last,
                total * (1.0 - ypart),
            );
            if ypart > 0.0 {
                draw(data, width, height, full, y as int + 1, last, total * ypart);
            }
        }
    } else {
        let x0part = start.0.fract();
        if x0part != 0.0 {
            let total = 1.0 - x0part;
            let ypart = y.fract();
            draw(
                data,
                width,
                height,
                full,
                start.0 as int,
                y as int,
                total * (1.0 - ypart),
            );
            if ypart > 0.0 {
                draw(
                    data,
                    width,
                    height,
                    full,
                    start.0 as int,
                    y as int + 1,
                    total * ypart,
                );
            }
        }

        for x in start.0.ceil() as int..end.0.floor() as int {
            let fpart = y.fract();
            let yi = y as int;

            draw(data, width, height, full, x, yi, 1.0 - fpart);

            if fpart > 0.0 {
                // let point = (x, yi + 1);
                draw(data, width, height, full, x, yi + 1, fpart);
            }
            y += gradient;
        }

        let x0part = end.0.fract();
        if x0part != 0.0 {
            let total = x0part;
            let ypart = y.fract();
            let last = end.0.floor() as int;
            draw(
                data,
                width,
                height,
                full,
                last,
                y as int,
                total * (1.0 - ypart),
            );
            if ypart > 0.0 {
                draw(data, width, height, full, last, y as int + 1, total * ypart);
            }
        }
    }
    // let t = now();
    // if t - s > 10.0 {
    // // let t = std::time::SystemTime::now();
    // // if t.duration_since(s).unwrap() > std::time::Duration::from_micros(100) {
    //     log!("Too long {} - steep {}, dx {}, dy {}", t - s, steep, dx, dy);
    // }
}

pub type Point<T> = (T, T);
