

use nannou::prelude::*;
use std::num::Wrapping;
use common::l_system::{LSystem};
use crate::peano::Peano;
use common::collinear::{condense_collinear};
use common::draw::rainbow_lines;

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<usize>,
    /// Buffer containing all of the lines needed to draw the complete curve for the current iteration.
    point_buffer: Vec<Point2>,
    iteration: usize,
    thickness: f32,
}


const ITERATION: usize = 3;


fn build_point_buffer(iterations: usize) -> Vec<Point2> {
    use Peano::*;
    let axiom = vec![L];
    let lsystem = LSystem::new(axiom).iterate_n(iterations);
    let line_length: f32 = 400.0 / (3.pow(iterations as u32)) as f32;

    // Rule = f(n-1) * 3 + 1
    fn offset(n: usize) -> usize {
        if n == 0 {
            0
        } else {
            offset(n - 1) * 3 + 1
        }
    }

    let o = offset(iterations) as f32;

    let origin = Point2 {
        x: -(o * line_length),
        y: (o * line_length)
    };



    let mut point_buffer: Vec<Point2> = lsystem
        .reify_iter(std::f32::consts::FRAC_PI_2 , line_length, origin)
        .collect();

    // for some reason, the last point doesn't get created, so this section generates it for us
    if let Some(last) = point_buffer.get(point_buffer.len() - 1) {
        let last = Point2 {
            x: last.x + line_length,
            y: last.y
        };
        point_buffer.push(last)
    }

    let point_buffer = condense_collinear(point_buffer);

    point_buffer
}





impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 13")
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
            thickness: 2.0
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
                Key::Up => {
                    model.thickness += 1.0;
                }
                Key::Down => {
                    model.thickness -= 1.0;
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

    frame.clear(DARK_CHARCOAL);


    let len = model.point_buffer.len();
    let skip = model.frame_counter.0 % len;
    let skip = match model.iteration {
        3 => skip * 1,
        4 => skip * 9,
        5 => skip * 81,
        6 => skip * 700,
        7 => skip * 81,
        8 => skip * 81,
        9 => skip * 243,
        _ => skip
    };

    rainbow_lines(&draw, &model.point_buffer, model.thickness, model.thickness, skip);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}

