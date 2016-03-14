
use ::game::enemies::EnemyType;
use ::game::levels::OpCode;

pub struct LevelBuilder {
    instructions: Vec<OpCode>
}

impl LevelBuilder {
    pub fn new() -> LevelBuilder {
        LevelBuilder {
            instructions: Vec::new()
        }
    }

    pub fn spawn(mut self, enemy_type: EnemyType) -> Self {
        self.instructions.push(OpCode::SpawnEnemy(enemy_type));

        self
    }

    pub fn wait_for(mut self, wait_seconds: u8) -> Self {
        self.instructions.push(OpCode::WaitFor(wait_seconds));

        self
    }

    pub fn build(self) -> Vec<OpCode> {
        self.instructions
    }
}
