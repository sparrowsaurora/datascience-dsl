// Abstract Syntax Tree definitions

#[derive(Debug, Clone)]
pub enum Statement {
    // load target <- path
    Load {
        target: String,
        path: String,
    },

    // filter(<expression>)
    Filter {
        expr: Expr,
    },

    GroupBy {
        keys: Vec<String>,
    },

    Aggregate {
        func: String,
        column: String,
        alias: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Comparison {
        left: Value,
        op: CompOp,
        right: Value,
    },
}

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    Number(f64),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum CompOp {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
}
