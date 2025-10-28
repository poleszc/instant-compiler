use std::fmt::Debug;



#[derive(Debug, PartialEq, Clone)]
pub struct SourcePosition(pub usize, pub usize);
impl From<(usize, usize)> for  SourcePosition{
    fn from(value: (usize, usize)) -> Self {
        SourcePosition(value.0, value.1)
    }
    
}


#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Stmt>);

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub position: SourcePosition,
}

// TODO: Why Eq can't be here?
#[derive(Debug, PartialEq, Clone)]
pub enum StmtKind {
    Assignment { name: String, value: Box<Expr> },
    Expression { value: Box<Expr> },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub position: SourcePosition,
}



#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
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
                lhs, rhs, weight, ..
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

    pub fn get_weight(&self) -> Option<u64> {
        match self {
            Expr::BinaryOp { weight, .. } => *weight,
            _ => Some(1),
        }
    }
}
