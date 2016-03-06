
use ::engine::context::Context;

pub trait Entity {
    fn render(&mut self, context: &mut Context, elapsed: f64);
    fn process(&mut self, context: &mut Context, elapsed: f64);
}
