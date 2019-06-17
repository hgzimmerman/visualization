use nannou::prelude::*;
use std::num::Wrapping;
use common::l_system::{LSystem};
use crate::gosper::Gosper;

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
const INITIAL_THICKNESS: f32 = 2.0;
const INITIAL_LINE_LENGTH: f32 = 8.0;


fn build_point_buffer(iterations: usize, line_length: f32,) -> Vec<Point2> {
    let lsystem = LSystem::new(vec![Gosper::A]).iterate_n(iterations);
    // TODO Consider taking this with a default origin, finding the middle element and using that as the center and using that as an offset for rendering?
    let point_buffer: Vec<Point2> = lsystem
        .reify_iter(std::f32::consts::FRAC_PI_3, line_length, Point2::default())
        .collect();

    let len = if point_buffer.len() > 0 {
        point_buffer.len()
    } else {
        1
    };

    dbg!(len);
    let index = (len * 3/ 8) - 1; // TODO, prevent underflow
    dbg!(index);

    let center = point_buffer[index]; // This is only approximately the center

    let point_buffer: Vec<Point2> = point_buffer
        .into_iter()
        .map(|pt| {
            pt - center
        })
        .collect();

    point_buffer
}


impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 9")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        let point_buffer = build_point_buffer(ITERATION, INITIAL_LINE_LENGTH);

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
                    if model.iteration < 7 {
                        model.iteration += 1;
                    }
                    model.point_buffer = build_point_buffer(model.iteration,INITIAL_LINE_LENGTH)
                },
                Key::Left => {
                    if model.iteration > 0 {
                        model.iteration -= 1;
                    }
                    model.point_buffer = build_point_buffer(model.iteration,INITIAL_LINE_LENGTH)
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
        _ => println!("{:?}", event)
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    frame.clear(WHITE);

    let half_thickness: f32 = model.thickness / 2.0;

    let len = model.point_buffer.len();
    let colors = vec![
        (0.0, RED),
        (25.0, GREEN),
        (75.0, BLUE),
        (100.0, RED)
    ];
    let skip = model.frame_counter.0 % len;
    let skip = match model.iteration {
        0 => skip,
        1 => skip,
        2 => skip * 5,
        3 => skip * 20,
        4 => skip * 75,
        5 => skip * 400,
        6 => skip * 1000,
        7 => skip * 300,
        _ => skip
    };

    model.point_buffer
        .windows(2)
        .zip(nannou::color::Gradient::with_domain(colors)
            .take(len)
            .cycle()
            .skip(skip )
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


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}