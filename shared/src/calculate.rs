use crate::arc::angle_norm;
use crate::line;
use crate::types::*;
use crate::Timer;

use crate::line::float;

use nalgebra::{Point2, Vector2};
use ncollide2d::query::Ray;
use ncollide2d::shape::Segment;

use std::f32::consts::PI;

fn rand() -> f32 {
    rand::random::<f32>()
}

fn xy(
    point: &Point2<line::float>,
    (zoom, dx, dy): (float, float, float),
) -> (line::float, line::float) {
    (point.x * zoom + dx, point.y * zoom + dy)
}

#[inline]
fn reflect(one: line::float, by: line::float) -> line::float {
    let transformed = angle_norm(angle_norm(one) - angle_norm(by));
    angle_norm((-transformed) + by)
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!( $( $t )* ).into());
        // ()
    };
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

#[inline]
pub fn run_ray(
    ray: &mut Ray<line::float>,
    config: &Config,
    walls: &[Wall],
    boundaries: &[Segment<line::float>],
    brightness_data: &mut [line::uint],
    transform: (f32, f32, f32),
) -> bool {
    let max_brightness = 100.0;
    match find_collision(walls, &ray) {
        None => {
            let toi = hit_boundary(boundaries, ray);
            line::draw_line(
                xy(&ray.origin, transform),
                xy(&ray.point_at(toi), transform),
                brightness_data,
                config.rendering.width,
                config.rendering.height,
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
                xy(&ray.origin, transform),
                xy(&new_origin, transform),
                brightness_data,
                config.rendering.width,
                config.rendering.height,
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

pub fn boundaries(config: &Config) -> Vec<Segment<line::float>> {
    let (tl, br) = config.bounds();
    let w = br.x - tl.x;
    let h = br.y - tl.y;
    vec![
        // left
        Segment::new(tl, tl + Vector2::new(0.0, h)),
        // right
        Segment::new(tl + Vector2::new(w, 0.0), br),
        // bottom
        Segment::new(tl + Vector2::new(0.0, h), br),
        // top
        Segment::new(tl, tl + Vector2::new(w, 0.0)),
    ]
}

pub fn deterministic_calc(config: &Config) -> Vec<line::uint> {
    let _timer = Timer::new("Calculate");
    let width = config.rendering.width;
    let height = config.rendering.height;

    let mut brightness_data = vec![0; width * height];

    let total_light: f32 = config.lights.iter().map(|l| l.brightness).sum();
    let walls = config.all_walls();
    let boundaries = boundaries(config);

    let transform = config.transform();

    for light in config.all_lights().iter() {
        let amount = light.brightness / total_light;
        let rrr: f32 = 360.0 * amount * config.lights.len() as f32;
        let rays: usize = rrr as usize;
        for r in 0..rays {
            let direction = (r as f32) / rays as f32;
            let mut ray = light.kind.spawn(direction);

            for _ in 0..30 {
                if run_ray(
                    &mut ray,
                    &config,
                    &walls,
                    &boundaries,
                    &mut brightness_data,
                    transform,
                ) {
                    break;
                }
            }
        }
    }

    brightness_data
}

pub fn calculate(config: &Config, rays: usize) -> Vec<line::uint> {
    // let _timer = Timer::new("Calculate");
    let width = config.rendering.width;
    let height = config.rendering.height;

    let mut brightness_data = vec![0; width * height];

    let total_light: f32 = { config.lights.iter().map(|l| l.brightness).sum() };
    let walls = config.all_walls().into_iter().filter(|w| !w.hide).collect::<Vec<Wall>>();
    let boundaries = boundaries(config);

    // if we don't draw at all, we're still getting only 400k/sec

    let transform = config.transform();

    for light in config.all_lights().iter() {
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
                    transform,
                ) {
                    break;
                }
            }
        }
    }

    brightness_data
}

use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f64;
}

#[cfg(target_arch = "wasm32")]
pub fn timed(config: &Config, rays: usize, limit: f64) -> (Vec<line::uint>, usize) {
    let width = config.rendering.width;
    let height = config.rendering.height;

    let mut brightness_data = vec![0; width * height];

    let total_light: f32 = { config.lights.iter().map(|l| l.brightness).sum() };
    let walls = config.all_walls().into_iter().filter(|w| !w.hide).collect::<Vec<Wall>>();
    let boundaries = boundaries(config);

    // if we don't draw at all, we're still getting only 400k/sec

    let transform = config.transform();

    // TODO support multiple lights
    let light = &config.all_lights()[0];

    // for light in config.all_lights().iter() {
    let amount = light.brightness / total_light;
    let rrr: f32 = rays as f32 * amount;
    let rays = rrr as usize;

    let start = now();
    let mut rendered = rays;

    for r in 0..rays {
        let mut ray = light.kind.spawn(rand());

        for _ in 0..30 {
            if run_ray(
                &mut ray,
                &config,
                &walls,
                &boundaries,
                &mut brightness_data,
                transform,
            ) {
                break;
            }
        }

        if r % 100 == 0 && now() - start > limit {
            // log!("{} rays rendered", r);
            rendered = r;
            break;
        }
    }
    // log!("Time took: {} vs limit {}", now() - start, limit);
    // }

    (brightness_data, rendered)
}
