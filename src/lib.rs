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

use nalgebra as na;
use nalgebra::geometry::{Isometry2, Rotation2, Translation2};

enum BallResult {
    Inside(line::float),
    Outside(line::float, line::float),
}

#[inline]
fn ball_toi_with_ray(
    center: &Point2<line::float>,
    radius: line::float,
    ray: &Ray<line::float>,
) -> Option<BallResult> {
    let dcenter = ray.origin - *center;

    // this confuses me, because I think the radius's dir is supposed to be noramlized, e.g. of length one
    let a = ray.dir.norm_squared();
    // the vector from the circle to the start of the ray, projected onto the line of the ray
    // if this is negative, then the ray has to start within the circle in order to intersect
    // if it's positive, then the tangent of the
    let b = dcenter.dot(&ray.dir);
    // the (distance from the ray start to the circle center)^2 - radius^2
    // if this is positive, the ray starts outside of the circle. if negative, it starts inside
    let c = dcenter.norm_squared() - radius * radius;

    if c > na::zero() && b > na::zero() {
        None
    } else {
        let delta = b * b - a * c;

        if delta < na::zero() {
            // no solution
            None
        } else {
            // I thiiiink if we do + delta.sqrt() then we get the other solution?? maybe???
            let t = (-b - delta.sqrt()) / a;

            if t <= na::zero() {
                // origin inside of the ball
                Some(BallResult::Inside((-b + delta.sqrt()) / a))
            } else {
                Some(BallResult::Outside(t, (-b + delta.sqrt()) / a))
            }
        }
    }
}

use ncollide2d::query::RayIntersection;

#[inline]
fn is_between(needle: line::float, start: line::float, end: line::float) -> bool {
    if start == -std::f32::consts::PI && end == std::f32::consts::PI {
        return true;
    }
    // TODO remove these
    let needle = angle_norm(needle);
    let start = angle_norm(start);
    let end = angle_norm(end);
    if start > end {
        needle > start || needle < end
    } else {
        needle > start && needle < end
    }
}

#[inline]
fn ray_arc_collision(
    ray: &Ray<line::float>,
    arc: (
        &Ball<line::float>,
        &Point2<line::float>,
        line::float,
        line::float,
    ),
) -> Option<ncollide2d::query::RayIntersection<line::float>> {
    match ball_toi_with_ray(&arc.1, arc.0.radius(), &ray) {
        None => None,
        Some(BallResult::Inside(dist)) => {
            let pos = ray.origin + ray.dir * dist - arc.1;
            let normal = -pos.normalize();

            let place = normal.y.atan2(normal.x) + std::f32::consts::PI;
            if is_between(place, arc.2, arc.3) {
                Some(RayIntersection::new(
                    dist,
                    normal,
                    ncollide2d::shape::FeatureId::Face(0),
                ))
            } else {
                None
            }
        }
        Some(BallResult::Outside(closer, farther)) => {
            let pos = ray.origin + ray.dir * closer - arc.1;
            let normal = -pos.normalize();

            let place = normal.y.atan2(normal.x) + std::f32::consts::PI;
            if is_between(place, arc.2, arc.3) {
                Some(RayIntersection::new(
                    closer,
                    normal,
                    ncollide2d::shape::FeatureId::Face(0),
                ))
            } else {
                let pos = ray.origin + ray.dir * farther - arc.1;
                let normal = -pos.normalize();

                let place = normal.y.atan2(normal.x) + std::f32::consts::PI;
                if is_between(place, arc.2, arc.3) {
                    Some(RayIntersection::new(
                        farther,
                        normal,
                        ncollide2d::shape::FeatureId::Face(0),
                    ))
                } else {
                    None
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    _real: f64,
    _imaginary: f64,
) -> Result<(), JsValue> {
    let _timer = Timer::new("draw all");

    let cx = (width / 2) as line::float + 10.0;
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

    for i in 0..count {
        let theta = i as line::float / (count as line::float) * line::PI * 2.0;
        let r0 = 300.0;
        let r1 = 350.0;
        let td = line::PI / (count as line::float * 4.0) + i as line::float * 0.1;

        walls.push(WallType::Line(ncollide2d::shape::Segment::new(
            Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
            Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
        )));

        walls.push(WallType::Circle(
            Ball::new(200.0),
            Point2::new(
                cx + (theta + td).cos() * 200.0,
                cy + (theta + td).sin() * 200.0,
            ),
            angle_norm(theta - std::f32::consts::PI * 4.0 / 5.0),
            angle_norm(theta - std::f32::consts::PI * 3.0 / 5.0),
        ))
    }

    let mut data = zen_photon(&walls, width as usize, height as usize);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)?;

    ctx.set_stroke_style(&JsValue::from_str("green"));
    // for wall in walls.iter() {
    //     wall.draw(&ctx);
    // }

    Ok(())
}

use nalgebra::{Point2, Vector2};

fn xy(point: &Point2<line::float>) -> (line::float, line::float) {
    (point.x, point.y)
}

use ncollide2d::query::Ray;
use ncollide2d::shape::Ball;

#[inline]
fn angle_norm(angle: line::float) -> line::float {
    let reduced = angle % (std::f32::consts::PI * 2.0);
    if reduced > std::f32::consts::PI {
        reduced - std::f32::consts::PI * 2.0
    } else if reduced < -std::f32::consts::PI {
        reduced + std::f32::consts::PI * 2.0
    } else {
        reduced
    }
}

#[inline]
fn reflect(one: line::float, by: line::float) -> line::float {
    let transformed = angle_norm(angle_norm(one) - angle_norm(by));
    angle_norm((-transformed) + by)
}

fn bounce_ray(
    ray: &mut Ray<line::float>,
    toi: line::float,
    wall_index: usize,
    normal: Vector2<line::float>,
) -> (Point2<line::float>, bool) {
    let r = random::<line::float>();
    // absorb
    // if wall_index % 2 == 0 {
    //     (ray.point_at(toi), true)
    // pass through
    // } else
    if r < 0.1 {
        (ray.point_at(toi + 0.1), false)
    // reflect
    } else {
        let new_origin = ray.point_at(toi - 0.1);
        let ray_dir = ray.dir.y.atan2(ray.dir.x);
        let normal_dir = normal.y.atan2(normal.x) + 3.14159 / 2.0;

        let ray_reflected = reflect(ray_dir, normal_dir);

        // draw from ray.origin to new_origin
        ray.dir = Vector2::new(ray_reflected.cos(), ray_reflected.sin());
        (new_origin, false)
    }
}

use ncollide2d::shape::Segment;

struct Wall {
    kind: WallType,
    reflect: f32,
    transmit: f32,
    refraction: f32,
}

enum WallType {
    Line(Segment<line::float>),
    Circle(
        ncollide2d::shape::Ball<line::float>,
        Point2<line::float>,
        line::float,
        line::float,
    ),
}

impl WallType {
    fn toi_and_normal_with_ray(
        &self,
        ray: &Ray<line::float>,
    ) -> Option<ncollide2d::query::RayIntersection<line::float>> {
        use ncollide2d::query::ray_internal::ray::RayCast;
        match self {
            WallType::Line(wall) => {
                wall.toi_and_normal_with_ray(&nalgebra::geometry::Isometry::identity(), ray, true)
            }
            WallType::Circle(circle, center, t0, t1) => {
                ray_arc_collision(&ray, (circle, center, *t0, *t1))
            }
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        match self {
            WallType::Line(wall) => {
                ctx.begin_path();
                ctx.move_to(wall.a().x as f64, wall.a().y as f64);
                ctx.line_to(wall.b().x as f64, wall.b().y as f64);
                ctx.stroke();
            }
            WallType::Circle(circle, center, t0, t1) => {
                ctx.begin_path();
                ctx.ellipse(
                    center.x as f64,
                    center.y as f64,
                    circle.radius() as f64,
                    circle.radius() as f64,
                    0.0,
                    *t0 as f64,
                    *t1 as f64,
                );
                ctx.stroke();
            }
        }
    }
}

fn find_collision(
    walls: &[WallType],
    ray: &Ray<line::float>,
) -> Option<(line::float, usize, Vector2<line::float>)> {
    let mut closest = None;

    for (i, wall) in walls.iter().enumerate() {
        match wall.toi_and_normal_with_ray(&ray) {
            None => (),
            Some(intersection) => match closest {
                Some((dist, _, _)) if intersection.toi > dist => (),
                None | Some(_) => closest = Some((intersection.toi, i, intersection.normal)),
            },
        }
    }

    closest
}

fn zen_photon(walls: &[WallType], width: usize, height: usize) -> Vec<u8> {
    let _timer = Timer::new("Calculate");

    let mut brightness_data = vec![0; width * height];

    // if we don't draw at all, we're still getting only 400k/sec
    let point = Point2::new(width as line::float / 2.0, height as line::float / 2.0);

    for _ in 0..100_000 {
        let direction = random::<line::float>() * 3.14159 * 2.0;
        let mut ray =
            ncollide2d::query::Ray::new(point, Vector2::new(direction.cos(), direction.sin()));
        let max_brightness = 5.0;

        for _ in 0..30 {
            match find_collision(walls, &ray) {
                // match None {
                None => {
                    line::draw_line(
                        xy(&ray.origin),
                        xy(&ray.point_at(600.0)),
                        &mut brightness_data,
                        width,
                        height,
                        max_brightness,
                    );
                    break;
                }
                Some((toi, wall_index, normal)) => {
                    let (new_origin, stop) = bounce_ray(&mut ray, toi, wall_index, normal);
                    line::draw_line(
                        xy(&ray.origin),
                        xy(&new_origin),
                        &mut brightness_data,
                        width,
                        height,
                        max_brightness,
                    );
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
    let top = top as line::float;
    // let scale =
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            data[index] = 255;
            data[index + 1] = 255;
            data[index + 2] = 255;
            data[index + 3] = ((brightness as line::float / top).sqrt().sqrt() * 255.0) as u8;
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
