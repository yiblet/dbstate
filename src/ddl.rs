use std::fmt::Write;

use crate::ir;

pub fn table(table: &ir::Table<'_>) -> anyhow::Result<String> {
    if table.table.table_type != Some("BASE TABLE".to_string()) {
        Err(anyhow!(
            "cannot handle table type: {:?}",
            table.table.table_type
        ))?
    }
    let mut res: String = "CREATE TABLE ".to_owned();

    res.push_str(&table_identifier(
        table.table.table_schema.as_ref().map(|s| s.as_str()),
        &table.table.table_name,
    ));
    res.push_str(" (");

    let mut cols: Vec<_> = table.columns.iter().cloned().collect();
    cols.sort_by_key(|col| (&col.column.column_name, col.column.ordinal_position));

    let mut seen = 0;
    for col in cols.into_iter() {
        let val = column(&col)?;
        if seen == 0 {
            res.push_str("\n\t")
        } else {
            res.push_str(",\n\t")
        }
        res.push_str(&val);
        seen += 1;
    }

    res.push_str("\n);");

    Ok(res)
}

fn schema(data: Option<&str>) -> String {
    let schema = data
        .filter(|x| *x != "public")
        .map(identifier)
        .unwrap_or_default();
    schema
}

fn table_identifier(table_schema: Option<&str>, table_name: &str) -> String {
    let mut res = schema(table_schema);
    if res != "" {
        res.push_str(".")
    }
    res.push_str(&identifier(table_name));
    res
}

// TODO be less aggressive with identifers
fn identifier(data: &str) -> String {
    format!("\"{}\"", data)
}

fn is_serial_expression(table_name: &str, column_name: &str, default_expression: &str) -> bool {
    return format!("nextval('{}_{}_seq'::regclass)", table_name, column_name)
        == default_expression;
}

fn column(col: &ir::Column<'_>) -> anyhow::Result<String> {
    let mut res = identifier(&col.column.column_name);

    if col.column.data_type == "ARRAY" || col.column.data_type == "USER-DEFINED" {
        Err(anyhow!("unimplmented data type"))?
    };

    let (is_serial, data_type) = match (
        &col.column.table_name,
        &col.column.column_default,
        col.column.data_type.as_str(),
    ) {
        (table_name, Some(default), "integer")
            if is_serial_expression(table_name, &col.column.column_name, default) =>
        {
            (true, "serial")
        }
        (_, _, data_type) => (false, data_type),
    };

    write!(&mut res, " {}", data_type)?;
    if col.column.is_nullable.unwrap_or_default().is_no() {
        write!(&mut res, " NOT NULL")?;
    }

    match (is_serial, col.column.column_default.as_ref()) {
        (false, Some(expr)) => write!(&mut res, " DEFAULT {}", expr)?,
        _ => {}
    };

    Ok(res)
}
