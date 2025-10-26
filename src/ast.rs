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
        weight: Option<u64>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Program {
    pub fn calculate_weights(&mut self) {
        for stmt in &mut self.0 {
            stmt.calculate_weights();
        }
    }
}

impl Stmt {
    fn calculate_weights(&mut self) {
        match self {
            Stmt::Assignment { value, .. } => {
                value.calculate_weights();
            }
            Stmt::Expression { value } => {
                value.calculate_weights();
            }
        }
    }
}

impl Expr {
    pub fn calculate_weights(&mut self) -> u64 {
        match self {
            Expr::Literal(_) => 1,
            Expr::Identifier(_) => 1,
            Expr::BinaryOp {
                lhs,
                rhs,
                weight,
                ..
            } => {
                let lhs_weight = lhs.calculate_weights();
                let rhs_weight = rhs.calculate_weights();

                let current_weight = if lhs_weight == rhs_weight {
                    lhs_weight + 1
                } else {
                    lhs_weight.max(rhs_weight)
                };

                *weight = Some(current_weight);

                current_weight
            }
        }
    }
}
