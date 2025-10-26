use std::fmt::{Debug, write};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{n:?}"),
            Op(ref l, op, ref r) => write!(fmt, "({l:?} {op:?} {r:?})"),
            Error => write!(fmt, "error"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
