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

fn collect_by_key<'a, D, K, I, F>(iter: I, func: F) -> multimap::MultiMap<K, &'a D>
where
    K: std::hash::Hash + std::cmp::Eq,
    I: Iterator<Item = &'a D> + 'a,
    F: Fn(&'a D) -> K,
{
    iter.map(|data| (func(data), data)).collect()
}

pub fn get_slice<'a, Q: ?Sized, K, V>(map: &'a multimap::MultiMap<K, V>, k: &Q) -> &'a [V]
where
    K: std::borrow::Borrow<Q> + Eq + std::hash::Hash,
    Q: Eq + std::hash::Hash,
{
    map.get_vec(k).map(Vec::as_slice).unwrap_or_default()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let (tables_res, columns_res, table_constraints_res) = futures::join!(
        action::get_all_tables(&pool),
        action::get_all_columns(&pool),
        action::get_all_table_constraints(&pool),
    );
    let (tables, columns, table_constraints) = (tables_res?, columns_res?, table_constraints_res?);

    let columns_by_table: multimap::MultiMap<(Option<&String>, Option<&String>), &schema::Column> =
        collect_by_key(columns.iter(), |c| {
            (c.table_schema.as_ref(), c.table_name.as_ref())
        });

    let table_constraints_by_table: multimap::MultiMap<
        (Option<&String>, Option<&String>),
        &schema::TableConstraint,
    > = collect_by_key(table_constraints.iter(), |c| {
        (c.table_schema.as_ref(), c.table_name.as_ref())
    });

    for table in tables.iter().filter(|t| !t.is_system_schema()) {
        let key = (table.table_schema.as_ref(), Some(&table.table_name));

        let columns = get_slice(&columns_by_table, &key);
        let table_constraints = get_slice(&table_constraints_by_table, &key);

        match ddl::table(table, columns, table_constraints) {
            Ok(table) => {
                println!("{}", table)
            }
            Err(_) => {}
        }
    }

    Ok(())
}
