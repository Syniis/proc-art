use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}
fn model(app: &App) -> Model {
    Model {}
}

fn update(app: &App, model: &mut Model, update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}
