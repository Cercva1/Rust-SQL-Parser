/// Represents a full SELECT SQL statement in AST form.
#[derive(Debug)]
pub struct SelectStatement {
    pub projection: Vec<SelectItem>,          // What columns or expressions to select
    pub from: String,                         // Table name to select from
    pub selection: Option<Expr>,              // Optional WHERE clause expression
    pub order_by: Option<Vec<OrderByExpr>>,  // Optional ORDER BY expressions
}

/// Items to select, typically column names.
#[derive(Debug)]
pub enum SelectItem {
    Column(String),  // Select a specific column by name
}

/// Expressions in WHERE, ORDER BY, or other clauses.
#[derive(Debug)]
pub enum Expr {
    BinaryOp {
        left: Box<Expr>,   // Left-hand side expression
        op: String,        // Operator, e.g., "+", ">", etc.
        right: Box<Expr>,  // Right-hand side expression
    },
    LiteralInt(i64),        // Integer literal
    ColumnRef(String),      // Reference to a column by name
}

/// Represents an ORDER BY expression with sorting direction.
#[derive(Debug)]
pub struct OrderByExpr {
    pub expr: Expr,  // Expression to order by
    pub asc: bool,   // true if ASC, false if DESC
}
