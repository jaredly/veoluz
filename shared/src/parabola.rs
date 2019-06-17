use crate::line;
use nalgebra::{Point2, Vector2};
use ncollide2d::query::{Ray, RayIntersection};
use ncollide2d::shape::FeatureId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

// Ok, finding the closest distance of a point to a parabola, it's wild.
//
// ```
// y = mx^2 # the parabola
// (a, b) # the point
// D = (mx^2 - b)^2 + (x - a)^2
// # wolfram alpha gives me this as the derivative
// D' = -2 a + 2 x - 4 b m x + 4 m^2 x^3
// # Real root found by wolfram alpha
// x = (sqrt(11664 a^2 m^8 + 864 m^6 (1 - 2 b m)^3) + 108 a m^4)^(1/3)/(6 2^(1/3) m^2) - (2^(1/3) (1 - 2 b m))/(sqrt(11664 a^2 m^8 + 864 m^6 (1 - 2 b m)^3) + 108 a m^4)^(1/3)
//
// aa = (sqrt(11664 a^2 m^8 + 864 m^6 (1 - 2 b m)^3) + 108 a m^4)^(1/3)
// s2 = 2^(1/3)
// x = aa/(6 s2 m^2) - (s2 (1 - 2 b m))/aa
//
// x = ((11664.0 * a * a * m.powi(8) + 864 * m.powi(6) * (1.0 - 2.0 * b * m).powi(3)).sqrt() + 108.0 * a * m.powi(4)).powf(1.0 / 3. 0)/(6 * (2.0).powf(1.0/3.0) * m.powi(2)) - ((2.0).powf(1.0 /3.0) * (1.0 - 2.0 * b * m))/((11664.0 a * a m.powi(8) + 864 * m.powi(6) * (1.0 - 2.0 * b * m).powi(3)).sqrt() + 108.0 * a * m.powi(4)).powf(1.0/3.0)
//
// dd = (1.0 - 2.0 * b * m)
// aa = (11664.0 * a * a * m.powi(8) + 864.0 * m.powi(6) * dd.powi(3)).sqrt()
// bb = (aa + 108.0 * a * m.powi(4)).powf(1.0/3.0)
// cc = (2.0).powf(1.0/3.0)
// x = bb/(6 * cc * m.powi(2)) - (cc * dd)/bb
// ```
//

#[inline]
pub fn point_dist(p: &Point2<line::float>, parabola: &Parabola) -> line::float {
    let p = parabola.transform.inverse_transform_point(p);

    let a = p.x as f64;
    let b = p.y as f64;
    let m = parabola.a as f64;

    let dd = 1.0 - 2.0 * b * m;
    let aa = (11664.0 * a * a * m.powi(8) + 864.0 * m.powi(6) * dd.powi(3)).sqrt();
    let bb = (aa + 108.0 * a * m.powi(4)).powf(1.0 / 3.0);
    let cc = (2.0 as f64).powf(1.0 / 3.0);
    let x = bb / (6.0 * cc * m.powi(2)) - (cc * dd) / bb;

    if x < parabola.left as f64 {
        return (Point2::new(parabola.left, parabola.left.powi(2) * parabola.a) - p)
            .norm_squared()
            .sqrt() as line::float;
    }
    if x > parabola.right as f64 {
        return (Point2::new(parabola.right, parabola.right.powi(2) * parabola.a) - p)
            .norm_squared()
            .sqrt() as line::float;
    }

    let y = m * x * x;
    (Point2::new(a, b) - Point2::new(x, y))
        .norm_squared()
        .sqrt() as line::float
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
