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
        // TODO add an "offset" number that makes it start a bit out from the center
        // which will reduce the "bright point" in the middle, and probably make a cool
        // dark circle in the center too
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

impl Exposure {
    fn default() -> Self {
        Exposure {
            curve: Curve::FourthRoot,
            min: 0.0,
            max: 1.0,
        }
    }
}

pub mod v2 {
    use super::*;

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
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Transform {
    pub rotational_symmetry: u8,
    pub reflection: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Coloration {
    Rgb {
        background: (u8, u8, u8),
        highlight: (u8, u8, u8),
    },
    // For the moment I'll expect the first one to be lower
    // ermmm should I allow modifying the saturation and lightness?
    // that can come later probably
    HueRange {
        start: f32,
        end: f32,
        saturation: f32,
        lightness: f32,
    }, // TODO make colorful schemes and such
}

impl Coloration {
    fn default() -> Self {
        Coloration::Rgb {
            background: (0, 0, 0),
            highlight: (255, 255, 255),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Rendering {
    pub exposure: Exposure,
    pub coloration: Coloration,
    pub width: usize,
    pub height: usize,
    // default 0.0, 0.0
    pub center: Point2<float>,
    // default 1.0
    pub zoom: float,
}

impl Rendering {
    pub fn new(width: usize, height: usize) -> Self {
        Rendering {
            exposure: Exposure::default(),
            coloration: Coloration::default(),
            width,
            height,
            center: Point2::origin(),
            zoom: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub walls: Vec<Wall>,
    pub lights: Vec<LightSource>,
    pub transform: Transform,
    pub rendering: Rendering,
}

pub fn from_v2(
    v2::Config {
        walls,
        lights,
        reflection,
        exposure,
        width,
        height,
    }: v2::Config,
) -> Config {
    let translate = Vector2::new(-(width as f32) / 2.0, -(height as f32) / 2.0);
    Config {
        walls: walls
            .into_iter()
            .map(|mut wall| {
                wall.kind.translate(&translate);
                wall
            })
            .collect(),
        lights: lights
            .into_iter()
            .map(|mut light| {
                light.kind.translate(&translate);
                light
            })
            .collect(),
        transform: Transform {
            rotational_symmetry: reflection,
            reflection: false,
        },
        rendering: Rendering {
            width,
            height,
            exposure,
            coloration: Coloration::default(),
            center: Point2::origin(),
            zoom: 1.0,
        },
    }
}
