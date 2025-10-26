use std::fmt::{Debug, write};

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

// TODO: Maybe implement your own debug traits.
// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         use self::Expr::*;
//         match *self {
//             Number(n) => write!(fmt, "{n:?}"),
//             Op(ref l, op, ref r) => write!(fmt, "({l:?} {op:?} {r:?})"),
//             Error => write!(fmt, "error"),
//         }
//     }
// }

// impl Debug for Opcode {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         use self::Opcode::*;
//         match *self {
//             Mul => write!(fmt, "*"),
//             Div => write!(fmt, "/"),
//             Add => write!(fmt, "+"),
//             Sub => write!(fmt, "-"),
//         }
//     }
// }
