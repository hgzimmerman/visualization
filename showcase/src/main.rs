

mod projects;
use projects::*;

use hilbert::*;

use clap::{App, Arg};


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

    select(day);
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

pub enum MetaModel {
    Day1(day_1::Model),
    Day2(day_2::Model),
    Day3(day_3::Model),

}



