use nalgebra::Point2;

use ncollide2d::shape::Ball;
use shared;
use shared::line;
use shared::{Wall, WallType};

use std::f32::consts::PI;

pub fn parabola_test() -> shared::Config {
    let width = 1024;
    let height = 576;
    let walls = vec![
        Wall::mirror(WallType::Parabola(shared::Parabola {
            a: -1.0 / (4.0 * 50.0),
            left: -50.0,
            right: 50.0,
            transform: nalgebra::Isometry2::from_parts(
                nalgebra::Translation2::from(nalgebra::Vector2::new(
                    width as f32 / 2.0,
                    height as f32 / 2.0 + 50.0,
                )),
                nalgebra::UnitComplex::from_angle(0.0),
            ),
        })),
        Wall::mirror(WallType::Parabola(shared::Parabola {
            a: -0.01,
            left: -50.0,
            right: 50.0,
            transform: nalgebra::Isometry2::from_parts(
                nalgebra::Translation2::from(nalgebra::Vector2::new(
                    width as f32 / 2.0,
                    height as f32 / 2.0 + 80.0,
                )),
                nalgebra::UnitComplex::from_angle(0.0),
            ),
        })),
        Wall::mirror(WallType::Parabola(shared::Parabola {
            a: -0.003,
            left: -50.0,
            right: 50.0,
            transform: nalgebra::Isometry2::from_parts(
                nalgebra::Translation2::from(nalgebra::Vector2::new(
                    width as f32 / 2.0,
                    height as f32 / 2.0 + 100.0,
                )),
                nalgebra::UnitComplex::from_angle(0.0),
            ),
        })),
        Wall::transparent(
            WallType::Parabola(shared::Parabola {
                a: -0.003,
                left: -50.0,
                right: 50.0,
                transform: nalgebra::Isometry2::from_parts(
                    nalgebra::Translation2::from(nalgebra::Vector2::new(
                        width as f32 / 2.0,
                        height as f32 / 2.0 + 140.0,
                    )),
                    nalgebra::UnitComplex::from_angle(0.0),
                ),
            }),
            1.5,
        ),
        Wall::transparent(
            WallType::Parabola(shared::Parabola {
                a: -0.003,
                left: -50.0,
                right: 50.0,
                transform: nalgebra::Isometry2::from_parts(
                    nalgebra::Translation2::from(nalgebra::Vector2::new(
                        width as f32 / 2.0,
                        height as f32 / 2.0 + 240.0,
                    )),
                    nalgebra::UnitComplex::from_angle(0.0),
                ),
            }),
            2.4,
        ),
    ];

    shared::Config::new(walls, width as usize, height as usize)
}

pub fn apple() -> shared::Config {
    let width = 1024;
    let height = 576;
    let cx = 0.0;
    let cy = 0.0;

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

    let _radius = 100.0;
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
    let mut config = shared::Config::new(walls, width as usize, height as usize);

    // config.lights.push(
    //     shared::LightSource {
    //         kind: shared::LightKind::Point {
    //             origin: Point2::new(100.0, 100.0),
    //             t0: 0.0,
    //             t1: std::f32::consts::PI
    //         },
    //         brightness: 0.5
    //     }
    // );

    config
}

use ncollide2d::shape::Segment;

pub fn playground() -> shared::Config {
    let width = 1024;
    let height = 576;
    let mut walls = vec![];

    for wt in WallType::rand_all(width, height) {
        walls.push(Wall::mirror(wt))
    }

    for wt in WallType::rand_all(width, height) {
        walls.push(Wall::transparent(wt, 2.4))
    }

    for wt in WallType::rand_all(width, height) {
        walls.push(Wall::transparent(wt, 1.0 / 2.4))
    }

    // for wt in WallType::rand_all(width, height) {
    //     walls.push(Wall::transparent(wt, 1.5))
    // }

    walls.push(Wall::block(WallType::rand_line(width, height)));
    walls.push(Wall::block(WallType::rand_line(width, height)));
    walls.push(Wall::block(WallType::rand_line(width, height)));

    shared::Config::new(walls, width as usize, height as usize)
}

pub fn circle_row() -> shared::Config {
    let width = 1024;
    let height = 576;
    let cx = (width / 2) as line::float;
    let cy = (height / 2) as line::float;
    let mut walls = vec![];

    let count = 10;

    let _radius = 100.0;
    let by = line::PI * 2.0 / (count as line::float);

    let r0 = 100.0;

    for i in 0..count {
        let _theta = i as line::float * by;

        walls.push(Wall::transparent(
            WallType::Circle(
                Ball::new(r0 / 5.0),
                Point2::new(cx - r0 * count as f32 / 4.0 + r0 * i as f32 / 2.0, cy),
                -PI,
                PI,
            ),
            if i % 2 == 0 { 0.8 } else { 1.0 / 0.8 },
        ));
    }

    let count = 3;
    for i in -count..=count {
        if i == 0 {
            continue;
        }
        let _theta = i as line::float * by;

        walls.push(Wall::transparent(
            WallType::Circle(
                Ball::new(r0 / 5.0),
                Point2::new(cx, cy + r0 * i as f32 / 2.0),
                -PI,
                PI,
            ),
            if i % 2 == 0 { 0.8 } else { 1.0 / 0.8 },
        ));
    }

    shared::Config::new(walls, width as usize, height as usize)
}

pub fn circles() -> shared::Config {
    let width = 1024;
    let height = 576;
    let cx = (width / 2) as line::float;
    let cy = (height / 2) as line::float;
    let mut walls = vec![];

    let count = 10;

    let _radius = 100.0;
    let by = line::PI * 2.0 / (count as line::float);

    let r0 = 100.0;

    for i in 0..count {
        let theta = i as line::float * by;

        walls.push(Wall::transparent(
            WallType::Circle(
                Ball::new(r0 / 5.0),
                Point2::new(cx + (theta).cos() * r0, cy + (theta).sin() * r0),
                -PI,
                PI,
            ),
            if i % 2 == 0 { 0.8 } else { 1.0 / 0.8 },
        ));
    }
    shared::Config::new(walls, width as usize, height as usize)
}

pub fn refract2() -> shared::Config {
    let width = 1024;
    let height = 576;
    let index = 0.5;
    let cx = (width / 2) as line::float;
    let cy = (height / 2) as line::float;
    let walls = vec![
        Wall::transparent(
            WallType::Line(Segment::new(
                Point2::new(cx - 50.0, cy - 50.0),
                Point2::new(cx + 50.0, cy - 70.0),
            )),
            index,
        ),
        Wall::transparent(
            WallType::Circle(Ball::new(50.0), Point2::new(cx, cy + 200.0), -PI, PI),
            index,
        ),
        Wall::transparent(
            WallType::Parabola(shared::Parabola::new(
                -70.0,
                -50.0,
                50.0,
                Point2::new(cx - 100.0, cy),
                PI / 2.0,
            )),
            index,
        ),
        Wall::transparent(
            WallType::Circle(
                Ball::new(50.0),
                Point2::new(cx + 100.0, cy + 200.0),
                0.0,
                PI,
            ),
            index,
        ),
    ];

    let mut config = shared::Config::new(walls, width as usize, height as usize);
    config.transform.rotational_symmetry = 3;
    config
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

    let _radius = 100.0;
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
            if i % 2 == 0 { index } else { 1.0 / index },
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
