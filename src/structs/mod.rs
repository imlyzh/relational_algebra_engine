pub mod ast;
pub mod plan;
pub mod plan_group;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Add(Box<Expr>),
    Sub(Box<Expr>),
    Mul(Box<Expr>),
    Div(Box<Expr>),
    Mod(Box<Expr>),
    Value(Value)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    String(String),
    Symbol(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol (pub String, pub Option<String>);