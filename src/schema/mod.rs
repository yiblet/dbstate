pub use yes_no::YesNo;

mod yes_no;

#[derive(Debug, Clone)]
pub struct All {
    pub tables: Vec<Table>,
    pub columns: Vec<Column>,
    pub views: Vec<View>,
    pub table_constraints: Vec<TableConstraint>,
    pub constraint_column_usage: Vec<ConstraintColumnUsage>,
    pub key_column_usage: Vec<KeyColumnUsage>,
    pub constraint_table_usage: Vec<ConstraintTableUsage>,
    pub element_types: Vec<ElementType>,
    pub check_constraints: Vec<CheckConstraint>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Table {
    /// Name of the database that contains the table (always the current database)
    pub table_catalog: Option<String>,

    /// Name of the schema that contains the table
    pub table_schema: Option<String>,

    /// Name of the table
    pub table_name: String,

    /// Type of the table: BASE TABLE for a persistent base table (the normal table type), VIEW for a view, FOREIGN for a foreign table, or LOCAL TEMPORARY for a temporary table
    pub table_type: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub self_referencing_column_name: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub reference_generation: Option<String>,

    /// If the table is a typed table, the name of the database that contains the underlying data type (always the current database), else null.
    pub user_defined_type_catalog: Option<String>,

    /// If the table is a typed table, the name of the schema that contains the underlying data type, else null.
    pub user_defined_type_schema: Option<String>,

    /// If the table is a typed table, the name of the underlying data type, else null.
    pub user_defined_type_name: Option<String>,

    /// YES if the table is insertable into, NO if not (Base tables are always insertable into, views not necessarily.)
    pub is_insertable_into: Option<YesNo>,

    /// YES if the table is a typed table, NO if not
    pub is_typed: Option<YesNo>,

    /// Not yet implemented
    pub commit_action: Option<String>,
}

impl Table {
    pub fn is_system_schema(&self) -> bool {
        let system_schemas = [Some("pg_catalog"), Some("information_schema")];
        let found = system_schemas
            .iter()
            .find(|s| self.table_schema.as_ref().map(String::as_str) == **s);

        found.is_some()
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct View {
    /// Name of the database that contains the view (always the current database)
    pub table_catalog: Option<String>,

    /// Name of the schema that contains the view
    pub table_schema: Option<String>,

    /// Name of the view
    pub table_name: Option<String>,

    /// Query expression defining the view (null if the view is not owned by a currently enabled role)
    pub view_definition: Option<String>,

    /// CASCADED or LOCAL if the view has a CHECK OPTION defined on it, NONE if not
    pub check_option: Option<String>,

    /// YES if the view is updatable (allows UPDATE and DELETE), NO if not
    pub is_updatable: Option<YesNo>,

    /// YES if the view is insertable into (allows INSERT), NO if not
    pub is_insertable_into: Option<YesNo>,

    /// YES if the view has an INSTEAD OF UPDATE trigger defined on it, NO if not
    pub is_trigger_updatable: Option<YesNo>,

    /// YES if the view has an INSTEAD OF DELETE trigger defined on it, NO if not
    pub is_trigger_deletable: Option<YesNo>,

    /// YES if the view has an INSTEAD OF INSERT trigger defined on it, NO if not
    pub is_trigger_insertable_into: Option<YesNo>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Trigger {
    /// Name of the database that contains the trigger (always the current database)
    pub trigger_catalog: Option<String>,

    /// Name of the schema that contains the trigger
    pub trigger_schema: Option<String>,

    /// Name of the trigger
    pub trigger_name: Option<String>,

    /// Event that fires the trigger (INSERT, UPDATE, or DELETE)
    pub event_manipulation: Option<String>,

    /// Name of the database that contains the table that the trigger is defined on (always the current database)
    pub event_object_catalog: Option<String>,

    /// Name of the schema that contains the table that the trigger is defined on
    pub event_object_schema: Option<String>,

    /// Name of the table that the trigger is defined on
    pub event_object_table: Option<String>,

    /// Firing order among triggers on the same table having the same event_manipulation, action_timing, and action_orientation. In PostgreSQL, triggers are fired in name order, so this column reflects that.
    pub action_order: Option<i32>,

    /// WHEN condition of the trigger, null if none (also null if the table is not owned by a currently enabled role)
    pub action_condition: Option<String>,

    /// Statement that is executed by the trigger (currently always EXECUTE FUNCTION function(...))
    pub action_statement: Option<String>,

    /// Identifies whether the trigger fires once for each processed row or once for each statement (ROW or STATEMENT)
    pub action_orientation: Option<String>,

    /// Time at which the trigger fires (BEFORE, AFTER, or INSTEAD OF)
    pub action_timing: Option<String>,

    /// Name of the ???old??? transition table, or null if none
    pub action_reference_old_table: Option<String>,

    /// Name of the ???new??? transition table, or null if none
    pub action_reference_new_table: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub action_reference_old_row: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub action_reference_new_row: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TableConstraint {
    /// Name of the database that contains the constraint (always the current database)
    pub constraint_catalog: Option<String>,

    /// Name of the schema that contains the constraint
    pub constraint_schema: Option<String>,

    /// Name of the constraint
    pub constraint_name: String,

    /// Name of the database that contains the table (always the current database)
    pub table_catalog: Option<String>,

    /// Name of the schema that contains the table
    pub table_schema: Option<String>,

    /// Name of the table
    pub table_name: String,

    /// Type of the constraint: CHECK, FOREIGN KEY, PRIMARY KEY, or UNIQUE
    pub constraint_type: String,

    /// YES if the constraint is deferrable, NO if not
    pub is_deferrable: Option<YesNo>,

    /// YES if the constraint is deferrable and initially deferred, NO if not
    pub initially_deferred: Option<YesNo>,

    /// Applies to a feature not available in PostgreSQL (currently always YES)
    pub enforced: Option<YesNo>,
    // NOTE there is an additional "nulls_distinct" that's only available in postgres v15 and up
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Column {
    /// Name of the database containing the table (always the current database)
    pub table_catalog: Option<String>,

    /// Name of the schema containing the table
    pub table_schema: Option<String>,

    /// Name of the table
    pub table_name: String,

    /// Name of the column
    pub column_name: String,

    /// Ordinal position of the column within the table (count starts at 1)
    pub ordinal_position: i32,

    /// Default expression of the column
    pub column_default: Option<String>,

    /// YES if the column is possibly nullable, NO if it is known not nullable. A not-null constraint is one way a column can be known not nullable, but there can be others.
    pub is_nullable: Option<YesNo>,

    /// Data type of the column, if it is a built-in type, or ARRAY if it is some array (in that case, see the view element_types), else USER-DEFINED (in that case, the type is identified in udt_name and associated columns). If the column is based on a domain, this column refers to the type underlying the domain (and the domain is identified in domain_name and associated columns).
    pub data_type: String,

    /// If data_type identifies a character or bit string type, the declared maximum length; null for all other data types or if no maximum length was declared.
    pub character_maximum_length: Option<i32>,

    /// If data_type identifies a character type, the maximum possible length in octets (bytes) of a datum; null for all other data types. The maximum octet length depends on the declared character maximum length (see above) and the server encoding.
    pub character_octet_length: Option<i32>,

    /// If data_type identifies a numeric type, this column contains the (declared or implicit) precision of the type for this column. The precision indicates the number of significant digits. It can be expressed in decimal (base 10) or binary (base 2) terms, as specified in the column numeric_precision_radix. For all other data types, this column is null.
    pub numeric_precision: Option<i32>,

    /// If data_type identifies a numeric type, this column indicates in which base the values in the columns numeric_precision and numeric_scale are expressed. The value is either 2 or 10. For all other data types, this column is null.
    pub numeric_precision_radix: Option<i32>,

    /// If data_type identifies an exact numeric type, this column contains the (declared or implicit) scale of the type for this column. The scale indicates the number of significant digits to the right of the decimal point. It can be expressed in decimal (base 10) or binary (base 2) terms, as specified in the column numeric_precision_radix. For all other data types, this column is null.
    pub numeric_scale: Option<i32>,

    /// If data_type identifies a date, time, timestamp, or interval type, this column contains the (declared or implicit) fractional seconds precision of the type for this column, that is, the number of decimal digits maintained following the decimal point in the seconds value. For all other data types, this column is null.
    pub datetime_precision: Option<i32>,

    /// If data_type identifies an interval type, this column contains the specification which fields the intervals include for this column, e.g., YEAR TO MONTH, DAY TO SECOND, etc. If no field restrictions were specified (that is, the interval accepts all fields), and for all other data types, this field is null.
    pub interval_type: Option<String>,

    /// Applies to a feature not available in PostgreSQL (see datetime_precision for the fractional seconds precision of interval type columns)
    pub interval_precision: Option<i32>,

    /// Applies to a feature not available in PostgreSQL
    pub character_set_catalog: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub character_set_schema: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub character_set_name: Option<String>,

    /// Name of the database containing the collation of the column (always the current database), null if default or the data type of the column is not collatable
    pub collation_catalog: Option<String>,

    /// Name of the schema containing the collation of the column, null if default or the data type of the column is not collatable
    pub collation_schema: Option<String>,

    /// Name of the collation of the column, null if default or the data type of the column is not collatable
    pub collation_name: Option<String>,

    /// If the column has a domain type, the name of the database that the domain is defined in (always the current database), else null.
    pub domain_catalog: Option<String>,

    /// If the column has a domain type, the name of the schema that the domain is defined in, else null.
    pub domain_schema: Option<String>,

    /// If the column has a domain type, the name of the domain, else null.
    pub domain_name: Option<String>,

    /// Name of the database that the column data type (the underlying type of the domain, if applicable) is defined in (always the current database)
    pub udt_catalog: Option<String>,

    /// Name of the schema that the column data type (the underlying type of the domain, if applicable) is defined in
    pub udt_schema: Option<String>,

    /// Name of the column data type (the underlying type of the domain, if applicable)
    pub udt_name: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub scope_catalog: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub scope_schema: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub scope_name: Option<String>,

    /// Always null, because arrays always have unlimited maximum cardinality in PostgreSQL
    pub maximum_cardinality: Option<i32>,

    /// An identifier of the data type descriptor of the column, unique among the data type descriptors pertaining to the table. This is mainly useful for joining with other instances of such identifiers. (The specific format of the identifier is not defined and not guaranteed to remain the same in future versions.)
    pub dtd_identifier: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub is_self_referencing: Option<String>,

    /// If the column is an identity column, then YES, else NO.
    pub is_identity: Option<YesNo>,

    /// If the column is an identity column, then ALWAYS or BY DEFAULT, reflecting the definition of the column.
    pub identity_generation: Option<String>,

    /// If the column is an identity column, then the start value of the internal sequence, else null.
    pub identity_start: Option<String>,

    /// If the column is an identity column, then the increment of the internal sequence, else null.
    pub identity_increment: Option<String>,

    /// If the column is an identity column, then the maximum value of the internal sequence, else null.
    pub identity_maximum: Option<String>,

    /// If the column is an identity column, then the minimum value of the internal sequence, else null.
    pub identity_minimum: Option<String>,

    /// If the column is an identity column, then YES if the internal sequence cycles or NO if it does not; otherwise null.
    pub identity_cycle: Option<YesNo>,

    /// If the column is a generated column, then ALWAYS, else NEVER.
    pub is_generated: Option<String>,

    /// If the column is a generated column, then the generation expression, else null.
    pub generation_expression: Option<String>,

    /// YES if the column is updatable, NO if not (Columns in base tables are always updatable, columns in views not necessarily)
    pub is_updatable: Option<YesNo>,
}

/// The view constraint_column_usage identifies all columns in the current database that are used by some constraint. Only those columns are shown that are contained in a table owned by a currently enabled role. For a check constraint, this view identifies the columns that are used in the check expression. For a foreign key constraint, this view identifies the columns that the foreign key references. For a unique or primary key constraint, this view identifies the constrained columns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ConstraintColumnUsage {
    /// Name of the database that contains the table that contains the column that is used by some constraint (always the current database)
    pub table_catalog: Option<String>,

    /// Name of the schema that contains the table that contains the column that is used by some constraint
    pub table_schema: Option<String>,

    /// Name of the table that contains the column that is used by some constraint
    pub table_name: String,

    /// Name of the column that is used by some constraint
    pub column_name: String,

    /// Name of the database that contains the constraint (always the current database)
    pub constraint_catalog: Option<String>,

    /// Name of the schema that contains the constraint
    pub constraint_schema: Option<String>,

    /// Name of the constraint
    pub constraint_name: String,
}

/// The view key_column_usage identifies all columns in the current database that are restricted by some unique, primary key, or foreign key constraint. Check constraints are not included in this view. Only those columns are shown that the current user has access to, by way of being the owner or having some privilege.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KeyColumnUsage {
    /// Name of the database that contains the constraint (always the current database)
    pub constraint_catalog: Option<String>,

    /// Name of the schema that contains the constraint
    pub constraint_schema: Option<String>,

    /// Name of the constraint
    pub constraint_name: String,

    /// Name of the database that contains the table that contains the column that is restricted by this constraint (always the current database)
    pub table_catalog: Option<String>,

    /// Name of the schema that contains the table that contains the column that is restricted by this constraint
    pub table_schema: Option<String>,

    /// Name of the table that contains the column that is restricted by this constraint
    pub table_name: String,

    /// Name of the column that is restricted by this constraint
    pub column_name: String,

    /// Ordinal position of the column within the constraint key (count starts at 1)
    pub ordinal_position: i32,

    /// For a foreign-key constraint, ordinal position of the referenced column within its unique constraint (count starts at 1); otherwise null
    pub position_in_unique_constraint: Option<i32>,
}

/// The view constraint_table_usage identifies all tables in the current database that are used by some constraint and are owned by a currently enabled role. (This is different from the view table_constraints, which identifies all table constraints along with the table they are defined on.) For a foreign key constraint, this view identifies the table that the foreign key references. For a unique or primary key constraint, this view simply identifies the table the constraint belongs to. Check constraints and not-null constraints are not included in this view.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ConstraintTableUsage {
    /// Name of the database that contains the table that is used by some constraint (always the current database)
    pub table_catalog: Option<String>,

    /// Name of the schema that contains the table that is used by some constraint
    pub table_schema: Option<String>,

    /// Name of the table that is used by some constraint
    pub table_name: String,

    /// Name of the database that contains the constraint (always the current database)
    pub constraint_catalog: Option<String>,

    /// Name of the schema that contains the constraint
    pub constraint_schema: Option<String>,

    /// Name of the constraint
    pub constraint_name: String,
}

/// The view element_types contains the data type descriptors of the elements of arrays. When a table column, composite-type attribute, domain, function parameter, or function return value is defined to be of an array type, the respective information schema view only contains ARRAY in the column data_type. To obtain information on the element type of the array, you can join the respective view with this view. For example, to show the columns of a table with data types and array element types, if applicable, you could do:
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ElementType {
    /// Name of the database that contains the object that uses the array being described (always the current database)
    pub object_catalog: Option<String>,

    /// Name of the schema that contains the object that uses the array being described
    pub object_schema: Option<String>,

    /// Name of the object that uses the array being described
    pub object_name: String,

    /// The type of the object that uses the array being described: one of TABLE (the array is used by a column of that table), USER-DEFINED TYPE (the array is used by an attribute of that composite type), DOMAIN (the array is used by that domain), ROUTINE (the array is used by a parameter or the return data type of that function).
    pub object_type: String,

    /// The identifier of the data type descriptor of the array being described. Use this to join with the dtd_identifier columns of other information schema views.
    pub collection_type_identifier: Option<String>,

    /// Data type of the array elements, if it is a built-in type, else USER-DEFINED (in that case, the type is identified in udt_name and associated columns).
    pub data_type: Option<String>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub character_maximum_length: Option<i32>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub character_octet_length: Option<i32>,

    /// Applies to a feature not available in PostgreSQL
    pub character_set_catalog: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub character_set_schema: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub character_set_name: Option<String>,

    /// Name of the database containing the collation of the element type (always the current database), null if default or the data type of the element is not collatable
    pub collation_catalog: Option<String>,

    /// Name of the schema containing the collation of the element type, null if default or the data type of the element is not collatable
    pub collation_schema: Option<String>,

    /// Name of the collation of the element type, null if default or the data type of the element is not collatable
    pub collation_name: Option<String>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub numeric_precision: Option<i32>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub numeric_precision_radix: Option<i32>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub numeric_scale: Option<i32>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub datetime_precision: Option<i32>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub interval_type: Option<String>,

    /// Always null, since this information is not applied to array element data types in PostgreSQL
    pub interval_precision: Option<i32>,

    /// Not yet implemented
    pub domain_default: Option<String>,

    /// Name of the database that the data type of the elements is defined in (always the current database)
    pub udt_catalog: Option<String>,

    /// Name of the schema that the data type of the elements is defined in
    pub udt_schema: Option<String>,

    /// Name of the data type of the elements
    pub udt_name: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub scope_catalog: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub scope_schema: Option<String>,

    /// Applies to a feature not available in PostgreSQL
    pub scope_name: Option<String>,

    /// Always null, because arrays always have unlimited maximum cardinality in PostgreSQL
    pub maximum_cardinality: Option<i32>,

    /// An identifier of the data type descriptor of the element. This is currently not useful.
    pub dtd_identifier: Option<String>,
}

/// The view check_constraints contains all check constraints, either defined on a table or on a domain, that are owned by a currently enabled role. (The owner of the table or domain is the owner of the constraint.)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CheckConstraint {
    /// Name of the database containing the constraint (always the current database)
    pub constraint_catalog: Option<String>,

    /// Name of the schema containing the constraint
    pub constraint_schema: Option<String>,

    /// Name of the constraint
    pub constraint_name: String,

    /// The check expression of the check constraint
    pub check_clause: String,
}
