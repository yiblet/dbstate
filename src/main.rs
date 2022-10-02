#[macro_use]
extern crate anyhow;

use std::env;

use sqlx::PgPool;

mod action;
mod ddl;
mod schema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ColumnsByTableIndex<'a> {
    table_schema: Option<&'a String>,
    table_name: Option<&'a String>,
    column_name: Option<&'a String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let tables = action::get_all_tables(&pool).await?;
    let columns = action::get_all_columns(&pool).await?;

    let columns_by_table: multimap::MultiMap<(Option<&String>, Option<&String>), &schema::Column> =
        columns
            .iter()
            .map(|c| ((c.table_schema.as_ref(), c.table_name.as_ref()), c))
            .collect();

    let empty_vec = vec![];
    for table in tables.iter().filter(|t| !t.is_system_schema()) {
        let cols =
            columns_by_table.get_vec(&(table.table_schema.as_ref(), Some(&table.table_name)));
        match ddl::table(table, cols.unwrap_or(&empty_vec)) {
            Ok(table) => {
                println!("{}", table)
            }
            Err(_) => {}
        }
    }

    Ok(())
}
