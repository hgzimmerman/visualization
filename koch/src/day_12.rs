/// Draws a tiled set of kosh snowflakes and anti-snowflakes.


use nannou::prelude::*;
use std::num::Wrapping;
use common::l_system::{LSystem};
use crate::koch::Koch;
use nannou::app::Draw;

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<usize>,
    /// Buffer containing all of the lines needed to draw the complete curve for the current iteration.
    point_buffer: Vec<Point2>,
    iteration: usize,

}


const ITERATION: usize = 3;


fn build_point_buffer(iterations: usize) -> Vec<Point2> {
    use Koch::*;
    let axiom = vec![F, Minus, Minus, F, Minus, Minus, F];
    let lsystem = LSystem::new(axiom).iterate_n(iterations);
    let line_length: f32 = 200.0 / (3.pow(iterations as u32)) as f32;

    let origin = Point2 {
        x: -(1.5 * 3.pow((iterations - 1) as u32) as f32) * line_length,
        y: (((3.0.sqrt() / 2.0) * 3.pow((iterations - 1) as u32) as f32) * line_length) as f32,
    };


    let point_buffer: Vec<Point2> = lsystem
        .reify_iter(std::f32::consts::FRAC_PI_3 , line_length, origin)
        .collect();

    point_buffer
}


impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 12")
            .view(view)
            .event(event)
            .resized(on_resize)
            .build()
            .unwrap();

        let point_buffer = build_point_buffer(ITERATION);

        Model {
            _window,
            window_dimensions: Vector2::default(),
            frame_counter: Wrapping(0),
            point_buffer,
            iteration: ITERATION,
        }
    }

    pub fn update(_app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += Wrapping(1);
    }
}

fn on_resize(_: &App, model: &mut Model, dimensions: Vector2) {
    model.window_dimensions = dimensions;
}

/// Handle events related to the window and update the model if necessary
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MouseMoved(_point) => {
        }
        WindowEvent::MousePressed(_) => {
        }
        WindowEvent::KeyPressed(key) => {
            match key {
                Key::Right => {
                    if model.iteration < 7 {
                        model.iteration += 1;
                    }
                    model.point_buffer = build_point_buffer(model.iteration)
                },
                Key::Left => {
                    if model.iteration > 0 {
                        model.iteration -= 1;
                    }
                    model.point_buffer = build_point_buffer(model.iteration)
                }
                Key::Q => {
                    std::process::exit(0); // Q -> exit program
                }
                _ => {}
            }

        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    frame.clear(DARK_BLUE);


    draw.ellipse()
        .radius(3.0)
        .xy(Point2::default())
        .color(BLACK);

    let pb = &model.point_buffer;
    let skip = model.point_buffer.len() / 6;

     (0..6)
        .map(|i| pb[i*skip])
        .map(|o| o * 2.0) // I want the origins of the other snowflakes to be 2x the distance to this pt.
        .map(|origin| {
            model.point_buffer
                .iter()
                .map(|pt| -> Point2 {
                   *pt + origin
                }).collect::<Vec<_>>()
        })
        .for_each(|snowflake| {
            paint_koch(&draw, &snowflake, ORANGE);
        });

        paint_koch(&draw, &model.point_buffer, ORANGE);




    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}

/// Because the Koch snowflake is concave, a vector of points can't just be supplied.
///
/// Instead, this function recursively (depth-first) paints the triangles that comprise the snowflake
/// from largest to smallest.
fn paint_koch(draw: &Draw, pts: &[Point2], color: Rgba) {
    let skip = pts.len() / 3;

    draw.polygon()
        .points([pts[0], pts[skip], pts[2*skip]].iter().cloned())
        .color(color);

    fn paint_koch_inner(draw: &Draw, pts: &[Point2], color: Rgba) {
        let skip = pts.len() / 4;

        draw.polygon()
            .points([pts[skip], pts[2*skip], pts[3*skip]].iter().cloned())
            .color(color);

        let (temp1, temp2) = pts.split_at(pts.len()/2);
        let (sec1, sec2) = temp1.split_at(pts.len()/4);
        let (sec3, sec4) = temp2.split_at(pts.len()/4);

        if skip >= 4 {
            paint_koch_inner(draw, sec1, color);
            paint_koch_inner(draw, sec2, color);
            paint_koch_inner(draw, sec3, color);
            paint_koch_inner(draw, sec4, color);
        }
    }

    let (sec1, sec2) = pts.split_at(skip);
    let (sec2, sec3) = sec2.split_at(skip);

    if skip >= 4 {
        paint_koch_inner(draw, sec1, color);
        paint_koch_inner(draw, sec2, color);
        paint_koch_inner(draw, sec3, color);
    }

}
