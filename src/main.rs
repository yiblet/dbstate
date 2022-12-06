#[macro_use]
extern crate anyhow;

use std::env;

use anyhow::Context;
use sqlx::PgPool;

mod action;
mod ddl;
mod ir;
mod schema;

async fn run() -> anyhow::Result<()> {
    let pool = PgPool::connect(
        &env::var("DATABASE_URL").context("missing DATABASE_URL environment flag")?,
    )
    .await?;

    let schema_all = action::get_all(&pool).await?;
    let ir_all = ir::get_all(&schema_all);

    for table in ir_all.tables.iter().filter(|t| !t.table.is_system_schema()) {
        match ddl::table(table) {
            Ok(table) => {
                println!("{}", table)
            }
            Err(e) => {
                eprintln!("error on table {}: {}", table.table.table_name, e);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    run().await
}
