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


const ITERATION: usize = 4;

fn fill_line_buffer(iteration: usize) -> Vec<(Point2, Point2)> {
    const SCALE: f32 = 480.0;
    fn transform(i: f32, n: f32) -> f32 {
        ((i - ((n - 1.0) / 2.0)) / n) * SCALE
    }
    let n = HilbertIterator::new_with_iteration(iteration).n();

    HilbertIterator::new_with_iteration(iteration)
        .map(|(pt_0, pt_1): (Point, Point)| {
            (
                Point2 {
                    x: transform(pt_0.x() as f32, n as f32),
                    y: transform(pt_0.y() as f32, n as f32),
                },
                Point2 {
                    x: transform(pt_1.x() as f32, n as f32),
                    y: transform(pt_1.y() as f32, n as f32),
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
            .with_title("day 8")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        Model {
            _window,
            window_dimensions: Vector2::default(),
            frame_counter: Wrapping(0),
            line_buffer: fill_line_buffer(ITERATION)
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

    frame.clear(WHITE);

    const THICKNESS: f32 = 10.0;
    const HALF_THICKNESS: f32 = THICKNESS / 2.0;

    model.line_buffer
        .iter()
        .take(1)
        .for_each(|(pt_0, _)| {
            draw.ellipse()
                .xy(*pt_0)
                .color(BLACK)
                .radius(HALF_THICKNESS);
        });

    model.line_buffer
        .iter()
        .for_each(|(pt_0, pt_1): &(Point2, Point2)| {
            draw.line()
                .start(*pt_0)
                .end(*pt_1)
                .thickness(THICKNESS)
                .color(BLACK);
            draw.ellipse()
                .xy(*pt_1)
                .color(BLACK)
                .radius(HALF_THICKNESS);
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}