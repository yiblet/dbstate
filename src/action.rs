use crate::schema::{Column, Table, View};

pub async fn get_all_tables(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<Table>> {
    let tables: Vec<Table> = sqlx::query_as(r#"select* from information_schema.tables"#)
        .fetch_all(pool)
        .await?;

    Ok(tables)
}

pub async fn get_all_views(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<View>> {
    let views: Vec<View> = sqlx::query_as(r#"select* from information_schema.views"#)
        .fetch_all(pool)
        .await?;

    Ok(views)
}

pub async fn get_all_columns(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<Column>> {
    let columns: Vec<Column> = sqlx::query_as(r#"select* from information_schema.columns"#)
        .fetch_all(pool)
        .await?;

    Ok(columns)
}
