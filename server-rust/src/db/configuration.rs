use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{db::seeder::seed_rooms, env::get_env_var};

pub async fn get_configuration() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = get_env_var("DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    seed_rooms(&pool).await?;

    Ok(pool)
}
