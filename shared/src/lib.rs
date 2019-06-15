pub mod arc;
pub mod line;
pub mod messaging;
pub mod parabola;
pub mod wall_type;
use arc::angle_norm;
pub use parabola::Parabola;
use serde::{Deserialize, Serialize};
pub use wall_type::WallType;
pub mod types;
pub use types::*;

use std::f32::consts::PI;

use ncollide2d::query::Ray;

use ncollide2d::query::RayIntersection;
use ncollide2d::shape::FeatureId;

use nalgebra::{Point2, Vector2};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use rand::random;
// use wasm_bindgen::Clamped;

fn rand() -> f32 {
    rand::random::<f32>()
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!( $( $t )* ).into());
        // ()
    };
}

use nalgebra as na;

impl LightKind {
    #[inline]
    pub fn spawn(&self, direction: line::float) -> Ray<line::float> {
        match self {
            LightKind::Point { origin, t0, t1 } => {
                let angle = direction * (t1 - t0) + t0;
                Ray::new(*origin, Vector2::new(angle.cos(), angle.sin()))
            }
        }
    }

    pub fn scale(&mut self, by: usize) {
        match self {
            LightKind::Point { origin, .. } => {
                *origin = *origin * by as f32;
            }
        }
    }
    pub fn translate(&mut self, by: Vector2<line::float>) {
        match self {
            LightKind::Point { origin, .. } => {
                *origin = *origin + by;
            }
        }
    }
}

impl Config {
    pub fn new(walls: Vec<Wall>, width: usize, height: usize) -> Self {
        Config {
            walls,
            width,
            height,
            reflection: 1,
            lights: vec![LightSource {
                kind: LightKind::Point {
                    origin: Point2::new(width as line::float / 2.0, height as line::float / 2.0),
                    t0: -PI,
                    t1: PI,
                },
                brightness: 1.0,
            }],
            exposure: Default::default(),
        }
    }

    pub fn resize_center(&mut self, width: usize, height: usize) {
        let uw = self.width;
        let uh = self.height;
        let ucenter = Point2::new(uw as f32 / 2.0, uh as f32 / 2.0);
        let center = Point2::new(width as f32 / 2.0, height as f32 / 2.0);
        let diff = center - ucenter;
        self.width = width;
        self.height = height;
        for light in self.lights.iter_mut() {
            light.kind.translate(diff);
        }
        for wall in self.walls.iter_mut() {
            wall.kind.translate(diff);
        }
    }
}

fn xy(point: &Point2<line::float>, scale: u8) -> (line::float, line::float) {
    (
        point.x * scale as line::float,
        point.y * scale as line::float,
    )
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
                reflect: 1.0,
                absorb: 0.0,
                roughness: 0.0,
                refraction: 0.5,
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

    for wall in walls.iter() {
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

pub fn hit_boundary(boundaries: &[Segment<line::float>], ray: &Ray<line::float>) -> line::float {
    let mut closest = None;
    for b in boundaries {
        use ncollide2d::query::ray_internal::ray::RayCast;
        match b.toi_with_ray(&nalgebra::geometry::Isometry::identity(), &ray, true) {
            None => (),
            Some(toi) => match closest {
                Some(t) if t > toi => (),
                _ => closest = Some(toi),
            },
        }
    }
    match closest {
        None => 0.0,
        Some(t) => t,
    }
}

// #[derive(Serialize, Deserialize)]
// pub enum WorkerMsg {
//     Finished(JsValue)
// }

#[inline]
pub fn run_ray(
    ray: &mut Ray<line::float>,
    config: &Config,
    walls: &[Wall],
    boundaries: &[Segment<line::float>],
    brightness_data: &mut [line::uint],
    scale: u8,
) -> bool {
    let max_brightness = 100.0;
    match find_collision(walls, &ray) {
        None => {
            let toi = hit_boundary(boundaries, ray);
            line::draw_line(
                xy(&ray.origin, scale),
                xy(&ray.point_at(toi), scale),
                brightness_data,
                config.width * scale as usize,
                config.height * scale as usize,
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
                xy(&ray.origin, scale),
                xy(&new_origin, scale),
                brightness_data,
                config.width * scale as usize,
                config.height * scale as usize,
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

pub fn extra_walls(walls: &mut Vec<Wall>, config: &Config) {
    let mut orig_walls = config.walls.clone();
    for wall in config.walls.iter() {
        let mut wall = wall.clone();
        wall.kind.reflect_across(config.width as f32 / 2.0);
        walls.push(wall.clone());
        orig_walls.push(wall);
    }

    let rot = PI * 2.0 / config.reflection as f32;
    let center = Point2::new(config.width as f32 / 2.0, config.height as f32 / 2.0);
    for i in 1..config.reflection {
        let angle = i as f32 * rot;
        for wall in orig_walls.iter() {
            let mut wall = wall.clone();
            wall.kind.rotate_around(&center, angle);
            walls.push(wall);
        }
    }
}

pub fn all_walls(config: &Config) -> Vec<Wall> {
    let mut walls = config.walls.clone();
    extra_walls(&mut walls, config);
    walls
}

use ncollide2d::shape::Segment;
pub fn boundaries(config: &Config) -> Vec<Segment<line::float>> {
    vec![
        // left
        Segment::new(
            Point2::new(0.0, 0.0),
            Point2::new(0.0, config.height as line::float),
        ),
        // right
        Segment::new(
            Point2::new(config.width as line::float, 0.0),
            Point2::new(config.width as line::float, config.height as line::float),
        ),
        // bottom
        Segment::new(
            Point2::new(0.0, config.height as line::float),
            Point2::new(config.width as line::float, config.height as line::float),
        ),
        // top
        Segment::new(
            Point2::new(0.0, 0.0),
            Point2::new(config.width as line::float, 0.0),
        ),
    ]
}

pub fn deterministic_calc(config: &Config, scale: u8) -> Vec<line::uint> {
    let _timer = Timer::new("Calculate");
    let width = config.width;
    let height = config.height;

    let mut brightness_data = vec![0; width * height * scale as usize * scale as usize];

    let total_light: f32 = config.lights.iter().map(|l| l.brightness).sum();
    let walls = all_walls(config);
    let boundaries = boundaries(config);

    for light in config.lights.iter() {
        let amount = light.brightness / total_light;
        let rrr: f32 = 360.0 * amount * config.lights.len() as f32;
        let rays: usize = rrr as usize;
        for r in 0..rays {
            let direction = (r as f32) / rays as f32;
            let mut ray = light.kind.spawn(direction);

            for _ in 0..100 {
                if run_ray(
                    &mut ray,
                    &config,
                    &walls,
                    &boundaries,
                    &mut brightness_data,
                    scale,
                ) {
                    break;
                }
            }
        }
    }

    brightness_data
}

pub fn calculate(config: &Config, rays: usize, scale: u8) -> Vec<line::uint> {
    let _timer = Timer::new("Calculate");
    let width = config.width;
    let height = config.height;

    let mut brightness_data = vec![0; width * height * scale as usize * scale as usize];

    let total_light: f32 = { config.lights.iter().map(|l| l.brightness).sum() };
    let walls = all_walls(config);
    let boundaries = boundaries(config);

    // if we don't draw at all, we're still getting only 400k/sec

    for light in config.lights.iter() {
        let amount = light.brightness / total_light;
        let rrr: f32 = rays as f32 * amount;
        let rays = rrr as usize;
        for _ in 0..rays {
            let mut ray = light.kind.spawn(rand());

            for _ in 0..30 {
                if run_ray(
                    &mut ray,
                    &config,
                    &walls,
                    &boundaries,
                    &mut brightness_data,
                    scale,
                ) {
                    break;
                }
            }
        }
    }

    brightness_data
}

pub fn grayscale(config: &Config, brightness_data: &[line::uint], scale: u8) -> Vec<u8> {
    let _timer = Timer::new("Grayscale");
    let width = config.width * scale as usize;
    let height = config.height * scale as usize;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }
    let expose = exposer(config);

    let mut data = vec![0; width * height];
    let top = top as line::float;
    for x in 0..width {
        for y in 0..height {
            let index = x + y * width;
            let brightness = brightness_data[x + y * width];
            data[index] = expose(top, brightness) as u8;
        }
    }

    data
}

fn exposer<'a>(config: &Config) -> Box<Fn(line::float, line::uint) -> f32> {
    let min: line::float = config.exposure.min;
    let max: line::float = config.exposure.max;
    // if it's 0 - 0.75
    // we want (x * (255.0 / .75)).max(255.0)
    // if it's 0.25 - 0.75
    // we want ((x - .25).min(0.0) * 255.0 / .5).max(255.0)
    let scale: line::float = 255.0 / (max - min);
    let scaler = move |amt: line::float| ((amt - min).max(0.0) * scale).min(255.0);
    match config.exposure.curve {
        Curve::FourthRoot => {
            Box::new(move |top: line::float, brightness: line::uint| {
                scaler((brightness as line::float / top).sqrt().sqrt())
            })
            // (((brightness as line::float / top).sqrt().sqrt() - min).min(0.0) * scale).max(255.0) as u8
        }
        // FourthRoot => move |top, brightness|
        //     scaler((brightness as line::float / top).sqrt().sqrt()),
        Curve::SquareRoot => {
            Box::new(move |top: line::float, brightness: line::uint| {
                scaler((brightness as line::float / top).sqrt())
            })
            // (((brightness as line::float / top).sqrt().sqrt() - min).min(0.0) * scale).max(255.0) as u8;
            // move |top, brightness: u8|
            // scaler((brightness as line::float / top).sqrt())
        }
        Curve::Linear => Box::new(move |top, brightness| scaler(brightness as line::float / top)),
    }
}

// #[inline]
// fn expose(top: line::float, brightness: line::uint) -> u8 {
//     // ((brightness as line::float / top).sqrt().sqrt() * 255.0) as u8
//     // ((brightness as line::float / top).sqrt() * 255.0) as u8
//     // ((brightness as line::float / top).sqrt() * 500.0).min(255.0) as u8
//     // (brightness as line::float / top * 1000.0).min(255.0) as u8
//     (brightness as line::float / top * 4000.0).min(255.0) as u8
// }

pub fn colorize(config: &Config, brightness_data: &[line::uint], scale: u8) -> Vec<u8> {
    let _timer = Timer::new("Colorize");
    let width = config.width * scale as usize;
    let height = config.height * scale as usize;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }

    let front = (255.0, 255.0, 255.0);
    let back = (0.0, 50.0, 0.0);
    // let front = (131.0, 43.0, 93.0);
    // let front = (255.0, 200.0, 230.0);

    let expose = exposer(config);

    let mut data = vec![0; width * height * 4];
    let top = top as line::float;
    // let scale =
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            let exposed = expose(top, brightness) / 255.0;
            data[index] = (front.0 * exposed + back.0 * (1.0 - exposed)) as u8;
            data[index + 1] = (front.1 * exposed + back.1 * (1.0 - exposed)) as u8;
            data[index + 2] = (front.2 * exposed + back.2 * (1.0 - exposed)) as u8;
            // data[index] = 255 - exposed;
            // data[index + 1] = 255 - exposed;
            // data[index + 2] = 255 - exposed;
            data[index + 3] = 255;
        }
    }

    data
}

pub fn black_colorize(config: &Config, brightness_data: &[line::uint], scale: u8) -> Vec<u8> {
    // something like 5% of the time is here
    let _timer = Timer::new("Colorize");
    let width = config.width * scale as usize;
    let height = config.height * scale as usize;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }
    let expose = exposer(config);

    let mut data = vec![0; width * height * 4];
    let top = top as line::float;
    // let scale =
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            let bright = expose(top, brightness) as u8;
            data[index] = bright;
            data[index + 1] = bright;
            data[index + 2] = bright;
            data[index + 3] = 255;
        }
    }

    data
}

pub fn zen_photon(config: &Config) -> Vec<u8> {
    let brightness_data = calculate(&config, 100_000, 1);

    colorize(&config, &brightness_data, 1)
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::time_end_with_label(self.name);
    }
}
