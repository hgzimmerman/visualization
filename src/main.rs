

pub mod ring_buffer;
mod fft;

mod projects;
use projects::*;


fn main() {
    select(4);
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
        _ => panic!("Not implemented")
    }
}



