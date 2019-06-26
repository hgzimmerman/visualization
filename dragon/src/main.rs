use dragon::day_16;

fn main() {
    nannou::app(day_16::Model::init)
        .update(day_16::Model::update)
        .run();
}
