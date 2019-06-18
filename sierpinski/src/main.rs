use sierpinski::day_10;

fn main() {
    nannou::app(day_10::Model::init)
        .update(day_10::Model::update)
        .run();
}
