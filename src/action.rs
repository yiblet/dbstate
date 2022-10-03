use crate::schema;

pub async fn get_all_tables(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<schema::Table>> {
    let tables: Vec<schema::Table> = sqlx::query_as(r#"select* from information_schema.tables"#)
        .fetch_all(pool)
        .await?;

    Ok(tables)
}

pub async fn get_all_views(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<schema::View>> {
    let views: Vec<schema::View> = sqlx::query_as(r#"select* from information_schema.views"#)
        .fetch_all(pool)
        .await?;

    Ok(views)
}

pub async fn get_all_columns(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<schema::Column>> {
    let columns: Vec<schema::Column> = sqlx::query_as(r#"select* from information_schema.columns"#)
        .fetch_all(pool)
        .await?;

    Ok(columns)
}

pub async fn get_all_table_constraints(
    pool: &sqlx::postgres::PgPool,
) -> anyhow::Result<Vec<schema::TableConstraint>> {
    let table_constraints: Vec<schema::TableConstraint> =
        sqlx::query_as(r#"select* from information_schema.table_constraints"#)
            .fetch_all(pool)
            .await?;

    Ok(table_constraints)
}
