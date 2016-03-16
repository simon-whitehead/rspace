
use ::game::enemies::EnemyType;

#[derive(Clone, Debug)]
pub enum OpCode {
    None,
    SpawnEnemy(EnemyType),
    WaitFor(u8),

    EndLevel
}
