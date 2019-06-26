use nannou::prelude::*;
use std::num::Wrapping;
use crate::hilbert::{RegularHilbertIterator};
use common::collinear::condense_collinear;

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<usize>,
    /// The iteration to draw.
    iteration: usize,
    /// Buffer containing all of the lines needed to draw the complete curve for the current iteration.
    line_buffer: Vec<(Point2)>
}

fn fill_line_buffer(iteration: usize) -> Vec<Point2> {
    const SCALE: f32 = 480.0;
    fn transform(i: f32, n: f32) -> f32 {
        ((i - ((n - 1.0) / 2.0)) / n) * SCALE
    }
    let n = RegularHilbertIterator::new_with_iteration(iteration).n();
    let max_d = RegularHilbertIterator::new_with_iteration(iteration).d_max();

    let point_buffer: Vec<Point2> = RegularHilbertIterator::new_with_iteration(iteration)
        .take(max_d + 1)
        .map(|pt| Point2 {
            x: transform(pt.x() as f32, n as f32),
            y: transform(pt.y() as f32, n as f32)
        })
        .collect();
    // Saves about a fifth of the size.
    let point_buffer:Vec<Point2> = condense_collinear(point_buffer);
    point_buffer
}


impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 6")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        Model {
            _window,
            window_dimensions: Vector2::default(),
            frame_counter: Wrapping(0),
            iteration: 0,
            line_buffer: fill_line_buffer(0)
        }
    }

    pub fn update(_app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += Wrapping(1);

        let speed = speed(model.iteration);

        let max_d = RegularHilbertIterator::new_with_iteration(model.iteration).d_max();

        // Bump the iteration count if the max_d has been surpassed for .5 second
        if model.frame_counter.0 > (max_d / speed) + 45 {
            model.frame_counter.0 = 0;
            model.iteration += 1;
            model.line_buffer = fill_line_buffer(model.iteration);
        }

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
            model.iteration = 0;
            model.line_buffer = fill_line_buffer(model.iteration);
        }
        WindowEvent::KeyPressed(Key::Q) => {
            std::process::exit(0); // Q -> exit program
        }
        _ => println!("{:?}", event)
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();


    if model.frame_counter.0 == 0 {
        frame.clear(WHITE);
    }

    const THICKNESS: f32 = 6.0;
    let thickness = THICKNESS / model.iteration as f32;
    let half_thickness = thickness / 2.0;


    let speed = speed(model.iteration);

    model.line_buffer
        .windows(2)
        .skip(model.frame_counter.0 * speed) // TODO skipping is dumb, as it still eats CPU. Instead just index into the array to get the points.
        .take(speed)
        .for_each(|window: &[Point2]| {
            let pt_0 = window[0];
            let pt_1 = window[1];
            draw.line()
                .start(pt_0)
                .end(pt_1)
                .thickness(thickness)
                .color(BLACK);
            draw.ellipse()
                .xy(pt_1)
                .color(BLACK)
                .radius(half_thickness);
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}

fn speed(iteration: usize) -> usize {
    match iteration {
        1 => 1,
        2 => 3,
        3 => 9,
        _ => iteration.pow(3) / 4 + 1
    }
}