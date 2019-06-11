use nannou::prelude::*;
use nannou::audio::buffer::Buffer;
use std::sync::{Mutex, Arc};

fn main() {
    nannou::app(model)
        .update(regular)
        .run();
}

struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
    _window: WindowId,
    point: Point2,
    entities: Vec<Entity>,
    _audio_stream: audio::Stream<Arc<Mutex<Audio>>>,
    audio: Arc<Mutex<Audio>>
}

struct Audio {
//    phase: f64,
    hz: f64, // TODO perform a fourier transform to get this boy.
    avg_amp: f32
}

#[derive(Default, Debug)]
struct Entity {
    point: Point2,
    direction: Point2,
}
impl Entity {

    fn new_random_direction(speed_range: f32, point: Point2) -> Self  {
        assert!(speed_range >= 0.0, "Speed range must be positive");
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
        .with_title("nannou")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .event(event) // The function that will be called when the window receives events.
        .build()
        .unwrap();

    let audio_model = Arc::new(Mutex::new(
        Audio {
            hz: 440.0,
            avg_amp: 0.0
        }
    ));
    let _audio_stream = app
        .audio
        .new_input_stream(audio_model.clone(), audio)
        .device(app.audio.default_input_device().unwrap())
        .build()
        .unwrap();

    Model {
        _window,
        point: Point2::default(),
        entities: vec![],
        _audio_stream,
        audio: audio_model
    }
}



fn audio(audio: &mut Arc<Mutex<Audio>>, buffer: &Buffer) {
//    let sample_rate = buffer.sample_rate() as f64;
//    let volume = 0.5;
    let avg_amp = buffer
        .frames()
        .map(|frame| frame.iter().map(|c|f32::abs(*c)).sum::<f32>() / frame.iter().count() as f32) // Average amp over all channels for each frame
        .fold(0.0, |acc, f: f32| acc + f) / buffer.frames().count() as f32; // Average amp over all frames

    let mut audio = audio.lock().unwrap();
    audio.avg_amp = avg_amp;
//    println!("{:?}", buffer);
}

// Handle events related to the window and update the model if necessary
fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MouseMoved(point) => {
            model.point = point;
        }
        WindowEvent::MousePressed(_mouse_button) => {
            (0..500).for_each(|_| {
                let entity = Entity::new_random_direction(30.0, model.point);
                model.entities.push(entity);
            });

            let audio = model.audio.lock().unwrap().avg_amp;
            println!("{}", audio);
        }
        _ => println!("{:?}", event)
    }
}

fn regular(_app: &App, model: &mut Model, update: Update) {
    let since_last: f32 = update.since_last.secs() as f32;

    let avg_audio_amp = model.audio.lock().unwrap().avg_amp;
    let multiplier = if avg_audio_amp > 0.01{
       avg_audio_amp * 100.0
    } else {
        1.0
    };

    model.entities.iter_mut().for_each(|e|{
        e.point.x = e.point.x + e.direction.x * since_last * multiplier;
        e.point.y = e.point.y + e.direction.y * since_last * multiplier;
    });
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) -> Frame {

    // Prepare to draw.
    let draw = app.draw();


    // Clear the background to purple.
    draw.background()
        .color(LIGHT_PURPLE);

    // Draw a blue ellipse with default size and position.
    draw.ellipse()
        .x(model.point.x)
        .y(model.point.y)
        .width(20.0)
        .height(20.0)
        .color(DARK_BLUE);

    model.entities.iter().for_each(|e| {
        draw.ellipse()
            .x(e.point.x)
            .y(e.point.y)
            .width(10.0)
            .height(10.0)
            .color(DARK_RED);
    });

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}

// TODO I'm going to want a ring buffer to limit the number of entities at a time, while still allowing a source to spawn them in relevant locations.

pub struct RingBuffer<T> {
    index: usize,
    buf: Vec<Option<T>>
}

pub struct RingBufferIterator<T> {
    ring_buffer: RingBuffer<T>,
    pos: usize
}

impl <T> Iterator for RingBufferIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ring_buffer.buf.len() ==
    }
}

impl <T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        RingBuffer {
            index: 0,
            buf: vec![None, size]
        }
    }

    pub fn push(&mut self, value: T){
        self.buf[index] = Some(value);
        index += 1 % (self.buf.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let v = self.buf[index];
        if index == 0 {
            index = self.buf.len() - 1;
        } else {
            index -= 1;
        }
        v
    }
}
