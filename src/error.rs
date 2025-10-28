use crate::ast::SourcePosition;

#[derive(Debug)]
pub enum LoweringError {
    UndefinedVariable {
        name: String,
        position: SourcePosition,
    },
    ZeroDivision {
        position: SourcePosition,
    },
}
