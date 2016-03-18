
level! {
    name: Level2,
    instructions: LevelBuilder::new()
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .wait_for(2)
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .build()
}
