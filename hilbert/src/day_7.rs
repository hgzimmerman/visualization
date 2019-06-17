use nannou::prelude::*;
use std::num::Wrapping;
use crate::hilbert::{HilbertIterator, Point};

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<usize>,
    /// Buffer containing all of the lines needed to draw the complete curve for the current iteration.
    line_buffer: Vec<(Point2, Point2)>
}


const ITERATION: usize = 5;

fn fill_line_buffer(iteration: usize, window_dimensions: Vector2) -> Vec<(Point2, Point2)> {
    fn transform(i: f32, n: f32, min_dimension: f32) -> f32 {
        ((i - ((n - 1.0) / 2.0)) / n) * min_dimension
    }
    let n = HilbertIterator::new_with_iteration(iteration).n();

    let scale = {
        let s: f32 = if window_dimensions.x < window_dimensions.y {
            window_dimensions.x
        } else {
            window_dimensions.y
        };
        s - 50.0
    };

    HilbertIterator::new_with_iteration(iteration)
        .map(|(pt_0, pt_1): (Point, Point)| {
            (
                Point2 {
                    x: transform(pt_0.x() as f32, n as f32, scale),
                    y: transform(pt_0.y() as f32, n as f32, scale),
                },
                Point2 {
                    x: transform(pt_1.x() as f32, n as f32, scale),
                    y: transform(pt_1.y() as f32, n as f32, scale),
                },
            )
        })
        .collect()
}


impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 7")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        Model {
            _window,
            window_dimensions: Vector2::default(),
            frame_counter: Wrapping(0),
            line_buffer: fill_line_buffer(ITERATION, Vector2::default())
        }
    }

    pub fn update(_app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += Wrapping(1);
    }
}

fn on_resize(_: &App, model: &mut Model, dimensions: Vector2) {
    model.window_dimensions = dimensions;
    model.line_buffer = fill_line_buffer(ITERATION, model.window_dimensions);
}

/// Handle events related to the window and update the model if necessary
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MouseMoved(_point) => {
        }
        WindowEvent::MousePressed(_) => {
        }
        WindowEvent::KeyPressed(Key::Space) => {
            model.frame_counter = Wrapping(0);
        }
        WindowEvent::KeyPressed(Key::Q) => {
            std::process::exit(0); // Q -> exit program
        }
        _ => println!("{:?}", event)
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    frame.clear(Rgba::new(0.05, 0.05, 0.05, 1.0));

    const THICKNESS: f32 = 6.0;
    const HALF_THICKNESS: f32 = THICKNESS / 2.0;

    let len = model.line_buffer.len();
    let colors = vec![
        (0.0, RED),
        (25.0, GREEN),
        (75.0, BLUE),
        (100.0, RED)
    ];
    let skip = model.frame_counter.0 % len;
    model.line_buffer
        .iter()
        .zip(nannou::color::Gradient::with_domain(colors)
            .take(len)
            .cycle()
            .skip(skip)
        )
        .for_each(|((pt_0, pt_1), color): (&(Point2, Point2), Rgba)| {
            draw.line()
                .start(*pt_0)
                .end(*pt_1)
                .thickness(THICKNESS)
                .color(color);
            draw.ellipse()
                .xy(*pt_1)
                .color(color)
                .radius(HALF_THICKNESS);
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}