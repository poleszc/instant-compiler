use crate::ast::{BinOp, Expr, Program, Stmt};
use std::collections::HashMap;

#[derive(Debug)]
pub enum LoweringError {
    UndefinedVariable(String),
    ZeroDivision,
}

#[derive(Debug, Default)]
pub struct JasmineGenerator {
    output: String,
    // Mapping of local variables to their index on the JVM stack.
    variables: HashMap<String, u32>,
    // TODO: Use type VarIndex = u32 instead of that.
    stack_index: u32,
}

impl JasmineGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            variables: HashMap::new(),
            stack_index: 1,
        }
    }

    // Performs the lowering for the whole program.
    pub fn lower_program(&mut self, program: &Program) -> Result<String, LoweringError> {
        self.emit_preambule();
        for stmt in &program.0 {
            self.lower_statement(stmt)?;
        }
        self.emit_postambule();
        Ok(self.output.clone())
    }

    // Performs the lowering of a single statement.
    fn lower_statement(&mut self, stmt: &Stmt) -> Result<(), LoweringError> {
        match stmt {
            Stmt::Assignment { name, value } => {
                self.lower_expression(value)?;
                let var_index = self.get_or_create_variable_index(name);
                self.emit_instruction(&format!("istore {}", var_index));
            }
            Stmt::Expression { value } => {
                self.lower_expression(value)?;
            }
        }
        Ok(())
    }

    // Performs the lowering of a single expression.
    fn lower_expression(&mut self, expr: &Expr) -> Result<(), LoweringError> {
        match expr {
            Expr::Literal(val) => {
                self.emit_instruction(&format!("ldc {}", val));
            }
            Expr::Identifier(name) => {
                if let Some(index) = self.variables.get(name) {
                    self.emit_instruction(&format!("iload {}", index));
                } else {
                    return Err(LoweringError::UndefinedVariable(name.clone()));
                }
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                self.lower_expression(lhs)?;
                self.lower_expression(rhs)?;
                let instr = match op {
                    BinOp::Add => "iadd",
                    BinOp::Sub => "isub",
                    BinOp::Mul => "imul",
                    // TODO: Check for zero division.
                    BinOp::Div => "idiv",
                };
                self.emit_instruction(instr);
            }
        }
        Ok(())
    }

    fn get_or_create_variable_index(&mut self, name: &str) -> u32 {
        if let Some(index) = self.variables.get(name) {
            *index
        } else {
            let index = self.stack_index;
            self.stack_index += 1;
            self.variables.insert(name.to_string(), index);
            index
        }
    }

    fn emit_instruction(&mut self, instruction: &str) {
        self.output.push_str("    ");
        self.output.push_str(instruction);
        self.output.push('\n');
    }

    fn emit(&mut self, instruction: &str) {
        self.output.push_str(instruction);
        self.output.push('\n');
    }

    fn emit_preambule(&mut self) {
        self.emit(".class public Main");
        self.emit(".super java/lang/Object");
        self.emit("");
        self.emit(".method public static main([Ljava/lang/String;)V");
        // TODO: Change that to use actual numbers.
        self.emit(" .limit stack 100");
        self.emit(" .limit locals 100");
        self.emit("");
    }

    fn emit_postambule(&mut self) {
        self.emit("");
        self.emit("return");
        self.emit(".end method");
    }
}
