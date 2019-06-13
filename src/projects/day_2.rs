/// Two sets of lines emanate off of a circle in different directions.
///
///

use crate::ring_buffer::RingBuffer;
use nannou::prelude::*;

pub struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
    _window: WindowId,
    last_point_1: Point2,
    last_point_2: Point2,
    entities: RingBuffer<Entity>,
    window_dimensions: Vector2,
    frame_counter: u64,
}

impl Model {
    pub fn init(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 2")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        Model {
            _window,
            last_point_1: Point2::default(),
            last_point_2: Point2::default(),
            entities: RingBuffer::new(140),
            window_dimensions: Vector2::default(),
            frame_counter: 0
        }
    }

    pub fn update(app: &App, model: &mut Model, update: Update) {
        model.frame_counter += 1;
        let since_last: f32 = update.since_last.secs() as f32;

        let last_point_1 = model.last_point_1;
        let new_point_1 = Point2 {
            x: app.time.cos() * 70.0,
            y: app.time.sin() * 70.0
        };
        model.last_point_1 = new_point_1;

        let last_point_2 = model.last_point_2;
        let new_point_2 = Point2 {
            x: (app.time * 1.1).sin() * 70.0,
            y: (app.time * 1.1).cos() * 70.0
        };
        model.last_point_2 = new_point_2;



        if model.frame_counter % 10 == 0 {
            const DIRECTION_SCALAR: f32 = 0.7;
            let new_entity_1 = Entity {
                points: {
                    let mut buf = RingBuffer::new(20);
                    buf.push(last_point_1);
                    buf.push(new_point_1);
                    buf
                },
                direction: (last_point_1 - new_point_1) * DIRECTION_SCALAR,
                acceleration: Vector2::default(),
                color: RED
            };
            model.entities.push(new_entity_1);

            let new_entity_2 = Entity {
                points: {
                    let mut buf = RingBuffer::new(20);
                    buf.push(last_point_2);
                    buf.push(new_point_2);
                    buf
                },
                direction: (last_point_2 - new_point_2) * DIRECTION_SCALAR,
                acceleration: Vector2::default(),
                color: BLUE
            };
            model.entities.push(new_entity_2);


        }
        model.entities
            .iter_mut()
            .for_each(|e| {
                e.direction += e.acceleration;
                let next_pt = *e.points.last().unwrap() + e.direction;
                e.points.push(next_pt);
            });
    }
}

fn on_resize(_: &App, model: &mut Model, dimensions: Vector2) {
    model.window_dimensions = dimensions;
    println!("Resized: {:?}", dimensions);
}



#[derive(Debug)]
struct Entity {
    points: RingBuffer<Point2>,
    direction: Vector2,
    acceleration: Vector2,
    color: Rgba<f32>
}


/// Handle events related to the window and update the model if necessary
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
//        WindowEvent::MouseMoved(point) => {
//            model.point = point;
//        }
        WindowEvent::KeyPressed(Key::Space) => {
            model.entities.clear(); // Space -> remove circles
        }
        WindowEvent::KeyPressed(Key::Q) => {
            std::process::exit(0) // Q -> exit program
        }
        _ => println!("{:?}", event)
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    draw.background()
        .color(DARK_BLUE);

    model.entities
        .iter()
        .for_each(|e| {
//            let vertices = e.points
//                .iter()
//                .map(|pt| nannou::geom::vertex::Rgba(*pt, e.color));
//            draw.polyline()
//                .vertices(0.50, vertices);

            draw.line()
                .color(e.color)
                .start(*e.points.first().unwrap())
                .end(*e.points.last().unwrap());
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame

}
