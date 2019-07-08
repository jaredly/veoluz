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
pub mod render;
pub mod line_algos;
pub use render::*;
pub mod calculate;
pub use calculate::calculate;

use std::f32::consts::PI;

use ncollide2d::query::Ray;
use ncollide2d::shape::Segment;

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
            LightKind::Point { offset, origin, t0, t1 } => {
                let angle = direction * (t1 - t0) + t0;
                let mut ray = Ray::new(*origin, Vector2::new(angle.cos(), angle.sin()));
                if *offset != 0.0 {
                    ray.origin = ray.point_at(*offset)
                }
                ray
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
    pub fn translate(&mut self, by: &Vector2<line::float>) {
        match self {
            LightKind::Point { origin, .. } => {
                *origin = *origin + by;
            }
        }
    }
}

impl LightSource {
    pub fn translate(&mut self, by: &Vector2<line::float>) {
        self.kind.translate(by);
    }
}

impl Config {
    pub fn new(walls: Vec<Wall>, width: usize, height: usize) -> Self {
        Config {
            walls,
            light_formation: Default::default(),
            lights: vec![LightSource {
                kind: LightKind::Point {
                    origin: Point2::origin(),
                    // offset: 50.0,
                    offset: 0.0,
                    t0: -PI,
                    t1: PI,
                },
                brightness: 1.0,
            }],
            rendering: Rendering::new(width, height),
            transform: Transform {
                rotational_symmetry: 1,
                reflection: false,
            },
        }
    }

    // maybe tl / br?
    // pub fn bounds(&self) -> (float, float, float, float) {
    //     let width = self.rendering.width as float / self.rendering.zoom;
    //     let height = self.rendering.height as float / self.rendering.zoom;
    //     (
    //         self.rendering.center.x - width / 2.0,
    //         self.rendering.center.y - height / 2.0,
    //         width, height
    //     )
    // }

    pub fn bounds(&self) -> (Point2<float>, Point2<float>) {
        let width = self.rendering.width as float / self.rendering.zoom;
        let height = self.rendering.height as float / self.rendering.zoom;
        (
            Point2::new(
                self.rendering.center.x - width / 2.0,
                self.rendering.center.y - height / 2.0,
            ),
            Point2::new(
                self.rendering.center.x + width / 2.0,
                self.rendering.center.y + height / 2.0,
            ),
        )
    }

    pub fn transform(&self) -> (float, float, float) {
        self.rendering.transform()
    }

    pub fn inverse_transform_point(&self, point: &Point2<float>) -> Point2<float> {
        self.rendering.inverse_transform_point(point)
    }

    pub fn transform_point(&self, point: &Point2<float>) -> Point2<float> {
        self.rendering.transform_point(point)
    }

    pub fn resize_center(&mut self, width: usize, height: usize) {
        // let uw = self.rendering.width;
        // let uh = self.rendering.height;
        // let ucenter = Point2::new(uw as f32 / 2.0, uh as f32 / 2.0);
        // let center = Point2::new(width as f32 / 2.0, height as f32 / 2.0);
        // let diff = center - ucenter;
        self.rendering.width = width;
        self.rendering.height = height;
        // for light in self.lights.iter_mut() {
        //     light.kind.translate(diff);
        // }
        // for wall in self.walls.iter_mut() {
        //     wall.kind.translate(diff);
        // }
    }
}

impl Rendering {
    pub fn transform(&self) -> (float, float, float) {
        (
            self.zoom,
            self.width as float / 2.0 - self.center.x * self.zoom,
            self.height as float / 2.0 - self.center.y * self.zoom,
        )
    }

    pub fn transform_point(&self, point: &Point2<float>) -> Point2<float> {
        let (zoom, dx, dy) = self.transform();
        Point2::new(dx + point.x * zoom, dy + point.y * zoom)
    }

    pub fn inverse_transform_point(&self, point: &Point2<float>) -> Point2<float> {
        let (zoom, dx, dy) = self.transform();
        Point2::new((point.x - dx) / zoom, (point.y - dy) / zoom)
    }
}

impl Wall {
    pub fn new(kind: WallType) -> Wall {
        Wall {
            kind,
            hide: false,
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
            hide: false,
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
            hide: false,
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
            hide: false,
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
            hide: false,
            properties: Properties {
                reflect: 0.0,
                absorb: 0.0,
                roughness: 0.0,
                refraction,
            },
        }
    }
}

// #[derive(Serialize, Deserialize)]
// pub enum WorkerMsg {
//     Finished(JsValue)
// }

pub fn extra_walls(mut orig_walls: Vec<Wall>, walls: &mut Vec<Wall>, config: &Config) {
    // let mut orig_walls = config.walls.clone();
    if config.transform.reflection {
        for wall in orig_walls.clone().iter() {
            let mut wall = wall.clone();
            wall.kind.reflect_across(0.0);
            walls.push(wall.clone());
            orig_walls.push(wall);
        }
    }

    let rot = PI * 2.0 / config.transform.rotational_symmetry as f32;
    let center = Point2::origin();
    for i in 1..config.transform.rotational_symmetry {
        let angle = i as f32 * rot;
        for wall in orig_walls.iter() {
            let mut wall = wall.clone();
            wall.kind.rotate_around(&center, angle);
            walls.push(wall);
        }
    }
}

fn all_walls(config: &Config) -> Vec<Wall> {
    let mut walls = config.walls.clone();
    extra_walls(walls.clone(), &mut walls, config);
    walls
}

// pub fn move_walls(config: &Config, walls: &mut [Wall]) {
//     let width = config.rendering.width;
//     let height = config.rendering.height;
//     let to_center = Vector2::new(
//         width as float / 2.0,
//         height as float / 2.0
//     ) - config.rendering.center.coords;
//     for wall in walls.iter_mut() {
//         wall.kind.translate(&to_center);
//     }
// }

pub use line::float;

impl Config {
    pub fn all_walls(&self) -> Vec<Wall> {
        all_walls(self)
    }

    pub fn all_lights(&self) -> Vec<LightSource> {
        match self.light_formation {
            LightFormation::Single(()) => self.lights.clone(),
            LightFormation::Line(count, spacing) => {
                let base = self.lights[0].clone();
                let mut lights = self.lights.clone();
                if count < 2 {
                    return lights;
                }
                let x0 = -spacing * (count - 1) as f32 / 2.0;
                for i in 0..count.max(1) {
                    let mut light = base.clone();
                        light.translate(&Vector2::new(x0 + spacing * i as f32, 0.0));
                    lights.push( light);
                }
                lights
            },
            LightFormation::Circle(count, spacing, center) => {
                let base = self.lights[0].clone();
                let mut lights = if center { self.lights.clone() } else { self.lights[1..].to_vec() };
                let offset =  -std::f32::consts::PI / 2.0;
                let r = std::f32::consts::PI * 2.0 / count.max(2) as f32;
                for i in 0..count.max(2) {
                    let mut light = base.clone();
                    let angle = (i as f32 * r) + offset;
                    light.translate(&Vector2::new(spacing * angle.cos(), spacing * angle.sin()));
                    lights.push(light)
                }
                lights
            }
        }
    }

    pub fn main_walls(&self) -> Vec<Wall> {
        self.walls.clone()
    }

    pub fn extra_walls(&self) -> Vec<Wall> {
        let mut extras = vec![];
        extra_walls(self.walls.clone(), &mut extras, self);
        extras
    }

    // fn all_lights(&self) -> Vec<LightSource> {
    //     self.lights.clone()
    // }
}

pub struct Timer<'a> {
    name: &'a str,
    #[cfg(not(target_arch = "wasm32"))]
    initial: std::time::SystemTime,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::time_with_label(name);
            return Timer { name };
        };
        #[cfg(not(target_arch = "wasm32"))]
        Timer { name, initial: std::time::SystemTime::now() }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::time_end_with_label(self.name);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let diff = std::time::SystemTime::now().duration_since(self.initial).unwrap();
            println!("Timer {}: {}s", self.name, diff.as_secs());
        }
    }
}
