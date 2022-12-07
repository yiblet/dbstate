use std::{collections::btree_set, fmt::Write};

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

    let mut inserted_lines = 0;
    let mut append = |data: &str| {
        if inserted_lines > 0 {
            res.push_str(",");
        }
        res.push_str("\n\t");
        res.push_str(data);
        inserted_lines += 1;
    };

    for col in cols.iter() {
        let val = column(&col)?;
        append(&val);
    }

    let is_non_nullable_check_constraint = create_is_non_nullable_constraint(&cols);
    for constraint in table
        .table_constraints
        .as_slice()
        .iter()
        .filter(|c| !is_non_nullable_check_constraint(c))
    {
        match table_constraint(constraint)? {
            Some(val) => append(&val),
            None => continue,
        }
    }
    drop(append);

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

fn table_constraint(item: &ir::TableConstraint<'_>) -> anyhow::Result<Option<String>> {
    match item.constraint_type.as_str() {
        "PRIMARY KEY" => {
            let cols = join(
                item.columns.iter().map(|c| identifier(&c.column_name)),
                ", ",
            );
            let res = format!("PRIMARY KEY ({})", cols);
            Ok(Some(res))
        }
        "UNIQUE" => {
            let cols = join(
                item.columns.iter().map(|c| identifier(&c.column_name)),
                ", ",
            );
            let res = format!("UNIQUE ({})", cols);
            Ok(Some(res))
        }
        _ => {
            log::warn!(
                "unexepected constraint type {} {:?}",
                item.constraint_type.as_str(),
                item
            );

            Ok(None)
        }
    }
}

fn create_is_non_nullable_constraint(
    cols: &[ir::Column<'_>],
) -> impl Fn(&ir::TableConstraint<'_>) -> bool
where
{
    let non_nullable_check_constraints: btree_set::BTreeSet<_> = cols
        .iter()
        .filter_map(|c| match non_nullable_check_constraint(c) {
            Ok(c) => c,
            Err(err) => {
                log::warn!("expected_check_constraint failed {}", err);
                None
            }
        })
        .collect();

    if log::log_enabled!(log::Level::Info) {
        log::info!(
            "valid non nullable check clauses {:?}",
            &non_nullable_check_constraints
        )
    }

    return move |constraint: &ir::TableConstraint<'_>| -> bool {
        if constraint.constraint_type != "CHECK" || constraint.check_constraints.is_empty() {
            return false;
        }
        constraint.check_constraints.iter().all(|c| {
            if !non_nullable_check_constraints.contains(&c.check_clause) {
                log::warn!(
                    "check clause {:?} is not a nullable check clause",
                    c.check_clause
                )
            }
            true
        })
    };
}

fn non_nullable_check_constraint(col: &ir::Column<'_>) -> anyhow::Result<Option<String>> {
    let is_nullable = col.is_nullable.map_or(true, |x| x.is_yes());
    let res = if !is_nullable {
        Some(format!("{} IS NOT NULL", &col.column_name))
    } else {
        None
    };

    Ok(res)
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

fn join<'a, I, S>(iter: I, sep: &'a str) -> String
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    iter.enumerate().fold(String::new(), |mut acc, (idx, id)| {
        if idx != 0 {
            acc.push_str(sep);
        };
        acc.push_str(id.as_ref());
        acc
    })
}
