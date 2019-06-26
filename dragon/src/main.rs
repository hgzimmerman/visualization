use dragon::day_14;

fn main() {
    nannou::app(day_14::Model::init)
        .update(day_14::Model::update)
        .run();
}
