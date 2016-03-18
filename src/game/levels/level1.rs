
level! {
    name: Level1,
    instructions: LevelBuilder::new()
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .spawn(EnemyType::BasicEnemy)
        .build()
}
