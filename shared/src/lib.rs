pub mod line;
pub mod messaging;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use rand::random;
// use wasm_bindgen::Clamped;
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

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Parabola {
    pub a: line::float,
    pub left: line::float,
    pub right: line::float,
    pub transform: nalgebra::geometry::Isometry2<line::float>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum WallType {
    Line(Segment<line::float>),
    Circle(
        ncollide2d::shape::Ball<line::float>,
        Point2<line::float>,
        line::float,
        line::float,
    ),
    Parabola(Parabola),
}

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
fn ray_parabola_collision(
    ray: &Ray<line::float>,
    parabola: &Parabola,
) -> Option<RayIntersection<line::float>> {

    macro_rules! log {
        ( $( $t:tt )* ) => {
            // web_sys::console::log_1(&format!( $( $t )* ).into());
            ();
        };
    }

    log!("Ray parabola collision");
    let ray = ray.inverse_transform_by(&parabola.transform);
    // I need the ray to u
    // x, y, dx, dy
    // a = x / dx
    // parabola.transform.rotation.deref().angle;
    // b = y - (a * dy)
    let y = ray.origin.y;

    #[inline]
    fn normal(x: f32, parabola: &Parabola, outside: bool) -> Vector2<f32> {
        let slope = parabola.a * 2.0 * x;
        let angle =
            slope.atan2(1.0) - std::f32::consts::PI / 2.0 + parabola.transform.rotation.angle();
        let angle = if outside { angle + std::f32::consts::PI } else { angle };
        Vector2::new(angle.cos(), angle.sin())
    }

    if (ray.dir.x).abs() < 0.0001 {
        log!("No dx");
        return if ray.origin.x > parabola.left && ray.origin.x < parabola.right {
            let py = parabola.a * ray.origin.x * ray.origin.x;
            if (py > ray.origin.y && ray.dir.y > 0.0) || (
                py < ray.origin.y && ray.dir.y < 0.0
            ) {

            Some(RayIntersection::new(
                (py - ray.origin.y) / ray.dir.y,
                normal(ray.origin.x, &parabola, ray.origin.y < py),
                // inside to outside
                FeatureId::Face(1),
            ))
            } else {
                None
            }
        } else {
            None
        };
    }

    let m = ray.dir.y / ray.dir.x;
    // y = mx + b
    // b = y - mx
    let b = ray.origin.y - m * ray.origin.x;
    // let b = ray.origin.y - (ray.origin.x / ray.dir.x) * ray.origin.y;

    // parabola
    // y = ax^2 + k
    // y = mx + b

    // mx + b = ax^2 + k
    // mx + b = ax^2 + k
    // b - k = ax^2 - mx

    // 0 = ax^2 - mx + k - b

    let a = parabola.a;
    let c = -b;
    let b = -m;

    let determinant = b * b - 4.0 * a * c;

    if determinant <= 0.0 {
        log!("Outside the realm");
        None
    } else {
        let rest = determinant.sqrt();

        let x0 = (-b + rest) / (a * 2.0);
        let x1 = (-b - rest) / (a * 2.0);

        let x0_valid = x0 > parabola.left && x0 < parabola.right;
        let x1_valid = x1 > parabola.left && x1 < parabola.right;
        log!(
            "a: {}, b: {}, c: {}, det: {}, x0: {}, x1; {}",
            a,
            b,
            c,
            determinant,
            x0,
            x1
        );
        log!("Transformed ray: {:?}", ray);

        if ray.origin.x < x0 {
            // left
            if ray.dir.x < 0.0 {
                log!("Left side going left");
                None
            } else if x0_valid {
                Some(RayIntersection::new(
                    (x0 - ray.origin.x) / ray.dir.x,
                    normal(x0, parabola, true),
                    // outside to inside
                    FeatureId::Face(0),
                ))
            } else if x1_valid {
                Some(RayIntersection::new(
                    (x1 - ray.origin.x) / ray.dir.x,
                    normal(x1, parabola, false),
                    // inside to outside
                    FeatureId::Face(1),
                ))
            } else {
                log!("Both intersections outside");
                None
            }
        } else if ray.origin.x < x1 {
            // middle
            if ray.dir.x < 0.0 {
                if x0_valid {
                    Some(RayIntersection::new(
                        (x0 - ray.origin.x) / ray.dir.x,
                        normal(x0, parabola, false),
                        // inside to outside
                        FeatureId::Face(1),
                    ))
                } else {
                    log!(
                        "Inside going left, outside bounds: x0: {}, rest: {}, deter: {}",
                        x0,
                        rest,
                        determinant
                    );
                    None
                }
            } else {
                if x1_valid {
                    Some(RayIntersection::new(
                        (x1 - ray.origin.x) / ray.dir.x,
                        normal(x1, parabola, false),
                        // inside to outside
                        FeatureId::Face(1),
                    ))
                } else {
                    log!("Inside going right, outside bounds");
                    None
                }
            }
        } else {
            // right
            if ray.dir.x > 0.0 {
                log!("Right going right");
                None
            } else if x1_valid {
                Some(RayIntersection::new(
                    (x1 - ray.origin.x) / ray.dir.x,
                    normal(x1, parabola, true),
                    // outside to inside
                    FeatureId::Face(0),
                ))
            } else if x0_valid {
                Some(RayIntersection::new(
                    (x0 - ray.origin.x) / ray.dir.x,
                    normal(x0, parabola, false),
                    // inside to outside
                    FeatureId::Face(1),
                ))
            } else {
                log!("Right both outside bounds");
                None
            }
        }
    }

    // ray going left or right

    // ray is on the left side
    // ray in the middle
    // ray on the right side

    // x = -b +/- sqrt(b^2 - 4ac)

    // x = (ax^2 - b) / m
    // x = ax^2 / m - b / m
    // b/m = ax^2 / m - x
}

use ncollide2d::shape::FeatureId;

#[inline]
fn ray_arc_collision(
    ray: &Ray<line::float>,
    arc: (
        &Ball<line::float>,
        &Point2<line::float>,
        line::float,
        line::float,
    ),
) -> Option<RayIntersection<line::float>> {
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
                    -normal,
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
        // sin(t) / sin(t1) = index
        // t = asin(index * sing(t1))
        let new_origin = if properties.refraction != 1.0 {
            let new_dir = refract(&ray.dir, &normal, &properties, left_side);
            match new_dir {
                Some(new_dir) => {
                    let p = ray.point_at(toi + 0.1);
                    ray.dir = Vector2::new(new_dir.cos(), new_dir.sin());
                    p
                }
                None => {
                    let p = ray.point_at(toi - 0.1);
                    let ray_dir = ray.dir.y.atan2(ray.dir.x);
                    let normal_dir = normal.y.atan2(normal.x) + PI / 2.0;
                    let ray_reflected = reflect(ray_dir, normal_dir);
                    ray.dir = Vector2::new(ray_reflected.cos(), ray_reflected.sin());
                    p
                }
            }
        } else {
            ray.point_at(toi + 0.1)
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
    let normal_dir = n + PI / 2.0;

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

use ncollide2d::shape::Segment;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Wall {
    pub kind: WallType,
    pub properties: Properties,
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

use std::f32::consts::PI;

impl WallType {
    fn toi_and_normal_with_ray(
        &self,
        ray: &Ray<line::float>,
    ) -> Option<ncollide2d::query::RayIntersection<line::float>> {
        use ncollide2d::query::ray_internal::ray::RayCast;
        match self {
            WallType::Line(wall) => {
                match wall.toi_and_normal_with_ray(
                    &nalgebra::geometry::Isometry::identity(),
                    ray,
                    true,
                ) {
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
                        intersection.feature =
                            ncollide2d::shape::FeatureId::Face(if left_side { 0 } else { 1 });
                        Some(intersection)
                    }
                }
            }
            WallType::Circle(circle, center, t0, t1) => {
                ray_arc_collision(&ray, (circle, center, *t0, *t1))
            }
            WallType::Parabola(parabola) => ray_parabola_collision(&ray, &parabola),
        }
    }

    pub fn check_handle(&self, pos: &Point2<line::float>, size: line::float) -> Option<usize> {
        let dist = size * size;
        for (i, handle) in self.handles().iter().enumerate() {
            if (handle - pos).norm_squared() < dist {
                return Some(i);
            }
        }
        None
    }

    pub fn move_handle(&mut self, id: usize, pos: &Point2<line::float>) {
        match self {
            WallType::Line(wall) => match id {
                0 => *wall = Segment::new(*pos, wall.b().clone()),
                1 => *wall = Segment::new(wall.a().clone(), *pos),
                _ => (),
            },
            WallType::Parabola(Parabola {
                a,
                left,
                right,
                transform,
            }) => match id {
                0 => transform.translation = nalgebra::Translation2::from(pos.coords),
                1 => {
                    let dist = transform.translation.vector - pos.coords;
                    *a = -1.0 / (4.0 * dist.norm_squared().sqrt());
                    transform.rotation = nalgebra::UnitComplex::from_angle(
                        dist.y.atan2(dist.x) - std::f32::consts::PI / 2.0,
                    );
                }
                _ => (),
            },
            WallType::Circle(circle, center, t0, t1) => match id {
                0 => *center = *pos,
                1 => {
                    let d = pos - *center;
                    *t0 = d.y.atan2(d.x);
                    *circle = Ball::new(d.norm_squared().sqrt());
                }
                2 => {
                    let d = pos - *center;
                    *t1 = d.y.atan2(d.x);
                    *circle = Ball::new(d.norm_squared().sqrt());
                }
                _ => (),
            },
        }
    }

    pub fn handles(&self) -> Vec<Point2<line::float>> {
        match self {
            WallType::Line(wall) => vec![wall.a().clone(), wall.b().clone()],
            WallType::Parabola(Parabola {
                a,
                left,
                right,
                transform,
            }) => vec![
                transform.translation.vector.into(),
                Point2::from(transform.translation.vector)
                    + transform
                        .rotation
                        .transform_vector(&Vector2::new(0.0, 1.0 / (*a * 4.0))),
            ], // TODO left & right
            WallType::Circle(circle, center, t0, t1) => vec![
                center.clone(),
                Point2::new(
                    center.x + t0.cos() * circle.radius(),
                    center.y + t0.sin() * circle.radius(),
                ),
                Point2::new(
                    center.x + t1.cos() * circle.radius(),
                    center.y + t1.sin() * circle.radius(),
                ),
            ],
        }
    }

    pub fn draw_handles(
        &self,
        ctx: &CanvasRenderingContext2d,
        size: f64,
        selected: Option<usize>,
    ) -> Result<(), JsValue> {
        for (i, handle) in self.handles().iter().enumerate() {
            ctx.begin_path();
            ctx.ellipse(
                handle.x as f64,
                handle.y as f64,
                size,
                size,
                0.0,
                0.0,
                PI as f64 * 2.0,
            )?;
            match selected {
                Some(s) if s == i => ctx.fill(),
                _ => ctx.stroke(),
            }
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        match self {
            WallType::Parabola(Parabola {
                a,
                left,
                right,
                transform,
            }) => {
                let count = 16;
                ctx.begin_path();
                let y0 = a * left * left;
                let p0 = transform.transform_point(&Point2::new(*left, y0));
                ctx.move_to(p0.x as f64, p0.y as f64);
                for i in 1..=count {
                    let x = (right - left) / count as f32 * i as f32 + left;
                    let y = a * x * x;
                    let p1 = transform.transform_point(&Point2::new(x, y));
                    ctx.line_to(p1.x as f64, p1.y as f64);
                }
                ctx.stroke();
                // let p0 = transform.transform_point(&Point2::new(*left, 0.0));
                // let p1 = transform.transform_point(&Point2::new(*right, 0.0));
                // ctx.begin_path();
                // ctx.move_to(p0.x as f64, p0.y as f64);
                // ctx.line_to(p1.x as f64, p1.y as f64);
                // ctx.stroke();
            }
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

pub fn find_collision(
    walls: &[Wall],
    ray: &Ray<line::float>,
) -> Option<(line::float, Properties, bool, Vector2<line::float>)> {
    let mut closest = None;

    for (i, wall) in walls.iter().enumerate() {
        match wall.kind.toi_and_normal_with_ray(&ray) {
            None => (),
            Some(intersection) => match closest {
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
            },
        }
    }

    closest
}

// #[derive(Serialize, Deserialize)]
// pub enum WorkerMsg {
//     Finished(JsValue)
// }

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
        let max_brightness = 100.0;

        for _ in 0..4 {
            match find_collision(&config.walls, &ray) {
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
                    let (new_origin, stop) =
                        bounce_ray(&mut ray, toi, properties, left_side, normal);
                    // if (new_origin.x > 10_000.0 || new_origin.y < -10_000.0) {
                    //     log!("Bad {:?} {:?} toi {}, normal {:?}", new_origin, ray, toi, normal) 
                    // }
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
