

mod projects;
use projects::*;

use hilbert::*;

use clap::{App, Arg};
use nannou::event::{Update};
use nannou::Event;


fn main() {

    let matches = App::new("\"Art\"")
        .version("0.1.0")
        .author("Henry Zimmerman")
        .about("Art things?")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("An Integer that corresponds to the day to be shown.")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let day: usize = matches
        .value_of("day")
        .map(String::from)
        .map(|s| s.parse().unwrap())
        .unwrap();


    select(day)
}


fn select(i: usize) {
    match i {
        1 => {
            nannou::app(day_1::Model::init)
                .update(day_1::Model::update)
                .run();
        }
        2 => {
            nannou::app(day_2::Model::init)
                .update(day_2::Model::update)
                .run();
        }
        3 => {
            nannou::app(day_3::Model::init)
                .update(day_3::Model::update)
                .run();
        }
        4 => {
            nannou::app(day_4::Model::init)
                .update(day_4::Model::update)
                .run();
        }
        5 => {
            nannou::app(day_5::Model::init)
                .update(day_5::Model::update)
                .run();
        }
        6 => {
            nannou::app(day_6::Model::init)
                .update(day_6::Model::update)
                .run();
        }
        7 => {
            nannou::app(day_7::Model::init)
                .update(day_7::Model::update)
                .run();
        }
        8 => {
            nannou::app(day_8::Model::init)
                .update(day_8::Model::update)
                .run();
        }
        _ => panic!("Not implemented")
    }
}

#[allow(dead_code)]
pub enum MetaModel {
    Day1(day_1::Model),
    Day2(day_2::Model),
    Day3(day_3::Model),
    Day4(day_4::Model),
    Day5(day_5::Model),
    Day6(day_6::Model),
    Day7(day_7::Model),
    Day8(day_8::Model),
}

#[allow(dead_code)]
impl MetaModel {
    fn init(variant: usize, app: &nannou::App) -> Self {
        match variant {
            1 => day_1::Model::init(app).into(),
            2 => day_2::Model::init(app).into(),
            3 => day_3::Model::init(app).into(),
            4 => day_4::Model::init(app).into(),
            5 => day_5::Model::init(app).into(),
            6 => day_6::Model::init(app).into(),
            7 => day_7::Model::init(app).into(),
            8 => day_8::Model::init(app).into(),
            _ => panic!("unimplemented")
        }
    }

    fn update(app: &nannou::App, model: &mut MetaModel, update: Update) {
        match model {
            MetaModel::Day1(model) => day_1::Model::update(app, model, update),
            MetaModel::Day2(model) => day_2::Model::update(app, model, update),
            MetaModel::Day3(model) => day_3::Model::update(app, model, update),
            MetaModel::Day4(model) => day_4::Model::update(app, model, update),
            MetaModel::Day5(model) => day_5::Model::update(app, model, update),
            MetaModel::Day6(model) => day_6::Model::update(app, model, update),
            MetaModel::Day7(model) => day_7::Model::update(app, model, update),
            MetaModel::Day8(model) => day_8::Model::update(app, model, update)
        }
    }

    fn event(_app: &nannou::App, _model: &mut MetaModel, event: Event) {
        match event {
//            Event::WindowEvent {raw, ..} => {
//                match raw {
//                    WindowEvent::KeyPressed(VirtualKeyCode::RightArrow) => println!("yeet"),
//                    _ => {}
//                }
//            }
            _ => {}
        }
    }

    fn to_order(&self) -> usize {
        let discriminant = std::mem::discriminant(self);
        unsafe {
            std::mem::transmute(discriminant)
        }
    }
    fn next(&mut self, app: &nannou::App) {
        *self = Self::init(self.to_order(), app);
    }
}


impl Into<MetaModel> for day_1::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day1(self)
    }
}
impl Into<MetaModel> for day_2::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day2(self)
    }
}
impl Into<MetaModel> for day_3::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day3(self)
    }
}
impl Into<MetaModel> for day_4::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day4(self)
    }
}
impl Into<MetaModel> for day_5::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day5(self)
    }
}
impl Into<MetaModel> for day_6::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day6(self)
    }
}
impl Into<MetaModel> for day_7::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day7(self)
    }
}
impl Into<MetaModel> for day_8::Model {
    fn into(self) -> MetaModel {
        MetaModel::Day8(self)
    }
}
