use crate::line::float;
use crate::wall_type::WallType;
use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Wall {
    pub kind: WallType,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LightSource {
    pub kind: LightKind,
    // something between 0 and 1 I think?
    pub brightness: float,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum LightKind {
    Point {
        origin: Point2<float>,
        t0: float,
        t1: float,
    },
}

mod v0 {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq)]
    pub struct Config {
        pub walls: Vec<Wall>,
        pub light_source: Point2<float>,
        pub reflection: u8,
        pub width: usize,
        pub height: usize,
    }
}

pub mod v1 {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq)]
    pub struct Config {
        pub walls: Vec<Wall>,
        pub lights: Vec<LightSource>,
        pub reflection: u8,
        pub width: usize,
        pub height: usize,
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Curve {
    FourthRoot,
    SquareRoot,
    Linear,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Exposure {
    pub curve: Curve,
    pub min: float,
    pub max: float,
}

impl Default for Exposure {
    fn default() -> Self {
        Exposure {
            curve: Curve::FourthRoot,
            min: 0.0,
            max: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub walls: Vec<Wall>,
    pub lights: Vec<LightSource>,
    pub reflection: u8,
    pub width: usize,
    pub height: usize,
    pub exposure: Exposure,
}

pub fn from_v1(
    v1::Config {
        walls,
        lights,
        reflection,
        width,
        height,
    }: v1::Config,
) -> Config {
    Config {
        walls,
        lights,
        reflection,
        width,
        height,
        exposure: Exposure {
            curve: Curve::FourthRoot,
            min: 0.0,
            max: 1.0,
        },
    }
}

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// pub struct Coloration {
//     pub exposure: Exposure,
//     pub background_color: (u8, u8, u8),
//     pub highlight_color: (u8, u8, u8),
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// pub struct Rendering {
//   pub coloration: Coloration,
//   pub width: usize,
//   pub height: usize,
//   pub zoom: usize,
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// pub enum Entity {
//     Wall(Wall),
//     Light(LightSource)
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// pub enum Transform {
//     Reflection { angle: float, center: Point2<float> },
//     Rotation {
//         center: Point2<float>,
//         count: u8,
//     },
//     Translation {
//         count: u8,
//         vector: Vector2<float>,
//         spacing: float,
//     },
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// pub struct SceneGroup {
//     pub contents: Vec<Entity>,
//     pub transforms: Vec<Transform>,
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// pub struct Config2 {
//     // the physics of it
//     pub scene: Vec<SceneGroup>,
//     // the view on it
//     pub rendering: Rendering,
//     // turning the brightness data into a picture
//     pub coloration: Coloration,
// }




