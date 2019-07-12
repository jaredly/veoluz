use crate::line;
use crate::parabola::{ray_parabola_collision, Parabola};
use serde::{Deserialize, Serialize};

use ncollide2d::query::Ray;
use ncollide2d::shape::Ball;

use line::float;
use ncollide2d::query::RayIntersection;
use ncollide2d::shape::FeatureId;
use ncollide2d::shape::Segment;
use std::f32::consts::PI;

use nalgebra::{Point2, Vector2};

// Ideas for other wall types:
// - a "portal" that has two straight lines, and transports from one to the other

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

use crate::types::LerpEq;

impl crate::types::LerpEq for Segment<line::float> {
    fn lerp(&self, other: &Self, amount: f32) -> Self {
        Segment::new(
            self.a().lerp(other.a(), amount),
            self.b().lerp(other.b(), amount),
        )
    }
}

impl crate::types::Lerp for WallType {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        match (self, other) {
            (WallType::Line(line1), WallType::Line(line2)) => {
                WallType::Line(line1.lerp(line2, amount))
            }
            (WallType::Circle(ball, pos, t0, t1), WallType::Circle(ball2, pos2, t02, t12)) => {
                WallType::Circle(
                    Ball::new(ball.radius().lerp(&ball2.radius(), amount)),
                    pos.lerp(&pos2, amount),
                    t0.lerp(&t12, amount),
                    t1.lerp(&t02, amount),
                )
            }
            (WallType::Parabola(p1), WallType::Parabola(p2)) => {
                WallType::Parabola(p1.lerp(p2, amount))
            }
            _ => panic!("Cannot lerp between wall types"),
        }
    }
}

impl WallType {
    pub fn rand_circle(width: usize, height: usize) -> WallType {
        WallType::Circle(
            Ball::new(rand::random::<f32>() * 50.0 + 50.0),
            Point2::new(
                rand::random::<f32>() * width as f32,
                rand::random::<f32>() * height as f32,
            ),
            -PI,
            PI,
        )
    }

    pub fn basic_circle(width: usize, height: usize) -> WallType {
        WallType::Circle(Ball::new(50.0), Point2::new(0.0, 200.0), -PI, PI)
    }

    pub fn line(p1: Point2<line::float>, p2: Point2<line::float>) -> Self {
        WallType::Line(Segment::new(p1, p2))
    }

    pub fn circle(
        center: Point2<line::float>,
        radius: line::float,
        t0: line::float,
        t1: line::float,
    ) -> Self {
        WallType::Circle(Ball::new(radius), center, t0, t1)
    }

    pub fn parabola(
        center: Point2<float>,
        focus_offset: Vector2<float>,
        left: float,
        right: float,
    ) -> Self {
        WallType::Parabola(Parabola {
            a: 1.0 / (4.0 * focus_offset.norm_squared().sqrt()),
            left,
            right,
            transform: nalgebra::Isometry2::from_parts(
                nalgebra::Translation2::from(center.coords),
                nalgebra::UnitComplex::from_angle(focus_offset.y.atan2(focus_offset.x)),
            ),
        })
    }

    pub fn rand_line(width: usize, height: usize) -> WallType {
        let c = Point2::new(
            rand::random::<f32>() * (width as f32 - 200.0) + 100.0,
            rand::random::<f32>() * (height as f32 - 200.0) + 100.0,
        );
        let r = rand::random::<f32>() * std::f32::consts::PI;
        let len = rand::random::<f32>() * 70.0 + 30.0;
        let off = Vector2::new(r.cos() * len, r.sin() * len);
        WallType::Line(Segment::new(c + off, c - off))
    }

    pub fn basic_line(width: usize, height: usize) -> WallType {
        let c = Point2::new(200.0, 0.0);
        let off = Vector2::new(50.0, 50.0);
        WallType::Line(Segment::new(c + off, c - off))
    }

    pub fn basic_parabola(width: usize, height: usize) -> WallType {
        let c = Vector2::new(-200.0, 0.0);
        WallType::Parabola(Parabola {
            a: 1.0 / (4.0 * 30.0),
            left: -60.0,
            right: 60.0,
            transform: nalgebra::Isometry2::from_parts(
                nalgebra::Translation2::from(c),
                nalgebra::UnitComplex::from_angle(0.0),
            ),
        })
    }

    pub fn rand_parabola(width: usize, height: usize) -> WallType {
        let c = Vector2::new(
            rand::random::<f32>() * (width as f32 - 200.0) + 100.0,
            rand::random::<f32>() * (height as f32 - 200.0) + 100.0,
        );
        let r = rand::random::<f32>() * std::f32::consts::PI;
        let a = rand::random::<f32>() * 100.0 + 5.0;
        WallType::Parabola(Parabola {
            a: 1.0 / (4.0 * a),
            left: -(rand::random::<f32>() * 50.0 + 10.0),
            right: (rand::random::<f32>() * 50.0 + 10.0),
            transform: nalgebra::Isometry2::from_parts(
                nalgebra::Translation2::from(c),
                nalgebra::UnitComplex::from_angle(r),
            ),
        })
    }

    pub fn rand_all(width: usize, height: usize) -> Vec<WallType> {
        vec![
            WallType::rand_circle(width, height),
            WallType::rand_line(width, height),
            WallType::rand_parabola(width, height),
        ]
    }

    pub fn translate(&mut self, by: &Vector2<line::float>) {
        match self {
            WallType::Line(wall) => *wall = Segment::new(wall.a() + by, wall.b() + by),
            WallType::Circle(ball, center, _, _) => {
                *center = *center + by;
            }
            WallType::Parabola(parabola) => {
                parabola.transform.translation.vector += by;
            }
        }
    }

    pub fn scale(&mut self, by: f32) {
        let by = by.max(0.001);
        match self {
            WallType::Line(wall) => *wall = Segment::new(wall.a() * by, wall.b() * by),
            WallType::Circle(ball, center, _, _) => {
                *ball = Ball::new(ball.radius() * by);
                *center = *center * by;
            }
            WallType::Parabola(parabola) => {
                parabola.transform.translation.vector *= by;
                if by == 0.0 {
                    parabola.a = 1.0;
                } else {
                    parabola.a /= by;
                }
                parabola.left *= by;
                parabola.right *= by;
            }
        }
    }

    pub fn reflect_across(&mut self, x: line::float) {
        match self {
            WallType::Line(wall) => {
                let mut a = wall.a().clone();
                let mut b = wall.b().clone();
                a.x -= (a.x - x) * 2.0;
                b.x -= (b.x - x) * 2.0;
                *wall = Segment::new(b, a)
            }
            WallType::Circle(_ball, center, t0, t1) => {
                center.x -= (center.x - x) * 2.0;
                let t1n = crate::arc::angle_norm(-(*t0 + PI / 2.0) - PI / 2.0);
                let t0n = crate::arc::angle_norm(-(*t1 + PI / 2.0) - PI / 2.0);
                *t0 = t0n;
                *t1 = t1n;
            }
            WallType::Parabola(parabola) => {
                parabola.transform.translation.vector.x -=
                    (parabola.transform.translation.vector.x - x) * 2.0;
                let mut angle = parabola.transform.rotation.angle();
                // angle = crate::arc::angle_norm(-(angle + PI/2.0) - PI / 2.0);
                angle = crate::arc::angle_norm(-angle);
                // left = -20
                // right = 10
                // left = -10
                // right = 20
                let (l, r) = (parabola.left, parabola.right);
                parabola.left = -r;
                parabola.right = -l;
                parabola.transform.rotation = nalgebra::UnitComplex::from_angle(angle);
            }
        }
    }

    pub fn rotate_around(&mut self, center: &Point2<line::float>, angle: line::float) {
        let base = self.point_base();
        let diff = base - center;
        let current_angle = diff.y.atan2(diff.x);
        let dist = diff.norm_squared().sqrt();
        let new_angle = current_angle + angle;
        let new_base = Point2::new(
            center.x + new_angle.cos() * dist,
            center.y + new_angle.sin() * dist,
        );
        match self {
            WallType::Line(wall) => {
                let diff = wall.b() - center;
                let current_angle = diff.y.atan2(diff.x);
                let dist = diff.norm_squared().sqrt();
                let new_angle = current_angle + angle;
                let new_b = Point2::new(
                    center.x + new_angle.cos() * dist,
                    center.y + new_angle.sin() * dist,
                );

                *wall = Segment::new(new_base, new_b);
            }
            WallType::Circle(ball, center, t0, t1) => {
                *center = new_base;
                if *t0 != -PI || *t1 != PI {
                    *t0 += angle;
                    *t1 += angle;
                }
            }
            WallType::Parabola(parabola) => {
                parabola.transform.translation = nalgebra::Translation2::from(new_base.coords);
                parabola.transform.rotation =
                    nalgebra::UnitComplex::from_angle(parabola.transform.rotation.angle() + angle);
            }
        }
    }

    pub fn point_dist(&self, point: &Point2<line::float>) -> line::float {
        match self {
            WallType::Line(wall) => {
                // y = mx + b
                // m = wall.dy / wall.dx
                // b = wall.y0 - m * wall.x0
                //
                // y = nx + c
                //
                // mx + b = nx + c
                // x = (c - b) / (m - n)
                let wd = wall.b() - wall.a();
                if wd.x == 0.0 {
                    let (top, bottom) = if wall.a().y < wall.b().y {
                        (wall.a(), wall.b())
                    } else {
                        (wall.b(), wall.a())
                    };
                    if point.y >= top.y && point.y <= bottom.y {
                        return (wall.a().x - point.x).abs();
                    }
                    if point.y < top.y {
                        return (top - point).norm_squared().sqrt();
                    } else {
                        return (bottom - point).norm_squared().sqrt();
                    }
                }

                let (left, right) = if wall.a().x < wall.b().x {
                    (wall.a(), wall.b())
                } else {
                    (wall.b(), wall.a())
                };
                let wd = right - left;
                if wd.y == 0.0 {
                    if point.x >= left.x && point.x <= right.x {
                        return (wall.a().y - point.y).abs();
                    }
                    if point.x < left.x {
                        return (left - point).norm_squared().sqrt();
                    } else {
                        return (right - point).norm_squared().sqrt();
                    }
                }
                let m = wd.y / wd.x;
                // b = y - mx
                let b = left.y - m * left.x;
                // n = perpendicular line
                let n = 1.0 / m;
                // b = y - mx
                let c = point.y - n * point.x;
                // y = mx + b
                // (y - b) / m = x

                // y = mx + b
                // y = nx + c
                // mx + b = nx + c
                // (m - n)x = c - b
                // x = (c - b)/(m - n)

                let x = (c - b) / (m - n);
                let y = m * x + b;
                // let x0 = wall.a().x.min(wall.b().x);
                // let x1 = wall.a().x.max(wall.b().x);
                // (point - Point2::new(x, y))
                //     .norm_squared()
                //     .sqrt()
                //     .min((point - left).norm_squared().sqrt())
                //     .min((point - right).norm_squared().sqrt())

                if left.x <= x && x <= right.x {
                    (point - Point2::new(x, y)).norm_squared().sqrt()
                } else if x < left.x {
                    (point - left).norm_squared().sqrt()
                } else {
                    (point - right).norm_squared().sqrt()
                }
            }
            WallType::Circle(circle, center, t0, t1) => {
                crate::arc::point_dist(point, center, circle.radius(), *t0, *t1)
            }
            WallType::Parabola(parabola) => crate::parabola::point_dist(point, parabola),
        }
    }

    pub fn point_base(&self) -> Point2<line::float> {
        match self {
            WallType::Line(wall) => wall.a().clone(),
            WallType::Circle(_, center, _, _) => center.clone(),
            WallType::Parabola(parabola) => parabola.transform.translation.vector.into(),
        }
    }

    pub fn set_point_base(&mut self, point: Point2<line::float>) {
        match self {
            WallType::Line(wall) => *wall = Segment::new(point, point + (wall.b() - wall.a())),
            WallType::Circle(_, center, _, _) => *center = point,
            WallType::Parabola(parabola) => {
                parabola.transform.translation = nalgebra::Translation2::from(point.coords);
            }
        }
    }

    pub fn toi_and_normal_with_ray(
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
                crate::arc::ray_arc_collision(&ray, (circle, center, *t0, *t1))
            }
            WallType::Parabola(parabola) => ray_parabola_collision(&ray, &parabola),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum HandleStyle {
    Circle,
    Resize,
    Rotate,
}

// Handles things
impl WallType {
    pub fn check_handle(
        &self,
        pos: &Point2<line::float>,
        size: line::float,
        selected: bool,
    ) -> Option<usize> {
        let dist = size * size;
        for (i, (handle, kind)) in self.all_handles().iter().enumerate() {
            if !selected && *kind != HandleStyle::Circle {
                continue;
            }
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
                // Resize
                2 => {
                    let center = (wall.a().coords + wall.b().coords) / 2.0;
                    let dist = (pos - center).coords.norm_squared().sqrt();
                    let adiff = wall.a() - center;
                    let angle = adiff.y.atan2(adiff.x);
                    let p0 =
                        Point2::new(center.x + angle.cos() * dist, center.y + angle.sin() * dist);
                    let p1 =
                        Point2::new(center.x - angle.cos() * dist, center.y - angle.sin() * dist);
                    *wall = Segment::new(p0, p1);
                }
                // Rotate
                3 => {
                    let center = (wall.a().coords + wall.b().coords) / 2.0;
                    let dist = (wall.a() - center).coords.norm_squared().sqrt();
                    let adiff = pos - center;
                    let angle = adiff.y.atan2(adiff.x) + std::f32::consts::PI / 2.0;
                    let p0 =
                        Point2::new(center.x + angle.cos() * dist, center.y + angle.sin() * dist);
                    let p1 =
                        Point2::new(center.x - angle.cos() * dist, center.y - angle.sin() * dist);
                    *wall = Segment::new(p0, p1);
                }
                _ => (),
            },
            WallType::Parabola(Parabola {
                a,
                left,
                right,
                transform,
            }) => match id {
                4 => {
                    let pos = transform.inverse_transform_point(pos);
                    let det = 4.0 * pos.y.abs();
                    if det != 0.0 {
                        *a = -1.0 / det;
                    }
                }
                1 => {
                    let pos = transform.inverse_transform_point(pos);
                    *left = pos.x;
                    if *right < *left {
                        *right = *left + 10.0;
                    }
                }
                2 => {
                    let pos = transform.inverse_transform_point(pos);
                    *right = pos.x;
                    if *left > *right {
                        *left = *right - 10.0;
                    }
                }
                3 => {
                    let dist = transform.translation.vector - pos.coords;
                    transform.rotation = nalgebra::UnitComplex::from_angle(
                        dist.y.atan2(dist.x) - PI / 2.0 + PI / 6.0,
                    );
                }
                0 => {
                    let pos = transform.inverse_transform_point(pos);
                    let dist = pos.coords.norm_squared().sqrt();
                    let det = 4.0 * dist;
                    if det != 0.0 {
                        let new_a = -1.0 / det;
                        let scale = new_a / *a;
                        *a *= scale;
                        *left /= scale;
                        *right /= scale;
                    }
                }
                _ => (),
            },
            WallType::Circle(circle, center, t0, t1) => match id {
                3 => {
                    let d = pos - *center;
                    *t0 = d.y.atan2(d.x);
                }
                1 => {
                    let d = pos - *center;
                    *t1 = d.y.atan2(d.x);
                }
                2 => {
                    let d = *t1 - *t0;
                    let a = *t0 + d / 3.0;
                    let d = pos - *center;
                    let new_a = d.y.atan2(d.x);
                    *t0 += new_a - a;
                    *t1 += new_a - a;
                }
                0 => {
                    let dist = (pos - *center).norm_squared().sqrt();
                    *circle = Ball::new(dist.max(0.0));
                }
                _ => (),
            },
        }
    }

    pub fn all_handles(&self) -> Vec<(Point2<line::float>, HandleStyle)> {
        match self {
            WallType::Line(wall) => {
                let center = (wall.a().coords + wall.b().coords) / 2.0;
                vec![
                    (wall.a().clone(), HandleStyle::Circle),
                    (wall.b().clone(), HandleStyle::Circle),
                    (
                        rotate_around(wall.a(), center, std::f32::consts::PI / 2.0),
                        HandleStyle::Resize,
                    ),
                    (
                        rotate_around(wall.a(), center, std::f32::consts::PI * 3.0 / 2.0),
                        HandleStyle::Rotate,
                    ),
                ]
            }
            WallType::Parabola(Parabola {
                a,
                left,
                right,
                transform,
            }) => vec![
                // transform.translation.vector.into(),
                (
                    Point2::from(transform.translation.vector)
                        + transform.rotation.transform_vector(&{
                            let dist = 1.0 / (*a * 4.0);
                            let angle = PI / 6.0 + PI / 2.0;
                            Vector2::new(angle.cos() * dist, angle.sin() * dist)
                        }),
                    HandleStyle::Resize,
                ),
                (
                    transform.transform_point(&Point2::new(*left, 0.0)),
                    HandleStyle::Circle,
                ),
                (
                    transform.transform_point(&Point2::new(*right, 0.0)),
                    HandleStyle::Circle,
                ),
                (
                    Point2::from(transform.translation.vector)
                        + transform.rotation.transform_vector(&{
                            // Vector2::new(0.0, -1.0 / (*a * 4.0))
                            let dist = 1.0 / (*a * 4.0);
                            let angle = -PI / 6.0 + PI / 2.0;
                            Vector2::new(angle.cos() * dist, angle.sin() * dist)
                        }),
                    HandleStyle::Rotate,
                ),
                (
                    Point2::from(transform.translation.vector)
                        + transform
                            .rotation
                            .transform_vector(&Vector2::new(0.0, 1.0 / (*a * 4.0))),
                    HandleStyle::Circle,
                ),
            ], // TODO left & right
            WallType::Circle(circle, center, t0, t1) => vec![
                // center.clone(),
                (
                    {
                        let d = t1 - t0;
                        // let a = (t0 + t1) / 2.0;
                        let a = t0 + d * 2.0 / 3.0;
                        Point2::new(
                            center.x + a.cos() * circle.radius(),
                            center.y + a.sin() * circle.radius(),
                        )
                    },
                    HandleStyle::Resize,
                ),
                (
                    Point2::new(
                        center.x + t1.cos() * circle.radius(),
                        center.y + t1.sin() * circle.radius(),
                    ),
                    HandleStyle::Circle,
                ),
                (
                    {
                        let d = t1 - t0;
                        // let a = (t0 + t1) / 2.0;
                        let a = t0 + d / 3.0;
                        Point2::new(
                            center.x + a.cos() * circle.radius(),
                            center.y + a.sin() * circle.radius(),
                        )
                    },
                    HandleStyle::Rotate,
                ),
                (
                    Point2::new(
                        center.x + t0.cos() * circle.radius(),
                        center.y + t0.sin() * circle.radius(),
                    ),
                    HandleStyle::Circle,
                ),
            ],
        }
    }
}

pub fn rotate_around(p1: &Point2<float>, center: Vector2<float>, angle: float) -> Point2<float> {
    let diff = p1.coords - center;
    let angle = diff.y.atan2(diff.x) + angle;
    let size = diff.norm_squared().sqrt();
    Point2::new(center.x + angle.cos() * size, center.y + angle.sin() * size)
}

// pub fn angle_from
