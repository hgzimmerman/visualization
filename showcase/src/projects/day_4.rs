use nannou::prelude::*;
use std::num::Wrapping;
use common::ring_buffer::RingBuffer;

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<u64>,
    circles: RingBuffer<Circle>
}

#[derive(Debug, Default, Clone)]
pub struct Circle {
    center: Point2,
    radius: f32,
    color: Rgba
}

impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 4")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        Model {
            _window,
            window_dimensions: Vector2::default(),
            frame_counter: Wrapping(0),
            circles: RingBuffer::new(60)
        }
    }

    pub fn update(app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += Wrapping(1);

        const SHOULD_ADD: u64 = 8;
        if model.frame_counter.0 % SHOULD_ADD == 0 {

            let radius = model.window_dimensions.y / 4.0;
            let center = Point2 {
                x: (app.time * 2.0).sin() * radius,
                y: (app.time * 2.0).cos() * radius
            };

            let colors = [
                Rgba::new_u8(0xe6, 0x26, 0x1f, 0x30), // RED
                Rgba::new_u8(0xeb, 0x75, 0x32, 0x30),
                Rgba::new_u8(0xf7, 0xd0, 0x38, 0x30), // f7d038
                Rgba::new_u8(0xa3, 0xe0, 0x48, 0x30), // a3e048
                Rgba::new_u8(0x49, 0xda, 0x9a, 0x30), // 49da9a
                Rgba::new_u8(0x34, 0xbb, 0xe6, 0x30), // 34bbe6
                Rgba::new_u8(0x43, 0x55, 0xdb, 0x30), // 4355db
                Rgba::new_u8(0xd2, 0x3b, 0xe7, 0x30), // d23be7
            ];

            let color_index = ((model.frame_counter.0 / SHOULD_ADD) % 8) as usize;
            let color = colors[color_index];

            let new_circle = Circle {
                center,
                radius,
                color
            };

            model.circles.push(new_circle);

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
            model.circles.clear()
        }
        WindowEvent::KeyPressed(Key::Q) => {
            std::process::exit(0); // Q -> exit program
        }
        _ => println!("{:?}", event)
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    frame.clear(BLACK);

    model.circles
        .iter()
        .for_each(|c| {
            draw.ellipse()
                .xy(c.center)
                .radius(c.radius)
                .color(c.color);
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}
