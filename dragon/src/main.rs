use dragon::day_15;

fn main() {
    nannou::app(day_15::Model::init)
        .update(day_15::Model::update)
        .run();
}
