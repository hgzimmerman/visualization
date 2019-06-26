//use hilbert::day_6;
//use hilbert::day_7;
use hilbert::day_17;

fn main() {
    nannou::app(day_17::Model::init)
        .update(day_17::Model::update)
        .run();
}