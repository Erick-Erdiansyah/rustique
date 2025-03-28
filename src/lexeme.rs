use std::collections::HashMap;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>), // left, operator, right
}

#[derive(Clone)]
pub enum Statement {
    VarDecl(Variable),
    Print(String),
    ForLoop {
        var_name: String,
        start: i64,
        end: i64,
        body: Vec<Statement>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expr>,
    },
    Assignment {
        name: String,
        expr: Expr,
    },
    Return(Expr),
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub type_annotation: String, // "int", "float" dll
    pub value: Value,
}

#[derive(Clone)]
pub struct FunctionDef {
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

pub struct Interpreter {
    // A stack of scopes; the last is the current environment.
    pub scopes: Vec<HashMap<String, Value>>,
    // Function definitions.
    pub functions: HashMap<String, FunctionDef>,
}
