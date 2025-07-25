use sqlx::PgPool;

pub async fn seed_rooms(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM questions").execute(pool).await?;
    sqlx::query!("DELETE FROM rooms").execute(pool).await?;

    let rooms = vec![
        ("Room 1", "Description for Room 1"),
        ("Room 2", "Description for Room 2"),
        ("Room 3", "Description for Room 3"),
        ("Room 4", "Description for Room 4"),
        ("Room 5", "Description for Room 5"),
    ];

    for (name, description) in rooms {
        sqlx::query!(
            "INSERT INTO rooms (name, description) VALUES ($1, $2)",
            name,
            description
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
