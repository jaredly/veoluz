use crate::line::float;
use crate::wall_type::WallType;
use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

pub trait Lerp {
    fn lerp_(&self, other: &Self, amount: f32) -> Self;
}

pub trait LerpEq {
    fn lerp(&self, other: &Self, amount: f32) -> Self;
}

impl<T: PartialEq + Clone + Lerp> LerpEq for T {
    fn lerp(&self, other: &Self, amount: f32) -> Self {
        if amount == 0.0 {
            self.clone()
        } else if amount == 1.0 {
            other.clone()
        } else if self == other {
            self.clone()
        } else {
            self.lerp_(other, amount)
        }
    }
}

impl Lerp for f32 {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        self + (other - self) * amount
    }
}

impl Lerp for usize {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        self + ((other - self) as f32 * amount) as usize
    }
}

impl Lerp for u8 {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        self + ((other - self) as f32 * amount) as u8
    }
}

impl Lerp for bool {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        if amount >= 0.5 { *other } else { *self }
    }
}

impl Lerp for Point2<f32> {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        Point2::new(
            self.x + (other.x - self.x) * amount,
            self.y + (other.y - self.y) * amount,
        )
    }
}

impl Lerp for Vector2<f32> {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        Vector2::new(
            self.x + (other.x - self.x) * amount,
            self.y + (other.y - self.y) * amount,
        )
    }
}

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

impl Lerp for Properties {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        Properties {
            absorb: self.absorb.lerp(&other.absorb, amount),
            reflect: self.absorb.lerp(&other.reflect, amount),
            roughness: self.absorb.lerp(&other.roughness, amount),
            refraction: self.absorb.lerp(&other.refraction, amount),
        }
    }
}

pub mod v0 {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct Wall {
        pub kind: WallType,
        pub properties: Properties,
        // TODO add "hide"
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

    impl LightKind {
        pub fn translate(&mut self, by: &Vector2<float>) {
            match self {
                LightKind::Point { origin, .. } => {
                    *origin = *origin + by;
                }
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq)]
    pub struct LightSource {
        pub kind: LightKind,
        // something between 0 and 1 I think?
        pub brightness: float,
    }

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
        pub walls: Vec<v0::Wall>,
        pub lights: Vec<v0::LightSource>,
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

impl Lerp for Exposure {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        if (other.curve != self.curve) {
            unimplemented!();
        }
        Exposure {
            curve: self.curve.clone(),
            min: self.min.lerp(&other.min, amount),
            max: self.max.lerp(&other.max, amount),
        }
    }
}

impl Exposure {
    fn default() -> Self {
        Exposure {
            curve: Curve::SquareRoot,
            min: 0.0,
            max: 1.0,
        }
    }
}

pub mod v2 {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq)]
    pub struct Config {
        pub walls: Vec<v0::Wall>,
        pub lights: Vec<v0::LightSource>,
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

impl Lerp for Transform {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        unimplemented!()
    }
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

impl<T: Clone + LerpEq> Lerp for (T, T, T) {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        (self.0.lerp(&other.0, amount), self.1.lerp(&other.1, amount), self.2.lerp(&other.2, amount))
    }
}

impl Lerp for Coloration {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        match (self, other) {
            (Coloration::Rgb {background, highlight}, Coloration::Rgb {background: b2, highlight: h2}) => {
                Coloration::Rgb {
                    background: background.lerp(b2, amount),
                    highlight: highlight.lerp(b2, amount),
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Coloration {
    fn default() -> Self {
        Coloration::Rgb {
            background: (0, 0, 0),
            highlight: (255, 255, 255),
        }
        // Coloration::HueRange {
        //     start: 100.0,
        //     end: 200.0,
        //     saturation: 100.0,
        //     lightness: 50.0,
        // }
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

impl Lerp for Rendering {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        Rendering {
            exposure: self.exposure.lerp(&other.exposure, amount),
            coloration: self.coloration.lerp(&other.coloration, amount),
            width: self.width.lerp(&other.width, amount),
            height: self.height.lerp(&other.height, amount),
            center: self.center.lerp(&other.center, amount),
            zoom: self.zoom.lerp(&other.zoom, amount),
        }
    }
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

pub mod v3 {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq)]
    pub struct Config {
        pub walls: Vec<v0::Wall>,
        pub lights: Vec<v0::LightSource>,
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
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Wall {
    pub kind: WallType,
    pub properties: Properties,
    pub hide: bool,
}

impl Lerp for Wall {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        Wall {
            kind: self.kind.lerp(&other.kind, amount),
            properties: self.properties.lerp(&other.properties, amount),
            hide: self.hide.lerp(&other.hide, amount),
        }
    }
}

impl From<v0::Wall> for Wall {
    fn from(other: v0::Wall) -> Self {
        Wall {
            kind: other.kind.clone(),
            properties: other.properties.clone(),
            hide: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum LightKind {
    Point {
        offset: float,
        origin: Point2<float>,
        t0: float,
        t1: float,
    },
}

impl Lerp for LightKind {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        match (self, other) {
            (LightKind::Point {offset, origin, t0, t1}, LightKind::Point {offset: o2, origin: or2, t0: t02, t1: t12}) => {
                LightKind::Point {
                    offset: offset.lerp(&o2, amount),
                    origin: origin.lerp(&or2, amount),
                    t0: offset.lerp(&t02, amount),
                    t1: offset.lerp(&t12, amount),
                }
            }
        }
    }
}

impl From<v0::LightKind> for LightKind {
    fn from(other: v0::LightKind) -> Self {
        match other {
            v0::LightKind::Point {origin, t0, t1} => LightKind::Point{origin, offset: 0.0, t0, t1}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LightSource {
    pub kind: LightKind,
    // something between 0 and 1 I think?
    pub brightness: float,
}

impl Lerp for LightSource {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        LightSource {
            kind: self.kind.lerp(&other.kind, amount),
            brightness: self.brightness.lerp(&other.brightness, amount),
        }
    }
}

impl From<v0::LightSource> for LightSource {
    fn from(other: v0::LightSource) -> Self {
        LightSource {
            kind: other.kind.into(),
            brightness: other.brightness,
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

impl<T: LerpEq> Lerp for Vec<T> {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        if other.len() != self.len() {
            panic!("Cannot lerp between vecs of different length")
        };
        self.iter().zip(other.iter()).map(|(one, two)| one.lerp(two, amount)).collect::<Vec<T>>()
    }
}

impl Lerp for Config {
    fn lerp_(&self, other: &Self, amount: f32) -> Self {
        Config {
            walls: self.walls.lerp(&other.walls, amount),
            lights: self.lights.lerp(&other.lights, amount),
            transform: self.transform.lerp(&other.transform, amount),
            rendering: self.rendering.lerp(&other.rendering, amount),
        }
    }
}

pub fn from_v3(
    v3::Config {
        walls,
        lights,
        transform,
        rendering,
    }: v3::Config,
) -> Config {
    Config {
        walls: walls.into_iter().map(|wall| wall.into()).collect(),
        lights: lights.into_iter().map(|light| light.into()).collect(),
        transform,
        rendering,
    }
}