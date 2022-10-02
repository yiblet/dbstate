use std::fmt::Write;

use crate::schema::{self, Column};

pub fn table(table: &schema::Table, columns: &[&Column]) -> anyhow::Result<String> {
    if table.table_type != Some("BASE TABLE".to_string()) {
        Err(anyhow!("cannot handle table type: {:?}", table.table_type))?
    }
    let mut res: String = "CREATE TABLE ".to_owned();

    res.push_str(&table_identifier(
        table.table_schema.as_ref().map(|s| s.as_str()),
        &table.table_name,
    ));
    res.push_str(" (");

    let mut cols: Vec<_> = columns.iter().cloned().collect();
    cols.sort_by_key(|col| (&col.column_name, col.ordinal_position));

    let mut seen = 0;
    for col in cols.into_iter() {
        let val = match column(col) {
            Ok(val) => val,
            Err(_) => continue,
        };
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

fn column(col: &schema::Column) -> anyhow::Result<String> {
    let mut res = col.column_name.clone();

    if col.data_type == "ARRAY" || col.data_type == "USER-DEFINED" {
        Err(anyhow!("unimplmented data type"))?
    };

    write!(&mut res, " {}", col.data_type)?;
    if col.is_nullable.unwrap_or_default().is_no() {
        write!(&mut res, " NOT NULL")?;
    }

    if let Some(default_expr) = col.column_default.as_ref() {
        write!(&mut res, " DEFAULT {}", default_expr)?;
    }

    Ok(res)
}
