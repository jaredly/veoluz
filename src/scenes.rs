use nalgebra::{Point2, Vector2};
use ncollide2d::query::Ray;
use ncollide2d::shape::Ball;
use shared;
use shared::line;
use shared::{Wall, WallType};
use wasm_bindgen::prelude::*;

use std::f32::consts::PI;

pub fn apple() -> shared::Config {
    let width = 1024;
    let height = 576;
    let cx = (width / 2) as line::float;
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

    let radius = 100.0;
    let by = line::PI * 2.0 / (count as line::float);

    for i in 0..count {
        let theta = i as line::float * by;

        let r0 = 100.0;
        let r1 = 250.0;
        let td = by / 2.0;

        let index = 0.6;

        // walls.push(Wall::transparent(WallType::Line(ncollide2d::shape::Segment::new(
        //     Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
        //     Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
        // )), 1.1));

        walls.push(Wall::transparent(
            WallType::Line(ncollide2d::shape::Segment::new(
                Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
                Point2::new(cx + (theta + td).cos() * r0, cy + (theta + td).sin() * r0),
            )),
            index,
        ));

        walls.push(Wall::transparent(
            WallType::Line(ncollide2d::shape::Segment::new(
                Point2::new(cx + theta.cos() * r1, cy + theta.sin() * r1),
                Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
            )),
            1.0 / index,
        ));

        walls.push(Wall::transparent(
            WallType::Circle(
                Ball::new(r0 / 5.0),
                Point2::new(cx + (theta).cos() * r0 / 2.0, cy + (theta).sin() * r0 / 2.0),
                // -PI,
                // PI,
                theta + PI / 2.0,
                theta - PI / 2.0,
            ),
            0.8,
        ));

        // walls.push(Wall::mirror(WallType::Circle(
        //     Ball::new(r0 / 5.0),
        //     Point2::new(
        //         cx + (theta).cos() * r0 / 2.0,
        //         cy + (theta).sin() * r0 / 2.0,
        //     ),
        //     theta - PI / 2.0,
        //     theta + PI / 2.0,
        // )));

        // walls.push(Wall::mirror(WallType::Circle(
        //     Ball::new(radius),
        //     Point2::new(
        //         cx + (theta).cos() * (radius - 20.0),
        //         cy + (theta).sin() * (radius - 20.0),
        //     ),
        //     0.0 + theta,
        //     PI + theta
        //     // angle_norm(theta),
        //     // angle_norm(theta + 0.1),
        //     // angle_norm(theta - std::f32::consts::PI * 4.0 / 5.0),
        //     // angle_norm(theta - std::f32::consts::PI * 3.0 / 5.0),
        // )))
    }
    shared::Config::new(walls, width as usize, height as usize)
}


pub fn refraction_test() -> shared::Config {
    let width = 1024;
    let height = 576;
    let cx = (width / 2) as line::float;
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

    let radius = 100.0;
    let by = line::PI * 2.0 / (count as line::float);

    for i in 0..count {
        let theta = i as line::float * by;

        let r0 = 100.0;
        let r1 = 250.0;
        let td = by / 2.0;

        let index = 0.6;

        // walls.push(Wall::transparent(WallType::Line(ncollide2d::shape::Segment::new(
        //     Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
        //     Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
        // )), 1.1));

        walls.push(Wall::transparent(
            WallType::Line(ncollide2d::shape::Segment::new(
                Point2::new(cx + theta.cos() * r0, cy + theta.sin() * r0),
                Point2::new(cx + (theta + td).cos() * r1, cy + (theta + td).sin() * r1),
            )),
            if (i % 2 == 0 ) { index } else { 1.0 / index},
        ));

        // walls.push(Wall::transparent(
        //     WallType::Circle(
        //         Ball::new(r1 / 5.0),
        //         Point2::new(cx + (theta).cos() * r1, cy + (theta).sin() * r1),
        //         // -PI,
        //         // PI,
        //         theta + PI / 2.0,
        //         theta - PI / 2.0,
        //     ),
        //     0.8,
        // ));

        // walls.push(Wall::mirror(WallType::Circle(
        //     Ball::new(r0 / 5.0),
        //     Point2::new(
        //         cx + (theta).cos() * r0 / 2.0,
        //         cy + (theta).sin() * r0 / 2.0,
        //     ),
        //     theta - PI / 2.0,
        //     theta + PI / 2.0,
        // )));

        // walls.push(Wall::mirror(WallType::Circle(
        //     Ball::new(radius),
        //     Point2::new(
        //         cx + (theta).cos() * (radius - 20.0),
        //         cy + (theta).sin() * (radius - 20.0),
        //     ),
        //     0.0 + theta,
        //     PI + theta
        //     // angle_norm(theta),
        //     // angle_norm(theta + 0.1),
        //     // angle_norm(theta - std::f32::consts::PI * 4.0 / 5.0),
        //     // angle_norm(theta - std::f32::consts::PI * 3.0 / 5.0),
        // )))
    }
    shared::Config::new(walls, width as usize, height as usize)
}