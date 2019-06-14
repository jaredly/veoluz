use nalgebra::{Point2, Vector2};
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use shared::Parabola;
use shared::WallType;

pub fn draw_handles(
    wall: &WallType,
    ctx: &CanvasRenderingContext2d,
    size: f64,
    selected: Option<usize>,
) -> Result<(), JsValue> {
    for (i, handle) in wall.handles().iter().enumerate() {
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

pub fn draw(wall: &WallType, ctx: &CanvasRenderingContext2d) {
    match wall {
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

            let p0 = transform.transform_point(&Point2::new(*left, 0.0));
            let p1 = transform.transform_point(&Point2::new(*right, 0.0));
            ctx.begin_path();
            ctx.move_to(p0.x as f64, p0.y as f64);
            ctx.line_to(p1.x as f64, p1.y as f64);
            ctx.stroke();

            let p0 = transform.transform_point(&Point2::new(0.0, 0.0));
            let p1 = transform.transform_point(&Point2::new(0.0, 1.0 / (4.0 * a)));
            ctx.begin_path();
            ctx.move_to(p0.x as f64, p0.y as f64);
            ctx.line_to(p1.x as f64, p1.y as f64);
            ctx.stroke();
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
