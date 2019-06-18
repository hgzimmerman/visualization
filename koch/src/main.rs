use koch::day_11;

fn main() {
    nannou::app(day_11::Model::init)
        .update(day_11::Model::update)
        .run();
}
