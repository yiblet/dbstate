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
        table.table_schema.as_ref().map(|s| s.as_str()),
        &table.table_name,
    ));
    res.push_str(" (");

    let mut cols: Vec<_> = table.columns.iter().cloned().collect();
    cols.sort_by_key(|col| (&col.column_name, col.ordinal_position));

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
    let mut res = identifier(&col.column_name);

    let data_type = match (&col.column_default, col.data_type.as_str()) {
        (_, "USER-DEFINED") => Err(anyhow!("unimplmented data type"))?,
        (_, "ARRAY") => {
            let element_type = col
                .element_type
                .ok_or_else(|| anyhow!("missing element type for array"))?;

            let data_type = element_type
                .data_type
                .as_ref()
                .ok_or_else(|| anyhow!("missing data type for array"))?;

            format!("[]{}", data_type)
        }
        (Some(default), "integer")
            if is_serial_expression(&col.table_name, &col.column_name, default) =>
        {
            "serial".to_owned()
        }
        (_, data_type) => data_type.to_owned(),
    };

    write!(&mut res, " {}", data_type)?;
    if col.is_nullable.unwrap_or_default().is_no() {
        write!(&mut res, " NOT NULL")?;
    }

    let is_serial = data_type == "serial";
    match (is_serial, col.column_default.as_ref()) {
        (false, Some(expr)) => write!(&mut res, " DEFAULT {}", expr)?,
        _ => {}
    };

    Ok(res)
}
