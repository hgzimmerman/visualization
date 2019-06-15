use nannou::prelude::*;
use std::num::Wrapping;
use crate::hilbert::{HilbertIterator, Point};

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<usize>,
    d_counter: usize,
    iteration: usize
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
            d_counter: 0,
            iteration: 1
        }
    }

    pub fn update(app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += Wrapping(1);


        let speed = 2.0 / model.iteration.pow(2) as f32;

        model.d_counter = (model.frame_counter.0 as f32 / speed) as usize;

//        if model.frame_counter.0 % (5 / model.iteration) == 0 {
//            model.d_counter += 1;
//        }

        let max_d = HilbertIterator::new_with_iteration(model.iteration).d_max();

        // Bump the iteration count if the max_d has been surpassed for .5 second
        if model.d_counter > max_d + (30.0 / speed) as usize {
            model.frame_counter.0 = 0;
            model.d_counter = 0;
            model.iteration += 1;
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
            model.iteration = 1;
            model.d_counter = 0;
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

    let iteration = model.iteration;

    const SCALE: f32 = 480.0;

    let n = HilbertIterator::new_with_iteration(iteration).n();

    fn transform(i: f32, n: f32) -> f32 {
        ((i - ((n - 1.0) / 2.0)) / n) * SCALE
    }

    HilbertIterator::new_with_iteration(iteration)
        .take(model.d_counter)
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
        .for_each(|(pt_0, pt_1): (Point2, Point2)| {
            draw.line()
                .start(pt_0)
                .end(pt_1)
                .thickness(2.0)
                .color(BLACK);
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}