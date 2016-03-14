 use ::game::levels::OpCode;

pub trait Level {
    fn init(&mut self);
    fn get(&mut self, index: usize) -> OpCode;
}
