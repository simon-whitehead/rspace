
level! {
    name: Level1,
    instructions: LevelBuilder::new()
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .wait_for(5)
        .spawn(EnemyType::BasicEnemy)
        .build()
}
