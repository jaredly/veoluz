use ncollide2d::query::{Ray, RayIntersection};
use ncollide2d::shape::{FeatureId};
use serde::{Deserialize, Serialize};
use nalgebra::{Point2, Vector2};
use crate::line;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Parabola {
    pub a: line::float,
    pub left: line::float,
    pub right: line::float,
    pub transform: nalgebra::geometry::Isometry2<line::float>,
}

impl Parabola {
    pub fn new(
        dist: line::float,
        left: line::float,
        right: line::float,
        center: Point2<line::float>,
        rotation: line::float,
    ) -> Self {
        Parabola {
            a: 1.0 / (4.0 * dist),
            left,
            right,
            transform: nalgebra::geometry::Isometry2::from_parts(
                nalgebra::Translation2::from_vector(center.coords),
                nalgebra::UnitComplex::from_angle(rotation),
            ),
        }
    }
}

#[inline]
pub fn ray_parabola_collision(
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
    let _y = ray.origin.y;

    #[inline]
    fn normal(x: f32, parabola: &Parabola, outside: bool) -> Vector2<f32> {
        let slope = parabola.a * 2.0 * x;
        let angle =
            slope.atan2(1.0) - std::f32::consts::PI / 2.0 + parabola.transform.rotation.angle();
        let angle = if outside {
            angle + std::f32::consts::PI
        } else {
            angle
        };
        Vector2::new(angle.cos(), angle.sin())
    }

    if (ray.dir.x).abs() < 0.0001 {
        log!("No dx");
        return if ray.origin.x > parabola.left && ray.origin.x < parabola.right {
            let py = parabola.a * ray.origin.x * ray.origin.x;
            if (py > ray.origin.y && ray.dir.y > 0.0) || (py < ray.origin.y && ray.dir.y < 0.0) {
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
