use nannou::prelude::*;
use nannou::audio::buffer::Buffer;
use std::sync::{Mutex, Arc};
use crate::ring_buffer::RingBuffer;

mod ring_buffer;

fn main() {
    nannou::app(model)
        .update(regular)
        .run();
}

struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
    _window: WindowId,
    point: Point2,
    entities: RingBuffer<Entity>,
//    _audio_stream: audio::Stream<Arc<Mutex<Audio>>>,
    audio: Arc<Mutex<Audio>>
}

#[derive(Default, Debug)]
struct Audio {
//    phase: f64,
    hz: f64, // TODO perform a fourier transform to get this boy.
    avg_amp: f32,
    sample_rate: f64,
}

#[derive(Default, Debug)]
struct Entity {
    point: Point2,
    direction: Point2,
}
impl Entity {

    /// This will create a distribution of points expanding in roughly a square shape.
    fn new_random_direction(speed_range: f32, point: Point2) -> Self  {
        assert!(speed_range >= 0.0, "Speed range must be positive");
        // TODO, make this a better distribution (bimodal normal), constrained to a unit circle.
        Entity {
            point,
            direction: Point2 {
                x: nannou::rand::random_range(-speed_range, speed_range),
                y: nannou::rand::random_range(-speed_range, speed_range)
            },
        }
    }
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let _window = app
        .new_window()
        .with_dimensions(512, 512)
        .with_title("yeet")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .event(event) // The function that will be called when the window receives events.
        .build()
        .unwrap();

    let audio_model = Arc::new(
        Mutex::new(
            Audio {
                hz: 440.0,
                avg_amp: 0.0,
                sample_rate: 0.0
            }
        )
    );
    let _audio_stream = app
        .audio
        .new_input_stream(audio_model.clone(), audio)
        .device(app.audio.default_input_device().unwrap())
        .build()
        .unwrap();

    Model {
        _window,
        point: Point2::default(),
        entities: RingBuffer::new(525),
//        _audio_stream,
        audio: audio_model
    }
}



fn audio(audio: &mut Arc<Mutex<Audio>>, buffer: &Buffer) {
    let sample_rate = buffer.sample_rate() as f64;

    let avg_amp = buffer
        .frames()
        .map(|frame| frame.iter().map(|c|f32::abs(*c)).sum::<f32>() / frame.iter().count() as f32) // Average amp over all channels for each frame
        .fold(0.0, |acc, f: f32| acc + f) / buffer.frames().count() as f32; // Average amp over all frames

    let mut audio = audio.lock().unwrap();
    audio.avg_amp = avg_amp;
    audio.sample_rate = sample_rate
}

/// Handle events related to the window and update the model if necessary
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MouseMoved(point) => {
            model.point = point;
        }
        WindowEvent::MousePressed(_mouse_button) => {
            // Add a bunch of entities
            (0..50).for_each(|_| {
                let entity = Entity::new_random_direction(55.0, model.point);
                model.entities.push(entity);
            });
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

fn regular(_app: &App, model: &mut Model, update: Update) {
    let since_last: f32 = update.since_last.secs() as f32;

    let avg_audio_amp = model.audio.lock().unwrap().avg_amp;
    let multiplier = if avg_audio_amp > 0.01 {
       avg_audio_amp * 100.0
    } else {
        1.0
    };

    model.entities
        .iter_mut()
        .for_each(|e| {
            e.point += e.direction * since_last * multiplier;
        });
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) -> Frame {

    // Prepare to draw.
    let draw = app.draw();

    draw.line()
        .start(Point2::default())
        .end(Point2{
            x: app.time.sin() * 50.0,
            y: app.time.cos() * 50.0
        })
        .thickness(3.5);

    draw.background()
        .color(PURPLE);

    draw.ellipse()
        .x(model.point.x)
        .y(model.point.y)
        .width(10.0)
        .height(10.0)
        .color(DARK_BLUE);

    const B: f32 = 6.0;

    model.entities.iter().for_each(|e| {
        draw.ellipse()
            .x(e.point.x)
            .y(e.point.y)
            .width(B)
            .height(B)
            .color(DARK_RED);
    });

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}


