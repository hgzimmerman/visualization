//! Terdragon

use nannou::prelude::*;
use std::num::Wrapping;
use common::l_system::{LSystem};
use crate::terdragon::TerDragon;
use common::collinear::{condense_collinear};
use common::draw::{colored_lines_no_corners, rainbow};

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<usize>,
    /// Buffer containing all of the lines needed to draw the complete curve for the current iteration.
    point_buffer: Vec<Point2>,
    iteration: usize,
    thickness: f32,
}


const ITERATION: usize = 4;
const ITERATION_LIMIT: usize = 19;
const TITLE: &str = "day 16";
const INITIAL_THICKNESS: f32 = 1.0;
const THICKNESS_STEP: f32 = 0.25;


fn build_point_buffer(iterations: usize) -> Vec<Point2> {
    use TerDragon::*;
    let axiom = vec![F];
    let lsystem = LSystem::new(axiom).iterate_n(iterations);
    let line_length: f32 = 500.0 / (2.0 * (iterations as f32).powf(2.2));


    let origin = Point2::default();

    let point_buffer: Vec<Point2> = lsystem
        .reify_iter(2.0 * std::f32::consts::FRAC_PI_3 , line_length, origin)
        .collect();

    let point_buffer = condense_collinear(point_buffer);

    point_buffer
}





impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title(TITLE)
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
            thickness: INITIAL_THICKNESS
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
                    if model.iteration < ITERATION_LIMIT {
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
                    model.thickness += THICKNESS_STEP;
                }
                Key::Down => {
                    model.thickness -= THICKNESS_STEP;
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
        4 => skip * 1,
        5 => skip * 1,
        6 => skip * 1,
        7 => skip * 4,
        8 => skip * 5,
        9 => skip * 7,
        10 => skip * 12,
        11 => skip * 24,
        12 => skip * 48,
        13 => skip * 96,
        14 => skip * 200,
        15 => skip * 500,
        16 => skip * 1000,
        17 => skip * 2400,
        18 => skip * 5000,
        19 => skip * 10000,
        _ => skip
    };

    let gradient = rainbow();
    colored_lines_no_corners(&draw, &model.point_buffer, model.thickness, skip, gradient);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}

