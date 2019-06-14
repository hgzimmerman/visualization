/// Two sets of lines emanate off of a circle in different directions.
///
///

use crate::ring_buffer::RingBuffer;
use nannou::prelude::*;

pub struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
    _window: WindowId,
    mouse_point: Option<Point2>,
    last_pts: [Point2; 4],
    entities: RingBuffer<Entity>,
    window_dimensions: Vector2,
    frame_counter: u64,
}

impl Model {
    pub fn init(app: &App) -> Model {
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
            mouse_point: None,
            last_pts: [Point2::default(); 4],
            entities: RingBuffer::new(370),
            window_dimensions: Vector2::default(),
            frame_counter: 0
        }
    }

    pub fn update(app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += 1;

        const CIRCLE_RADIUS: f32 = 100.0;
        const INNER_CIRCLE_RADIUS: f32 = 40.0;

        let new_point_0 = Point2 {
            x: (app.time * 2.0).cos() * CIRCLE_RADIUS,
            y: (app.time * 2.0).sin() * CIRCLE_RADIUS
        };
        let new_point_1 = Point2 {
            x: (app.time * 2.2).sin() * CIRCLE_RADIUS,
            y: (app.time * 2.2).cos() * CIRCLE_RADIUS
        };
        let new_point_2 = Point2 {
            x: (app.time * 2.0).cos() * INNER_CIRCLE_RADIUS,
            y: (app.time * 2.0).sin() * INNER_CIRCLE_RADIUS
        };
        let new_point_3 = Point2 {
            x: (app.time * 2.2).sin() * INNER_CIRCLE_RADIUS,
            y: (app.time * 2.2).cos() * INNER_CIRCLE_RADIUS
        };

        // Adds an entity to the ring buffer
        fn add_entity(last_point: Point2, new_point: Point2, color: Rgba, rb: &mut RingBuffer<Entity>) {
            const DIRECTION_SCALAR: f32 = 1.0;
            let new_entity = Entity {
                points: {
                    let mut buf = RingBuffer::new(40);
                    buf.push(last_point);
                    buf.push(new_point);
                    buf
                },
                direction: (last_point - new_point).normalize_to(DIRECTION_SCALAR),
                acceleration: (last_point - new_point).normalize_to(0.004),
                color
            };
            rb.push(new_entity);
        }

        if model.frame_counter % 6 == 0 {
            add_entity(model.last_pts[0], new_point_0, RED, &mut model.entities);
            add_entity(model.last_pts[1], new_point_1, BLUE, &mut model.entities);
            add_entity(model.last_pts[2], new_point_2, GREEN, &mut model.entities);
            add_entity(model.last_pts[3], new_point_3, ORANGE, &mut model.entities);
        }

        model.last_pts[0] = new_point_0;
        model.last_pts[1] = new_point_1;
        model.last_pts[2] = new_point_2;
        model.last_pts[3] = new_point_3;


        let mouse_point = model.mouse_point.clone();
        model.entities
            .iter_mut()
            .for_each(|e| {
                e.direction += e.acceleration;
                if let Some(mouse_point) = mouse_point {
                    let mouse_grav = (mouse_point - e.points.last().cloned().unwrap()).normalize_to(1.0) / (e.points.last().cloned().unwrap() - mouse_point).magnitude();
                    e.direction += mouse_grav;
                }

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
        WindowEvent::MouseMoved(point) => {
            if let Some(mouse_point) = model.mouse_point.as_mut() {
                *mouse_point = point;
            }
        }
        WindowEvent::MousePressed(_) => {
            if model.mouse_point.is_none() {
                model.mouse_point = Some(Point2::default())
            } else {
                model.mouse_point = None;
            }
        }
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

    frame.clear(DARK_CHARCOAL);

//    draw.background()
//        .color(LIGHT_YELLOW);

    model.entities
        .iter()
        .for_each(|e| {
//            let vertices = e.points
//                .iter()
//                .map(|pt| nannou::geom::vertex::Rgba(*pt, e.color));
//            draw.polyline()
//                .vertices(0.50, vertices);

            draw.line()
                .thickness(2.7)
                .color(e.color)
                .caps_round()
                .start(*e.points.first().unwrap())
                .end(*e.points.last().unwrap());
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame

}
