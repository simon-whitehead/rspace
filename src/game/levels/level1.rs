
use ::game::levels::{Level, LevelBuilder, OpCode};
use ::game::enemies::EnemyType;

pub struct Level1 {
    pub instructions: Vec<OpCode>
}

impl Level1 {
    pub fn new() -> Level1 {
        Level1 {
            instructions: Vec::new()
        }
    }
}

impl Level for Level1 {
    fn init(&mut self) {
        self.instructions = LevelBuilder::new()
            .spawn(EnemyType::BasicEnemy)
            .spawn(EnemyType::BasicEnemy)
            .spawn(EnemyType::BasicEnemy)
            .wait_for(5)
            .spawn(EnemyType::BasicEnemy)
            .build();
    }

    fn get(&mut self, index: usize) -> OpCode {
        if index >= self.instructions.len() {
            OpCode::EndLevel
        } else {
            self.instructions[index].clone()
        }
    }
}
