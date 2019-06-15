use nannou::prelude::*;
use std::num::Wrapping;
use rand::thread_rng;
use rand::distributions::{UnitCircle, Distribution};
use rand::seq::SliceRandom;
use std::cmp::Ordering;

pub struct Model {
    _window: WindowId,
    window_dimensions: Vector2,
    frame_counter: Wrapping<u64>,
    circles: Circ,
    circ_buffer: Vec<(f32, Point2, Rgba)>
}

// TODO consider adding a value that allows every circle to be rotated slowly
// (Rotate the children)
#[derive(Default)]
pub struct Circ {
    center: Point2<f32>,
    radius: f32,
    color: Rgba, // consider having a child color as well.
    inner: Vec<Circ>,
}

impl Circ {

    fn color_pallette() -> [Rgba; 14] {
        [
            DARK_RED,
            GREEN,
            DARK_CHARCOAL,
            LIGHT_CHARCOAL,
            LIGHT_ORANGE,
            DARK_PURPLE,
            LIGHT_BLUE,
            YELLOW,
            DARK_BLUE,
            LIGHT_BROWN,
            LIGHT_RED,
            DARK_ORANGE,
            DARK_GREEN,
            GRAY
        ]
    }
    // Needs to be top-down -> to ensure that radius matches screen height
    // Although that could just be scaled up or down as needed
    fn pack_circle(&mut self) {
        // If it is too small, don't look at inner.
        if self.radius > 6.0 {
            // Maybe, generate 6 coordinates, select 3 that are farthest away? 1/3 the size of parent
            // Then generate 12 more at 2/3rds the size of siblings, take as many that don't collide?
            let mut rng = thread_rng();
            let circle = UnitCircle::new();

            let large_inner_radius = self.radius / 3.0;
            let small_inner_radius = self.radius / 5.0;
            let tiny_inner_radius = self.radius / 7.0;
            let micro_inner_radius = self.radius / 11.0;
            let max_large_inner_unit_circle_radius = self.radius - large_inner_radius;
            let min_large_inner_unit_circle_radius = small_inner_radius;

            let mut colors = Circ::color_pallette();
            colors.shuffle(&mut rng);

            let inner: Vec<Circ> = (0..)
                .map(|_| {
                    let v: Point2<f64> = circle.sample(&mut rng).into();
                    let v: Point2<f32> = v.map(|a| a as f32);
                    (v * nannou::rand::random_range(min_large_inner_unit_circle_radius, max_large_inner_unit_circle_radius)) + self.center
                })
                .take(750)
                .fold(Vec::with_capacity(4), |mut acc: Vec<Circ>, pt| {

                    // If the new circle doesn't collide
                    if let Some(mut new_circ) = if acc.iter().all(|c| (c.center - pt).magnitude() > c.radius + large_inner_radius ) {
                        Some(Circ {
                            center: pt,
                            radius: large_inner_radius,
                            color: colors[0],
                            inner: vec![]
                        })
                    } else if acc.iter().all(|c| (c.center - pt).magnitude() > c.radius + small_inner_radius ) {
                        Some(Circ {
                            center: pt,
                            radius: small_inner_radius,
                            color: colors[1],
                            inner: vec![]
                        })
                    } else if acc.iter().all(|c| (c.center - pt).magnitude() > c.radius + tiny_inner_radius ) {
                        Some(Circ {
                            center: pt,
                            radius: tiny_inner_radius,
                            color: colors[2],
                            inner: vec![]
                        })
                    } else if micro_inner_radius > 10.0
                        && acc.iter().all(|c| (c.center - pt).magnitude() > c.radius + micro_inner_radius )
                        && acc.len() > 6 {
                        Some(Circ {
                            center: pt,
                            radius: micro_inner_radius,
                            color: colors[3],
                            inner: vec![]
                        })
                    } else {
                        None
                    } {
                        // lol, this is a valid expression
                        new_circ.pack_circle();
                        acc.push(new_circ);
                    }

                    acc
                });



            self.inner = inner;
        }
    }

}

impl Model {
    pub fn init(app: &App) -> Model {
        let _window = app
            .new_window()
            .with_dimensions(512, 512)
            .with_title("day 3")
            .view(view) // The function that will be called for presenting graphics to a frame.
            .event(event) // The function that will be called when the window receives events.
            .resized(on_resize)
            .build()
            .unwrap();

        let mut c = Circ::default();
        c.color = RED;

        Model {
            _window,
            window_dimensions: Vector2::default(),
            frame_counter: Wrapping(0),
            circles: c,
            circ_buffer: Vec::new()
        }
    }

    pub fn update(_app: &App, model: &mut Model, _update: Update) {
        model.frame_counter += Wrapping(1);

        model.circles.radius = model.window_dimensions.y / 2.0;
        model.circles.color = RED;
    }
}




fn setup_circ_buffer(buf: &mut Vec<(f32, Point2, Rgba)>, circle: &Circ) {

    fn dump_circ_info(circle: &Circ, buffer: &mut Vec<(f32, Point2, Rgba)>) {
        buffer.push((circle.radius, circle.center, circle.color));
        circle.inner.iter().for_each(|i| dump_circ_info(i, buffer))
    }

    dump_circ_info(circle, buf);
    buf.sort_by(|x, y| {
      x.0.partial_cmp( &y.0).unwrap_or_else(|| Ordering::Greater).reverse()
    });
}

fn on_resize(_: &App, model: &mut Model, dimensions: Vector2) {
    model.window_dimensions = dimensions;
    model.circles.radius = model.window_dimensions.y / 2.0;
    model.circles.inner.clear();
    model.circles.pack_circle();


    // fill up the circ buffer
    model.circ_buffer.clear();
    setup_circ_buffer(&mut model.circ_buffer, &model.circles);

    println!("Resized: {:?}", dimensions);
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
            model.circles.inner.clear();
            model.circles.pack_circle();
            model.circ_buffer.clear();
            setup_circ_buffer(&mut model.circ_buffer, &model.circles);
        }
        WindowEvent::KeyPressed(Key::Q) => {
            std::process::exit(0); // Q -> exit program
        }
        _ => println!("{:?}", event)
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    frame.clear(LIGHT_YELLOW);



    model.circ_buffer
        .iter()
        .take(model.frame_counter.0 as usize)
        .for_each(|c| {
            draw.ellipse()
                .xy(c.1)
                .radius(c.0)
                .color(c.2);
        });


    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    // Return the drawn frame.
    frame
}
