use peano::day_13;

fn main() {
    nannou::app(day_13::Model::init)
        .update(day_13::Model::update)
        .run();
}
