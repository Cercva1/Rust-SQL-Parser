use crate::error::Result;

/// Represents a column definition inside a CREATE TABLE statement.
#[derive(Debug)]
pub struct ColumnDef {
    pub name: String,               // Column name
    pub data_type: DataType,        // Column data type (e.g., INT, TEXT)
    pub constraints: Vec<ColumnConstraint>, // Column constraints like NOT NULL, PRIMARY KEY
}

/// Supported SQL data types in this simplified parser.
#[derive(Debug)]
pub enum DataType {
    Int,
    Text,
}

/// Column constraints that can modify column behavior.
#[derive(Debug)]
pub enum ColumnConstraint {
    NotNull,
    PrimaryKey,

}
