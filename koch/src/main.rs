//use koch::day_11;
use koch::day_12;

fn main() {
    nannou::app(day_12::Model::init)
        .update(day_12::Model::update)
        .run();
}
