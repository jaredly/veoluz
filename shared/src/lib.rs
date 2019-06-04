pub mod line;
use serde::{Serialize, Deserialize};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use rand::random;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

fn rand() -> f32 {
    rand::random::<f32>()
}

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
                    ncollide2d::shape::FeatureId::Face(1),
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
                // on the inside now
                let pos = ray.origin + ray.dir * farther - arc.1;
                let normal = -pos.normalize();

                let place = normal.y.atan2(normal.x) + std::f32::consts::PI;
                if is_between(place, arc.2, arc.3) {
                    Some(RayIntersection::new(
                        farther,
                        normal,
                        ncollide2d::shape::FeatureId::Face(1),
                    ))
                } else {
                    None
                }
            }
        }
    }
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

fn bounce_ray(
    ray: &mut Ray<line::float>,
    toi: line::float,
    properties: Properties,
    left_side: bool,
    normal: Vector2<line::float>,
) -> (Point2<line::float>, bool) {
    if check(properties.absorb) {
        (ray.point_at(toi), true)
    } else if check(properties.reflect) {
        let new_origin = ray.point_at(toi - 0.1);
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
        let new_origin = ray.point_at(toi + 0.1);

        // sin(t) / sin(t1) = index
        // t = asin(index * sing(t1))
        if properties.refraction != 1.0 {
            let new_dir = refract(&ray.dir, &normal, &properties, left_side);
            ray.dir = Vector2::new(new_dir.cos(), new_dir.sin());
        }
        // TODO refraction
        (new_origin, false)
    }
}

#[inline]
fn refract(ray_dir: &Vector2<line::float>, normal: &Vector2<line::float>, properties: &Properties, left_side: bool) -> line::float {
    let ray_dir = ray_dir.y.atan2(ray_dir.x);
    let n = normal.y.atan2(normal.x);
    let normal_dir = n + PI / 2.0;

    #[inline]
    fn deg(r: f32) -> f32 {
        r * 180.0 / PI
    }

    let index = if left_side { properties.refraction } else { 1.0 / properties.refraction};
    let opposite = angle_norm(n + PI);
    let diff = ray_dir - opposite;
    let new_dir = (properties.refraction * diff.sin()).asin() + opposite;
    // log!("Refracting index: {}, ray_dir: {}, n: {}, normal_dir: {}, oppoosite: {}, diff: {}, new_dir: {}", index, deg(ray_dir), deg(n), deg(normal_dir), deg(opposite), deg(diff), deg(new_dir));
    new_dir
}

use ncollide2d::shape::Segment;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Properties {
    // percentage of incoming light that's just absorbed
    // TODO(color): this should be a triple, for each rgb component... or something?
    absorb: f32,
    // of the light that's not absorbed, how much is reflected (vs transmitted)?
    reflect: f32,
    // when reflecting, how much is scattered (vs a pure reflection)
    roughness: f32,
    // when transmitting, what's the index of refraction?

    // this is the index of refraction from *left* to *right*
    // - circle "left" is outside, "right" inside
    // - line, "left" when at the first point facing the second point.
    // when the RayIntersection has FeatureId::Face(0), then it's hitting the left side
    // Face(1) is hitting the right side
    refraction: f32,
}

pub struct Wall {
    kind: WallType,
    properties: Properties,
}

impl Wall {
    pub fn new(kind: WallType) -> Wall {
        Wall { kind, properties: Properties { reflect: 0.0, absorb: 1.0, roughness: 0.0, refraction: 1.0 }}
    }

    pub fn mirror(kind: WallType) -> Wall {
        Wall { kind, properties: Properties { reflect: 1.0, absorb: 0.0, roughness: 0.0, refraction: 1.0 }}
    }

    pub fn transparent(kind: WallType, refraction: f32) -> Wall {
        Wall { kind, properties: Properties { reflect: 0.0, absorb: 0.0, roughness: 0.0, refraction }}
    }
}

pub enum WallType {
    Line(Segment<line::float>),
    Circle(
        ncollide2d::shape::Ball<line::float>,
        Point2<line::float>,
        line::float,
        line::float,
    ),
}

#[derive(Serialize, Deserialize)]
enum SerializableWallType {
  Line(SerializableSegment),
  Circle(line::float, SerializablePoint, line::float, line::float)
}

impl From<&WallType> for SerializableWallType {
  fn from(other: &WallType) -> Self {
    match other {
      WallType::Line(segment) => SerializableWallType::Line(segment.into()),
      WallType::Circle(ball, center, t0, t1) => SerializableWallType::Circle(ball.radius(), center.into(), *t0, *t1)
    }
  }
}

impl From<&SerializableWallType> for WallType {
  fn from(other: &SerializableWallType) -> Self {
    match other {
      SerializableWallType::Line(segment) => WallType::Line((*segment).into()),
      SerializableWallType::Circle(radius, center, t0, t1) => WallType::Circle(Ball::new(*radius), (*center).into(), *t0, *t1)
    }
  }
}

// use serde::ser::{Serializer, SerializeStruct, SerializeTupleVariant};

#[derive(Serialize, Deserialize, Clone, Copy)]
struct SerializablePoint {
  x: line::float,
  y: line::float,
}

impl From<&Point2<line::float>> for SerializablePoint {
  fn from(other: &Point2<line::float>) -> Self {
    SerializablePoint {x: other.x, y: other.y}
  }
}

impl Into<Point2<line::float>> for SerializablePoint {
  fn into(self) -> Point2<line::float> {
    Point2::new(self.x, self.y)
  }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct SerializableSegment {
  a: SerializablePoint,
  b: SerializablePoint,
}

impl From<&Segment<line::float>> for SerializableSegment {
  fn from(other: &Segment<line::float>) -> Self {
    SerializableSegment {a: other.a().into(), b: other.b().into()}
  }
}

impl Into<Segment<line::float>> for SerializableSegment {
  fn into(self) -> Segment<line::float> {
    let m: Point2<line::float> = self.a.into();
    Segment::new(m, self.b.into())
  }
}

// impl Serialize for WallType {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//       match self {
//         WallType::Line(segment) => {
//           serializer.serialize_newtype_variant("WallType", 0, "Line", &SerializableSegment::from(segment))
//         }
//         WallType::Circle(ball, center, t0, t1) => {
//           let mut state = serializer.serialize_tuple_variant("WallType", 1, "Circle", 4)?;
//           state.serialize_field(&ball.radius())?;
//           state.serialize_field(&SerializablePoint::from(center))?;
//           state.serialize_field(&t0)?;
//           state.serialize_field(&t1)?;
//           state.end()
//         }
//       }
//     }
// }

use std::f32::consts::PI;

impl WallType {
    fn toi_and_normal_with_ray(
        &self,
        ray: &Ray<line::float>,
    ) -> Option<ncollide2d::query::RayIntersection<line::float>> {
        use ncollide2d::query::ray_internal::ray::RayCast;
        match self {
            WallType::Line(wall) => {
                match wall.toi_and_normal_with_ray(&nalgebra::geometry::Isometry::identity(), ray, true) {
                    None => None,
                    Some(mut intersection) => {
                        let delta = wall.b() - wall.a();
                        let wall_theta = delta.y.atan2(delta.x);
                        let normal_theta = intersection.normal.y.atan2(intersection.normal.x);
                        let left_side = if wall_theta >= 0.0 {
                            normal_theta < wall_theta && normal_theta > wall_theta - PI
                        } else {
                            normal_theta < wall_theta || normal_theta > wall_theta + PI
                        };
                        intersection.feature = ncollide2d::shape::FeatureId::Face(if left_side { 0 } else { 1 });
                        Some(intersection)
                    }
                }
            }
            WallType::Circle(circle, center, t0, t1) => {
                ray_arc_collision(&ray, (circle, center, *t0, *t1))
            }
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
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
    walls: &[Wall],
    ray: &Ray<line::float>,
) -> Option<(line::float, Properties, bool, Vector2<line::float>)> {
    let mut closest = None;

    for (i, wall) in walls.iter().enumerate() {
        match wall.kind.toi_and_normal_with_ray(&ray) {
            None => (),
            Some(intersection) => match closest {
                Some((dist, _, _, _)) if intersection.toi > dist => (),
                None | Some(_) => closest = Some((intersection.toi, wall.properties, match intersection.feature { ncollide2d::shape::FeatureId::Face(0) => true, _ => false }, intersection.normal)),
            },
        }
    }

    closest
}

pub fn zen_photon(walls: &[Wall], width: usize, height: usize) -> Vec<u8> {
    let _timer = Timer::new("Calculate");

    let mut brightness_data = vec![0; width * height];

    // if we don't draw at all, we're still getting only 400k/sec
    let point = Point2::new(width as line::float / 2.0, height as line::float / 2.0);

    for r in 0..100_000 {
        let direction = rand() * PI * 2.0;
        // let direction = (r as f32) / 180.0 * PI;
        let mut ray =
            ncollide2d::query::Ray::new(point, Vector2::new(direction.cos(), direction.sin()));
        let max_brightness = 100.0;

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
                Some((toi, properties, left_side, normal)) => {
                    let (new_origin, stop) = bounce_ray(&mut ray, toi, properties, left_side, normal);
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

