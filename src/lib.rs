mod line;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use rand::random;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

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
    let _timer = Timer::new("draw all");

    let mut walls = vec![
        // ncollide2d::shape::Segment::new(Point2::new(100.0, 100.0), Point2::new(101.0, 400.0)),
        // ncollide2d::shape::Segment::new(Point2::new(550.0, 100.0), Point2::new(551.0, 500.0)),
        // ncollide2d::shape::Segment::new(Point2::new(100.0, 100.0), Point2::new(350.0, 101.0)),
        // ncollide2d::shape::Segment::new(Point2::new(100.0, 550.0), Point2::new(500.0, 561.0)),
    ];

    let cx = (width / 2) as f32 + 10.0;
    let cy = (height / 2) as f32;

    for i in 0..10 {
        let theta = i as f32 / 10.0 * std::f32::consts::PI * 2.0;
        let r0 = 50.0;
        let r1 = 200.0;
        let td = std::f32::consts::PI / 5.0 + i as f32 * 0.1;
        walls.push(ncollide2d::shape::Segment::new(
            Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
            Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
        ))
    }

    let mut data = zen_photon(&walls, width as usize, height as usize);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)?;

    ctx.set_stroke_style(&JsValue::from_str("green"));
    for wall in walls.iter() {
        ctx.begin_path();
        ctx.move_to(wall.a().x as f64, wall.a().y as f64);
        ctx.line_to(wall.b().x as f64, wall.b().y as f64);
        ctx.stroke();
    }

    Ok(())
}

use nalgebra::{Point2, Vector2};

fn xy(point: &Point2<f32>) -> (f32, f32) {
    (point.x, point.y)
}

use ncollide2d::query::Ray;

fn bounce_ray(
    ray: &mut Ray<f32>,
    toi: f32,
    wall_index: usize,
    normal: Vector2<f32>,
) -> (Point2<f32>, bool) {
    let r = random::<f32>();
    // absorb
    if wall_index % 2 == 0 {
        (ray.point_at(toi), true)
    // pass through
    } else if r < 0.1 {
        (ray.point_at(toi + 1.0), false)
    // reflect
    } else {
        let new_origin = ray.point_at(toi - 1.0);
        let ray_dir = ray.dir.y.atan2(ray.dir.x);
        let normal_dir = normal.y.atan2(normal.x) + 3.14159 / 2.0;

        #[inline]
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

        #[inline]
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
        ray.dir = Vector2::new(ray_reflected.cos(), ray_reflected.sin());
        (new_origin, false)
    }
}

use ncollide2d::shape::Segment;

fn find_collision(walls: &[Segment<f32>], ray: &Ray<f32>) -> Option<(f32, usize, Vector2<f32>)> {
    let mut closest = None;

    use ncollide2d::query::ray_internal::ray::RayCast;

    for (i, wall) in walls.iter().enumerate() {
        match wall.toi_and_normal_with_ray(&nalgebra::geometry::Isometry::identity(), &ray, true) {
            None => (),
            Some(intersection) => match closest {
                Some((dist, _, _)) if intersection.toi > dist => (),
                None | Some(_) => closest = Some((intersection.toi, i, intersection.normal)),
            },
        }
    }

    closest
}

fn zen_photon(walls: &[Segment<f32>], width: usize, height: usize) -> Vec<u8> {
    let _timer = Timer::new("Calculate");

    let mut brightness_data = vec![0; width * height];

    log!("[[[[[[[[[[[[ Starting I guess");

    // for wall in walls.iter() {
    //     let mut line = line::XiaolinWu::<f32, i16>::new(xy(&wall.a()), xy(&wall.b()));
    //     line.draw(&mut data, width, height, 255, 0, 0);
    // }

    // optimized build, takes about 2.4 seconds to render 100k rays.
    // ok, so we're getting ~97k rays per second, whereas zenphoton is getting ~900k / sec
    // So there's at least room for a factor of 10 speedup.
    // Wow ok a ton of time spent iterating off screen to draw the final lines...
    // Now we're at 120k/sec, getting better. still room for a 7x speedup
    let point = Point2::new(width as f32 / 2.0, height as f32 / 2.0);
    let mut ray = ncollide2d::query::Ray::new(point, Vector2::new(0.0, 0.0));
    for _ in 0..30_000 {
        let direction = random::<f32>() * 3.14159 * 2.0;
        ray = ncollide2d::query::Ray::new(point, Vector2::new(direction.cos(), direction.sin()));
        let max_brightness = 255.0;

        // 30 bounces
        for _ in 0..30 {
            match find_collision(walls, &ray) {
                None => {
                    // let mut line = line::XiaolinWu::<f32, i16>::new(
                    //     xy(&ray.origin),
                    //     xy(&ray.point_at(1000.0)),
                    // );
                    // line.draw_brightness(&mut brightness_data, width, height, max_brightness);

                    // 314 -> 243 (20% savings), from going to direct style for line drawing
                    // 243 -> 197 (another 20%) going from 1000.0 for outside line to 500.0
                    // TODO dial the final line back in so it's just on the edge of the screen
                    line::draw_line(xy(&ray.origin), xy(&ray.point_at(600.0)), &mut brightness_data, width, height, max_brightness);
                    // log!("No collision");
                    break;
                }
                Some((toi, wall_index, normal)) => {
                    // let wall = &walls[wall_index];

                    let (new_origin, stop) = bounce_ray(&mut ray, toi, wall_index, normal);

                    // 7% gain by switching to direct style, w/o XiaolinWu allocation & indirection
                    line::draw_line(xy(&ray.origin), xy(&new_origin), &mut brightness_data, width, height, max_brightness);

                    // let mut line =
                    //     line::XiaolinWu::<f32, i16>::new(xy(&ray.origin), xy(&new_origin));
                    // line.draw_brightness(&mut brightness_data, width, height, max_brightness);

                    ray.origin = new_origin;

                    if stop {
                        break;
                    }
                }
            }
        }
    }

    // something like 5% of the time is here
    let _timer = Timer::new("Last bit");

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }

    let mut data = vec![0; width * height * 4];
    let top = top as f32;
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            data[index] = 255;
            data[index + 1] = 255;
            data[index + 2] = 255;
            data[index + 3] = ((brightness as f32 / top).sqrt().sqrt() * 255.0) as u8;
            // data[index + 3] = 255;
        }
    }

    data
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        web_sys::console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(self.name);
    }
}
