use std::env;

use sqlx::PgPool;

mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let tables = schema::table::get_all_tables(&pool).await;

    println!("{:?}", tables);

    Ok(())
}
