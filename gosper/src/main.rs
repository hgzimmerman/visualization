use gosper::day_9;

fn main() {
    nannou::app(day_9::Model::init)
        .update(day_9::Model::update)
        .run();
}
