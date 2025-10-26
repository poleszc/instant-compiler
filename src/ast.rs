use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Stmt>);

// TODO: Why Eq can't be here?
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Assignment { name: String, value: Box<Expr> },
    Expression { value: Box<Expr> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(i64),
    Identifier(String),
    BinaryOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
