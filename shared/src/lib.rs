pub mod line;
pub mod messaging;
pub mod wall_type;
pub mod parabola;
pub mod arc;
use serde::{Deserialize, Serialize};
use wall_type::WallType;
use arc::angle_norm;

use std::f32::consts::PI;




use ncollide2d::query::Ray;

use ncollide2d::query::RayIntersection;
use ncollide2d::shape::FeatureId;

use nalgebra::{Point2, Vector2};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use rand::random;
// use wasm_bindgen::Clamped;

fn rand() -> f32 {
    rand::random::<f32>()
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use nalgebra as na;

pub struct LightSource {
    kind: LightKind,
    // something between 0 and 1 I think?
    brightness: line::float,
}

pub enum LightKind {
    Point {
        origin: Point2<line::float>,
        t0: line::float,
        t1: line::float,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub walls: Vec<Wall>,
    pub light_source: Point2<line::float>,
    pub width: usize,
    pub height: usize,
}

impl Config {
    pub fn new(walls: Vec<Wall>, width: usize, height: usize) -> Self {
        Config {
            walls,
            width,
            height,
            light_source: Point2::new(width as line::float / 2.0, height as line::float / 2.0),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct Properties {
    // percentage of incoming light that's just absorbed
    // TODO(color): this should be a triple, for each rgb component... or something?
    pub absorb: f32,
    // of the light that's not absorbed, how much is reflected (vs transmitted)?
    pub reflect: f32,
    // when reflecting, how much is scattered (vs a pure reflection)
    pub roughness: f32,
    // when transmitting, what's the index of refraction?

    // this is the index of refraction from *left* to *right*
    // - circle "left" is outside, "right" inside
    // - line, "left" when at the first point facing the second point.
    // when the RayIntersection has FeatureId::Face(0), then it's hitting the left side
    // Face(1) is hitting the right side
    pub refraction: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Wall {
    pub kind: WallType,
    pub properties: Properties,
}

fn xy(point: &Point2<line::float>) -> (line::float, line::float) {
    (point.x, point.y)
}

#[inline]
fn reflect(one: line::float, by: line::float) -> line::float {
    let transformed = angle_norm(angle_norm(one) - angle_norm(by));
    angle_norm((-transformed) + by)
}

#[inline]
fn check(v: f32) -> bool {
    if v == 0.0 {
        false
    } else if v == 1.0 {
        true
    } else {
        rand() < v
    }
}

pub fn bounce_ray(
    ray: &mut Ray<line::float>,
    toi: line::float,
    properties: Properties,
    left_side: bool,
    normal: Vector2<line::float>,
) -> (Point2<line::float>, bool) {
    if check(properties.absorb) {
        (ray.point_at(toi), true)
    } else if check(properties.reflect) {
        let new_origin = ray.point_at(toi);
        let normal_dir = normal.y.atan2(normal.x) + PI / 2.0;
        let ray_reflected = if check(properties.roughness) {
            normal_dir - rand() * PI
        } else {
            let ray_dir = ray.dir.y.atan2(ray.dir.x);
            reflect(ray_dir, normal_dir)
        };

        ray.dir = Vector2::new(ray_reflected.cos(), ray_reflected.sin());
        (new_origin, false)
    } else {
        // sin(t) / sin(t1) = index
        // t = asin(index * sing(t1))
        let new_origin = if properties.refraction != 1.0 {
            let new_dir = refract(&ray.dir, &normal, &properties, left_side);
            match new_dir {
                Some(new_dir) => {
                    let p = ray.point_at(toi);
                    ray.dir = Vector2::new(new_dir.cos(), new_dir.sin());
                    p
                }
                None => {
                    let p = ray.point_at(toi);
                    let ray_dir = ray.dir.y.atan2(ray.dir.x);
                    let normal_dir = normal.y.atan2(normal.x) + PI / 2.0;
                    let ray_reflected = reflect(ray_dir, normal_dir);
                    ray.dir = Vector2::new(ray_reflected.cos(), ray_reflected.sin());
                    p
                }
            }
        } else {
            ray.point_at(toi)
        };
        // TODO refraction
        (new_origin, false)
    }
}

#[inline]
fn refract(
    ray_dir: &Vector2<line::float>,
    normal: &Vector2<line::float>,
    properties: &Properties,
    left_side: bool,
) -> Option<line::float> {
    let ray_dir = ray_dir.y.atan2(ray_dir.x);
    let n = normal.y.atan2(normal.x);
    let _normal_dir = n + PI / 2.0;

    #[inline]
    fn deg(r: f32) -> f32 {
        r * 180.0 / PI
    }

    let index = if left_side {
        properties.refraction
    } else {
        1.0 / properties.refraction
    };
    let opposite = angle_norm(n + PI);
    let diff = ray_dir - opposite;
    let mid = (index * diff.sin()).asin();
    if mid.is_nan() {
        None
    } else {
        let new_dir = mid + opposite;
        Some(new_dir)
    }
}

impl Wall {
    pub fn new(kind: WallType) -> Wall {
        Wall {
            kind,
            properties: Properties {
                reflect: 0.0,
                absorb: 1.0,
                roughness: 0.0,
                refraction: 1.0,
            },
        }
    }

    pub fn rough(kind: WallType) -> Wall {
        Wall {
            kind,
            properties: Properties {
                reflect: 1.0,
                absorb: 0.0,
                roughness: 1.0,
                refraction: 1.0,
            },
        }
    }

    pub fn block(kind: WallType) -> Wall {
        Wall {
            kind,
            properties: Properties {
                reflect: 0.0,
                absorb: 1.0,
                roughness: 0.0,
                refraction: 1.0,
            },
        }
    }

    pub fn mirror(kind: WallType) -> Wall {
        Wall {
            kind,
            properties: Properties {
                reflect: 1.0,
                absorb: 0.0,
                roughness: 0.0,
                refraction: 1.0,
            },
        }
    }

    pub fn transparent(kind: WallType, refraction: f32) -> Wall {
        Wall {
            kind,
            properties: Properties {
                reflect: 0.0,
                absorb: 0.0,
                roughness: 0.0,
                refraction,
            },
        }
    }
}


pub fn find_collision(
    walls: &[Wall],
    ray: &Ray<line::float>,
) -> Option<(line::float, Properties, bool, Vector2<line::float>)> {
    let mut closest = None;

    for (_i, wall) in walls.iter().enumerate() {
        match wall.kind.toi_and_normal_with_ray(&ray) {
            None => (),
            Some(intersection) => {
                if intersection.toi.abs() > 0.01 {
                    match closest {
                        Some((dist, _, _, _)) if intersection.toi > dist => (),
                        None | Some(_) => {
                            closest = Some((
                                intersection.toi,
                                wall.properties,
                                match intersection.feature {
                                    ncollide2d::shape::FeatureId::Face(0) => true,
                                    _ => false,
                                },
                                intersection.normal,
                            ))
                        }
                    }
                }
            }
        }
    }

    closest
}

// #[derive(Serialize, Deserialize)]
// pub enum WorkerMsg {
//     Finished(JsValue)
// }

#[inline]
pub fn run_ray(
    ray: &mut Ray<line::float>,
    config: &Config,
    brightness_data: &mut [line::uint],
) -> bool {
    let max_brightness = 100.0;
    match find_collision(&config.walls, &ray) {
        None => {
            line::draw_line(
                xy(&ray.origin),
                xy(&ray.point_at(600.0)),
                brightness_data,
                config.width,
                config.height,
                max_brightness,
            );
            return true;
        }
        Some((toi, properties, left_side, normal)) => {
            let (new_origin, stop) = bounce_ray(ray, toi, properties, left_side, normal);
            // if (new_origin.x > 10_000.0 || new_origin.y < -10_000.0) {
            //     log!("Bad {:?} {:?} toi {}, normal {:?}", new_origin, ray, toi, normal)
            // }
            line::draw_line(
                xy(&ray.origin),
                xy(&new_origin),
                brightness_data,
                config.width,
                config.height,
                max_brightness,
            );
            ray.origin = new_origin;
            if stop {
                return true;
            }
        }
    };
    false
}

pub fn deterministic_calc(config: &Config) -> Vec<line::uint> {
    let _timer = Timer::new("Calculate");
    let width = config.width;
    let height = config.height;

    let mut brightness_data = vec![0; width * height];

    // if we don't draw at all, we're still getting only 400k/sec

    for r in 0..360 {
        let direction = (r as f32) / 180.0 * PI;
        let mut ray = ncollide2d::query::Ray::new(
            config.light_source,
            Vector2::new(direction.cos(), direction.sin()),
        );

        for _ in 0..30 {
            if run_ray(&mut ray, &config, &mut brightness_data) {
                break;
            }
        }
    }

    brightness_data
}

pub fn calculate(config: &Config, rays: usize) -> Vec<line::uint> {
    let _timer = Timer::new("Calculate");
    let width = config.width;
    let height = config.height;

    let mut brightness_data = vec![0; width * height];

    // if we don't draw at all, we're still getting only 400k/sec

    for _ in 0..rays {
        let direction = rand() * PI * 2.0;
        // let direction = (r as f32) / 180.0 * PI;
        let mut ray = ncollide2d::query::Ray::new(
            config.light_source,
            Vector2::new(direction.cos(), direction.sin()),
        );

        for _ in 0..30 {
            if run_ray(&mut ray, &config, &mut brightness_data) {
                break;
            }
        }
    }

    brightness_data
}

pub fn colorize(config: &Config, brightness_data: &[line::uint]) -> Vec<u8> {
    // something like 5% of the time is here
    let _timer = Timer::new("Colorize");

    let mut top = 0;
    for x in 0..config.width {
        for y in 0..config.height {
            top = top.max(brightness_data[x + y * config.width]);
        }
    }

    let mut data = vec![0; config.width * config.height * 4];
    let top = top as line::float;
    // let scale =
    for x in 0..config.width {
        for y in 0..config.height {
            let index = (x + y * config.width) * 4;
            let brightness = brightness_data[x + y * config.width];
            data[index] = 255;
            data[index + 1] = 255;
            data[index + 2] = 255;
            data[index + 3] = ((brightness as line::float / top).sqrt().sqrt() * 255.0) as u8;
        }
    }

    data
}

pub fn zen_photon(config: &Config) -> Vec<u8> {
    let brightness_data = calculate(&config, 100_000);

    colorize(&config, &brightness_data)
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
