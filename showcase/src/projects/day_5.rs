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
            .with_title("day 5")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        Model {
            _window,
            window_dimensions: Vector2::default(),
            frame_counter: Wrapping(0),
            circles: RingBuffer::new(80)
        }
    }

    pub fn update(app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += Wrapping(1);
        let count = model.frame_counter.0;

        const SHOULD_ADD: u64 = 3;
        if model.frame_counter.0 % SHOULD_ADD == 0 {


            let radius = (model.window_dimensions.y / 8.0) + (count / (SHOULD_ADD / 2)) as f32;
            let center = Point2 {
                x: (app.time * 2.0).sin() * radius,
                y: (app.time * 2.0).cos() * radius
            };

            let opacity = 0x18;

            let colors = [
                Rgba::new_u8(0xe6, 0x26, 0x1f, opacity), // RED
                Rgba::new_u8(0xeb, 0x75, 0x32, opacity),
                Rgba::new_u8(0xf7, 0xd0, 0x38, opacity), // f7d038
                Rgba::new_u8(0xa3, 0xe0, 0x48, opacity), // a3e048
                Rgba::new_u8(0x49, 0xda, 0x9a, opacity), // 49da9a
                Rgba::new_u8(0x34, 0xbb, 0xe6, opacity), // 34bbe6
                Rgba::new_u8(0x43, 0x55, 0xdb, opacity), // 4355db
                Rgba::new_u8(0xd2, 0x3b, 0xe7, opacity), // d23be7
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
            model.circles.clear();
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