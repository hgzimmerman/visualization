/// Clicking randomly spawns clusters of moving dots.
/// These dots speed up depending on the amplitude of an input audio source.
///
/// ALSA / PA are unstable, so this is prone to crashing.

use nannou::prelude::*;
use nannou::audio::buffer::Buffer;
use std::sync::{Mutex, Arc};
use common::ring_buffer::RingBuffer;
use nannou::audio::Stream;
use common::fft;


pub struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
    _window: WindowId,
    _audio_stream: Option<audio::Stream<Arc<Mutex<Audio>>>>,
    point: Point2,
    entities: RingBuffer<Entity>,
    audio: Arc<Mutex<Audio>>,
    window_dimensions: Vector2,
    frame_counter: u64,
}

impl Model {
    pub fn init(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("yeet")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        let audio_model = Arc::new(
            Mutex::new(
                Audio {
                    fft: Vec::new(),
                    avg_amp: 0.0,
                    sample_rate: 0.0
                }
            )
        );
        const AUDIO_ENABLED: bool = false;
        let _audio_stream = init_audio_stream(app,audio_model.clone(), AUDIO_ENABLED);

        Model {
            _window,
            _audio_stream,
            point: Point2::default(),
            entities: RingBuffer::new(1525),
            audio: audio_model,
            window_dimensions: Vector2::default(),
            frame_counter: 0
        }
    }

    pub fn update(_app: &App, model: &mut Model, update: Update) {
        model.frame_counter += 1;
        let since_last: f32 = update.since_last.secs() as f32;

        let avg_audio_amp = model.audio.lock().unwrap().avg_amp;
        let multiplier = if avg_audio_amp > 0.01 {
           avg_audio_amp * 100.0
        } else {
            1.0
        };
        let window_dimensions = &model.window_dimensions;

        if model.frame_counter % 60 == 0 {
            // Remove the entities that are out of bounds
            model.entities.retain(|e| {
                !(e.point.x.abs() > window_dimensions.x / 2.0
                || e.point.y.abs() > window_dimensions.y / 2.0)
            });
        }

        model.entities
            .iter_mut()
            .for_each(|e| {
                e.direction += e.acceleration * since_last;
                e.point += e.direction * since_last * multiplier;
            });
    //    dbg!(model.entities.occupied);
    }
}

#[derive(Default, Debug)]
struct Audio {
//    phase: f64,
    fft: Vec<f32>,
    avg_amp: f32,
    sample_rate: f64,
}

#[derive(Default, Debug)]
struct Entity {
    point: Point2,
    direction: Point2,
    acceleration: Point2,
    color: Rgba<f32>
}

impl Entity {
    /// This will create a distribution of points expanding in roughly a square shape.
    fn new_random_direction(speed_range: f32, point: Point2) -> Self  {
        assert!(speed_range >= 0.0, "Speed range must be positive");
        // TODO, make this a better distribution (bimodal normal), constrained to a unit circle.
        let direction= Point2 {
            x: nannou::rand::random_range(-speed_range, speed_range),
            y: nannou::rand::random_range(-speed_range, speed_range)
        };

        Entity {
            point,
            direction,
            acceleration: direction / 10.0,
            color: Rgba::new_u8(255, nannou::rand::random_range(0, 128), nannou::rand::random_range(0, 128), 255)
        }
    }
}

fn init_audio_stream(app: &App, audio_model: Arc<Mutex<Audio>>, enabled: bool) -> Option<Stream<Arc<Mutex<Audio>>>> {
    if enabled {
        Some(
            app
            .audio
            .new_input_stream(audio_model, audio)
            .device(app.audio.default_input_device().unwrap())
            .build()
            .unwrap()
        )
    } else {
        None
    }
}


fn on_resize(_: &App, model: &mut Model, dimensions: Vector2) {
    model.window_dimensions = dimensions;
    println!("Resized: {:?}", dimensions);
}


fn audio(audio: &mut Arc<Mutex<Audio>>, buffer: &Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let fft = fft::find_frequencies(&buffer, sample_rate as f32, 1.1); // I don't know about the magnitute cutoff here... it may need to be tuned.

    let avg_amp = buffer
        .frames()
        .map(|frame| frame.iter().map(|c|f32::abs(*c)).sum::<f32>() / frame.iter().count() as f32) // Average amp over all channels for each frame
        .fold(0.0, |acc, f: f32| acc + f) / buffer.frames().count() as f32; // Average amplitude over all frames

    let mut audio = audio.lock().unwrap();
    audio.fft = fft;
    audio.avg_amp = avg_amp;
    audio.sample_rate = sample_rate;
//    dbg!(&audio);
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

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) -> Frame {

    // Prepare to draw.
    let draw = app.draw();

    draw.background()
        .color(DARK_BLUE);


    draw.line()
        .start(Point2::default())
        .end(Point2{
            x: app.time.sin() * 50.0,
            y: app.time.cos() * 50.0
        })
        .color(BLUE)
        .thickness(3.5);


    draw.ellipse()
        .x(model.point.x)
        .y(model.point.y)
        .width(2.0)
        .height(2.0)
        .color(DARK_BLUE);

    const B: f32 = 6.0;
    model.entities.iter().for_each(|e| {
        draw.ellipse()
            .x(e.point.x)
            .y(e.point.y)
            .width(B)
            .height(B)
            .color(e.color);
    });

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}