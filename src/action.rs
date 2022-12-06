use crate::schema;

pub async fn get_all_tables(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<schema::Table>> {
    let tables: Vec<schema::Table> = sqlx::query_as(r#"select * from information_schema.tables"#)
        .fetch_all(pool)
        .await?;

    Ok(tables)
}

pub async fn get_all_views(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<schema::View>> {
    let views: Vec<schema::View> = sqlx::query_as(r#"select * from information_schema.views"#)
        .fetch_all(pool)
        .await?;

    Ok(views)
}

pub async fn get_all_columns(pool: &sqlx::postgres::PgPool) -> anyhow::Result<Vec<schema::Column>> {
    let columns: Vec<schema::Column> =
        sqlx::query_as(r#"select * from information_schema.columns"#)
            .fetch_all(pool)
            .await?;

    Ok(columns)
}

pub async fn get_all_table_constraints(
    pool: &sqlx::postgres::PgPool,
) -> anyhow::Result<Vec<schema::TableConstraint>> {
    let table_constraints: Vec<schema::TableConstraint> =
        sqlx::query_as(r#"select * from information_schema.table_constraints"#)
            .fetch_all(pool)
            .await?;

    Ok(table_constraints)
}

pub async fn get_all_constraint_column_usage(
    pool: &sqlx::postgres::PgPool,
) -> anyhow::Result<Vec<schema::ConstraintColumnUsage>> {
    let table_constraints: Vec<schema::ConstraintColumnUsage> =
        sqlx::query_as(r#"select * from information_schema.constraint_column_usage"#)
            .fetch_all(pool)
            .await?;

    Ok(table_constraints)
}

pub async fn get_all_constraint_table_usage(
    pool: &sqlx::postgres::PgPool,
) -> anyhow::Result<Vec<schema::ConstraintTableUsage>> {
    let table_constraints: Vec<schema::ConstraintTableUsage> =
        sqlx::query_as(r#"select * from information_schema.constraint_table_usage"#)
            .fetch_all(pool)
            .await?;

    Ok(table_constraints)
}

pub async fn get_all_element_types(
    pool: &sqlx::postgres::PgPool,
) -> anyhow::Result<Vec<schema::ElementType>> {
    let rows: Vec<schema::ElementType> =
        sqlx::query_as(r#"select * from information_schema.element_types"#)
            .fetch_all(pool)
            .await?;

    Ok(rows)
}

pub async fn get_all_check_constraints(
    pool: &sqlx::postgres::PgPool,
) -> anyhow::Result<Vec<schema::CheckConstraint>> {
    let rows: Vec<schema::CheckConstraint> =
        sqlx::query_as(r#"select * from information_schema.check_constraints"#)
            .fetch_all(pool)
            .await?;

    Ok(rows)
}

pub async fn get_all(pool: &sqlx::postgres::PgPool) -> anyhow::Result<schema::All> {
    let mut fetch_start_time = None;
    if log::log_enabled!(log::Level::Info) {
        fetch_start_time = Some(std::time::SystemTime::now());
    }
    let (
        tables_res,
        columns_res,
        views_res,
        table_constraints_res,
        constraint_column_usage_res,
        constraint_table_usage_res,
        element_types_res,
        check_constraints_res,
    ) = futures::join!(
        get_all_tables(&pool),
        get_all_columns(&pool),
        get_all_views(&pool),
        get_all_table_constraints(&pool),
        get_all_constraint_column_usage(&pool),
        get_all_constraint_table_usage(&pool),
        get_all_element_types(&pool),
        get_all_check_constraints(&pool),
    );
    if let Some(dur) = fetch_start_time.and_then(|s| s.elapsed().ok()) {
        log::info!(
            "get all fetched successfully elapsed: {}ms",
            dur.as_micros() as f64 / 1e3
        );
    }

    let (
        tables,
        columns,
        views,
        table_constraints,
        constraint_column_usage,
        constraint_table_usage,
        element_types,
        check_constraints,
    ) = (
        tables_res?,
        columns_res?,
        views_res?,
        table_constraints_res?,
        constraint_column_usage_res?,
        constraint_table_usage_res?,
        element_types_res?,
        check_constraints_res?,
    );

    let res = schema::All {
        tables,
        columns,
        views,
        table_constraints,
        constraint_column_usage,
        constraint_table_usage,
        element_types,
        check_constraints,
    };

    Ok(res)
}
