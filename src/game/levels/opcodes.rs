
use ::game::enemies::EnemyType;

#[derive(Clone)]
pub enum OpCode {
    SpawnEnemy(EnemyType),
    WaitFor(u8),

    EndLevel
}
