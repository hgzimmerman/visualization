/// Draws a gasper curve.
///


use nannou::prelude::*;
use std::num::Wrapping;
use common::l_system::{LSystem};
use crate::sierpinski::Sierpinski;

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
const INITIAL_THICKNESS: f32 = 2.0;


/// Uses an L-system + Gosper grammar to construct a list of items.
/// Since the l system is reified with points starting at the center,
/// the path is not centered on the screen.
///
/// So an approximate spacial center of the path is found, and the points are all then offset from that.
fn build_point_buffer(iterations: usize) -> Vec<Point2> {
    use Sierpinski::*;
    let axiom = vec![F, Minus, G, Minus, G];
    let lsystem = LSystem::new(axiom).iterate_n(iterations);

    let line_length = 550.0 / 2.pow(iterations as u32) as f32;

    let origin = Point2 {
        x: -(2.pow(iterations as u32) as f32 / 2.0) * line_length,
        y: -(2.pow(iterations as u32) as f32 * 3.0.sqrt() / 2.0) * line_length / 2.0
    };

    let point_buffer: Vec<Point2> = lsystem
        .reify_iter(std::f32::consts::FRAC_PI_3 * 2.0, line_length, origin)
        .collect();

    point_buffer
}


impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 11")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
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
            thickness: INITIAL_THICKNESS,
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
                    if model.iteration < 10 {
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

    frame.clear(WHITE);

    let half_thickness: f32 = model.thickness / 2.0;


    let len = model.point_buffer.len();
    let colors = vec![
        (0.0, Rgba::new_u8(0xff, 0, 0, 0xff)),
        (25.0, Rgba::new_u8(0, 0xff, 0, 0xff)),
        (75.0, Rgba::new_u8(0, 0, 0xff, 0xff)),
        (100.0, Rgba::new_u8(0xff, 0, 0, 0xff)),
    ];
    let skip = model.frame_counter.0 % len;
    let skip = match model.iteration {
        3 => skip * 3,
        4 => skip * 3,
        5 => skip * 3,
        6 => skip * 9,
        7 => skip * 27,
        8 => skip * 81,
        9 => skip * 243,
        _ => skip
    };

    model.point_buffer
        .windows(2)
        .zip(nannou::color::Gradient::with_domain(colors.clone())
            .take(len)
            .cycle()
            .skip(skip)
        )
        .for_each(|(window, color): (&[Point2], Rgba)| {
            let pt_0 = window[0];
            let pt_1 = window[1];
            draw.line()
                .start(pt_0)
                .end(pt_1)
                .thickness(model.thickness)
                .color(color);
            draw.ellipse()
                .xy(pt_1)
                .color(color)
                .radius(half_thickness);
        });

    let last_color = nannou::color::Gradient::with_domain(colors)
        .take(len)
        .cycle()
        .skip(skip)
        .next().unwrap();

    draw.line()
        .start(model.point_buffer[0])
        .end(model.point_buffer[model.point_buffer.len() - 1])
        .thickness(model.thickness)
        .color(last_color);
    draw.ellipse()
        .xy(model.point_buffer[0])
        .color(last_color)
        .radius(half_thickness);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}