mod line;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use rand::random;
use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imaginary };
    // let mut data = get_julia_set(width as usize, height as usize, c);

    let mut data = zen_photon(width as usize, height as usize);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

use nalgebra::{Point2, Vector2};
use ncollide2d::query::Ray;

fn xy(point: &Point2<f32>) -> (f32, f32) {
    (point.x, point.y)
}

fn zen_photon(width: usize, height: usize) -> Vec<u8> {
    let mut data = vec![0; width * height * 4];
    log!("[[[[[[[[[[[[ Starting I guess");

    let walls = vec![
        // ((100.0, 100.0), (100.0, 200.0)),
        ncollide2d::shape::Segment::new(Point2::new(100.0, 100.0), Point2::new(101.0, 400.0)),
        ncollide2d::shape::Segment::new(Point2::new(550.0, 100.0), Point2::new(551.0, 500.0)),
        ncollide2d::shape::Segment::new(Point2::new(100.0, 100.0), Point2::new(350.0, 101.0)),
        ncollide2d::shape::Segment::new(Point2::new(100.0, 550.0), Point2::new(500.0, 561.0)),
    ];

    // for x in 0..100 {
    //     for y in 0..10 {
    //         let index = ((y as usize) * width + x as usize) * 4;
    //         data[index] = 255;
    //         data[index + 1] = 0;
    //         data[index + 2] = 0;
    //         data[index + 3] = 255;
    //     }
    // }

    for wall in walls.iter() {
        let mut line = line::XiaolinWu::<f32, i16>::new(xy(&wall.a()), xy(&wall.b()));
        // log!("Drawwing wwall");
        line.draw(&mut data, width, height, 255, 0, 0);
        // for ((x, y), amount) in line {
        //     let index = ((y as usize) * width + x as usize) * 4;
        //     data[index] = 0;
        //     data[index + 1] = 0;
        //     data[index + 2] = 0;
        //     data[index + 3] = 255;
        //     println!("Ok {},{}", x, y);
        // }
    }

    use ncollide2d::query::ray_internal::ray::RayCast;

    for _ in 0..10 {
        let direction = random::<f32>() * 3.14159 * 2.0;
        let point = Point2::new(width as f32 / 2.0, height as f32 / 2.0);
        let mut ray =
            ncollide2d::query::Ray::new(point, Vector2::new(direction.cos(), direction.sin()));
        log!(
            "Create ray {} @ ({},{})",
            direction * 180.0 / 3.14,
            ray.origin.x,
            ray.origin.y
        );

        // 30 bounces
        for r in 0..300 {
            // log!(
            //     "Round {} ray {} @ ({},{})",
            //     r,
            //     ray.dir.y.atan2(ray.dir.x) * 180.0 / 3.1415,
            //     ray.origin.x,
            //     ray.origin.y
            // );

            let mut closest = None;

            for (i, wall) in walls.iter().enumerate() {
                match wall.toi_and_normal_with_ray(
                    &nalgebra::geometry::Isometry::identity(),
                    &ray,
                    true,
                ) {
                    None => (),
                    Some(intersection) => {
                        // log!("Collided with wall {} at {}", i, intersection.toi);
                        match closest {
                            Some((dist, _, _)) if intersection.toi > dist => (),
                            None | Some(_) => {
                                closest = Some((intersection.toi, i, intersection.normal))
                            }
                        }
                    }
                }
            }

            match closest {
                None => {
                    let mut line =
                        line::XiaolinWu::<f32, i16>::new(xy(&ray.origin), xy(&ray.point_at(1000.0)));
                    line.draw(&mut data, width, height, 255, 255, 255);
                    // log!("No collision");
                    break;
                }
                Some((toi, wall_index, normal)) => {
                    let wall = &walls[wall_index];
                    let new_origin = ray.point_at(toi - 1.0);
                    let ray_dir = ray.dir.y.atan2(ray.dir.x);
                    let normal_dir = normal.y.atan2(normal.x) + 3.14159 / 2.0;

                    fn angle_norm(angle: f32) -> f32 {
                        let reduced = angle % (3.14159 * 2.0);
                        if reduced > 3.14159 {
                            reduced - 3.14159 * 2.0
                        } else if reduced < -3.14159 {
                            reduced + 3.14159 * 2.0
                        } else {
                            reduced
                        }
                    }

                    fn reflect(one: f32, by: f32) -> f32 {
                        let transformed = angle_norm(angle_norm(one) - angle_norm(by));
                        angle_norm((-transformed) + by)
                    }

                    let ray_reflected = reflect(ray_dir, normal_dir);
                    // log!(
                    //     "Bouncing ray: {}, normal: {}, reflected: {}",
                    //     ray_dir * 180.0 / 3.14,
                    //     normal_dir * 180.0 / 3.14,
                    //     ray_reflected * 180.0 / 3.14
                    // );
                    // log!("Ray from {} bounce at {}", ray.origin, new_origin);

                    // draw from ray.origin to new_origin
                    let mut line =
                        line::XiaolinWu::<f32, i16>::new(xy(&ray.origin), xy(&new_origin));
                    line.draw(&mut data, width, height, 255, 255, 255);

                    ray.origin = new_origin;
                    ray.dir = Vector2::new(ray_reflected.cos(), ray_reflected.sin());
                }
            }
        }
    }

    data
}

fn get_julia_set(width: usize, height: usize, c: Complex) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * 4);

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
