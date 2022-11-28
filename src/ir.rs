use std::rc::Rc;

use crate::schema;

#[derive(Debug, Clone)]
pub struct All<'a> {
    pub tables: Rc<Vec<Table<'a>>>,
}

#[derive(Debug, Clone)]
pub struct Table<'a> {
    pub table: &'a schema::Table,
    pub table_constraints: Rc<Vec<TableConstraint<'a>>>,
    pub columns: Rc<Vec<Column<'a>>>,
}

impl<'a> std::ops::Deref for Table<'a> {
    type Target = &'a schema::Table;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

#[derive(Debug, Clone)]
pub struct Column<'a> {
    pub column: &'a schema::Column,
}

impl<'a> std::ops::Deref for Column<'a> {
    type Target = &'a schema::Column;

    fn deref(&self) -> &Self::Target {
        &self.column
    }
}

#[derive(Debug, Clone)]
pub struct TableConstraint<'a> {
    pub table_constraint: &'a schema::TableConstraint,
    pub columns: Rc<Vec<Column<'a>>>,
    // should always be just one table
    pub tables: Rc<Vec<&'a schema::Table>>, //referencing the schema (not the ir::Table) since using the ir table would cause a circular reference.
}

impl<'a> std::ops::Deref for TableConstraint<'a> {
    type Target = &'a schema::TableConstraint;

    fn deref(&self) -> &Self::Target {
        &self.table_constraint
    }
}

fn get_all_tables<'a>(
    all: &'a schema::All,
    columns: &[Column<'a>],
    table_constraints: &[TableConstraint<'a>],
) -> Vec<Table<'a>> {
    let columns_by_table = collect_by_key(columns.iter(), |c| {
        (c.column.table_schema.as_ref(), &c.column.table_name)
    });
    let table_constraints_by_table = collect_by_key(table_constraints.iter(), |c| {
        (
            c.table_constraint.table_schema.as_ref(),
            &c.table_constraint.table_name,
        )
    });

    all.tables
        .iter()
        .map(|table| {
            let columns: Vec<_> = columns_by_table
                .get_vec(&(table.table_schema.as_ref(), &table.table_name))
                .iter()
                .flat_map(|v| v.iter())
                .cloned()
                .cloned()
                .collect();

            let table_constraints: Vec<_> = table_constraints_by_table
                .get_vec(&(table.table_schema.as_ref(), &table.table_name))
                .iter()
                .flat_map(|v| v.iter())
                .cloned()
                .cloned()
                .collect();

            Table {
                table,
                columns: Rc::new(columns),
                table_constraints: Rc::new(table_constraints),
            }
        })
        .collect()
}

fn get_all_columns<'a>(all: &'a schema::All) -> Vec<Column<'a>> {
    all.columns.iter().map(|column| Column { column }).collect()
}

fn get_all_table_constraints<'a>(
    all: &'a schema::All,
    columns: &[Column<'a>],
) -> Vec<TableConstraint<'a>> {
    let schema_tables_by_table = collect_by_key(all.tables.iter(), |c| {
        (c.table_schema.as_ref(), &c.table_name)
    });
    let column_by_table_column = collect_by_key(columns.iter(), |c| {
        (
            c.column.table_schema.as_ref(),
            &c.column.table_name,
            &c.column.column_name,
        )
    });

    let schema_constraint_column_usage_by_table_constraints =
        collect_by_key(all.constraint_column_usage.iter(), |c| {
            (c.constraint_schema.as_ref(), &c.constraint_name)
        });
    let schema_constraint_table_usage_by_table_constraints =
        collect_by_key(all.constraint_table_usage.iter(), |c| {
            (c.constraint_schema.as_ref(), &c.constraint_name)
        });

    all.table_constraints
        .iter()
        .map(|table_constraint| -> TableConstraint<'_> {
            let columns: Vec<_> = schema_constraint_column_usage_by_table_constraints
                .get_vec(&(
                    table_constraint.constraint_schema.as_ref(),
                    &table_constraint.constraint_name,
                ))
                .iter()
                .flat_map(|v| v.iter())
                .filter_map(|usage| {
                    let column = column_by_table_column
                        .get(&(
                            usage.table_schema.as_ref(),
                            &usage.table_name,
                            &usage.column_name,
                        ))
                        .cloned();
                    if column.is_none() {
                        eprintln!(
                            "cannot find column {} in table {}",
                            usage.column_name, usage.table_name
                        )
                    }
                    column.cloned()
                })
                .collect();

            let tables: Vec<_> = schema_constraint_table_usage_by_table_constraints
                .get_vec(&(
                    table_constraint.constraint_schema.as_ref(),
                    &table_constraint.constraint_name,
                ))
                .iter()
                .flat_map(|v| v.iter())
                .filter_map(|usage| {
                    let table = schema_tables_by_table
                        .get(&(usage.table_schema.as_ref(), &usage.table_name))
                        .cloned();
                    if table.is_none() {
                        eprintln!("cannot find table {}", usage.table_name)
                    }
                    table
                })
                .collect();

            TableConstraint {
                table_constraint,
                columns: Rc::new(columns),
                tables: Rc::new(tables),
            }
        })
        .collect()
}

pub fn get_all<'a>(all: &'a schema::All) -> All<'a> {
    let mut ir_start_time = None;
    if log::log_enabled!(log::Level::Info) {
        ir_start_time = Some(std::time::SystemTime::now());
    }
    let columns = get_all_columns(all);
    let table_constraints = get_all_table_constraints(all, &columns);
    let tables = get_all_tables(all, &columns, &table_constraints);

    let res = All {
        tables: Rc::new(tables),
    };
    if let Some(dur) = ir_start_time.and_then(|s| s.elapsed().ok()) {
        log::info!(
            "ir get all completed: elapsed: {}ms",
            dur.as_micros() as f64 / 1e3
        );
    }
    res
}

fn collect_by_key<'a, D, K, I, F>(iter: I, func: F) -> multimap::MultiMap<K, &'a D>
where
    K: std::hash::Hash + std::cmp::Eq,
    I: Iterator<Item = &'a D> + 'a,
    F: Fn(&'a D) -> K,
{
    iter.map(|data| (func(data), data)).collect()
}
