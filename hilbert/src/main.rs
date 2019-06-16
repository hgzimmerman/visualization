//use hilbert::day_6;
//use hilbert::day_7;
use hilbert::day_8;

fn main() {
    nannou::app(day_8::Model::init)
        .update(day_8::Model::update)
        .run();
}