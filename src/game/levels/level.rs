 use ::game::levels::OpCode;

pub trait Level {
    fn init(&mut self);
    fn get(&mut self, index: usize) -> OpCode;
    fn get_instruction_count(&self) -> usize;
}

#[macro_export]
macro_rules! level {
    (
        name: $name:ident,
        instructions: $instructions:expr
    ) => {

        use ::game::levels::{Level, LevelBuilder, OpCode};
        use ::game::enemies::EnemyType;

        pub struct $name {
            pub instructions: Vec<OpCode>
        }

        impl $name {
            pub fn new() -> $name {
                let mut l = $name {
                    instructions: Vec::new()
                };

                l.init();

                l
            }
        }

        impl Level for $name {
            fn init(&mut self) {
                self.instructions = $instructions;
            }

            fn get(&mut self, index: usize) -> OpCode {
                if index >= self.instructions.len() {
                    OpCode::EndLevel
                } else {
                    self.instructions[index].clone()
                }
            }

            fn get_instruction_count(&self) -> usize {
                self.instructions.len()
            }
        }
    }
}
