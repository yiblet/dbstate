#[macro_use]
extern crate anyhow;

use std::env;

use sqlx::PgPool;

mod action;
mod ddl;
mod ir;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

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
