
level! {
    name: Level2,
    instructions: LevelBuilder::new()
        .spawn(EnemyType::BasicEnemy)
        .wait_for(1)
        .spawn(EnemyType::BasicEnemy)
        .wait_for(1)
        .spawn(EnemyType::BasicEnemy)
        .wait_for(1)
        .spawn(EnemyType::BasicEnemy)
        .wait_for(1)
        .spawn(EnemyType::BasicEnemy)
        .wait_for(1)
        .spawn(EnemyType::BasicEnemy)
        .build()
}
