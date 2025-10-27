#[derive(Debug)]
pub enum LoweringError {
    UndefinedVariable(String),
    ZeroDivision,
}
