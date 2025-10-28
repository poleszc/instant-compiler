use crate::ast::{BinOp, Expr, ExprKind, Program, Stmt, StmtKind};
use std::collections::HashMap;

#[derive(Debug)]
pub enum LoweringError {
    UndefinedVariable(String),
    ZeroDivision,
}

#[derive(Debug, Default)]
pub struct JasmineGenerator {
    class_name: String,
    output: String,
    // Mapping of local variables to their index on the JVM stack.
    variables: HashMap<String, u32>,
    // TODO: Use type VarIndex = u32 instead of that.
    stack_index: u32,
}

impl JasmineGenerator {
    pub fn new(class_name: &str) -> Self {
        Self {
            class_name: class_name.to_string(),
            output: String::new(),
            variables: HashMap::new(),
            stack_index: 1,
        }
    }

    // Performs the lowering for the whole program.
    pub fn lower_program(&mut self, program: &mut Program) -> Result<String, LoweringError> {
        program.calculate_weights();
        self.emit_preambule();
        for stmt in &program.0 {
            self.lower_statement(stmt)?;
        }
        self.emit_postambule();
        Ok(self.output.clone())
    }

    // Performs the lowering of a single statement.
    fn lower_statement(&mut self, stmt: &Stmt) -> Result<(), LoweringError> {
        match &stmt.kind {
            StmtKind::Assignment { name, value } => {
                self.lower_expression(value)?;
                let var_index = self.get_or_create_variable_index(name);
                self.emit_instruction(&format!("istore {}", var_index));
            }
            StmtKind::Expression { value } => {
                self.lower_expression(value)?;
                self.emit_print();
            }
        }
        Ok(())
    }

    // Performs the lowering of a single expression.
    fn lower_expression(&mut self, expr: &Expr) -> Result<(), LoweringError> {
        match &expr.kind {
            ExprKind::Literal(val) => {
                self.emit_instruction(&format!("ldc {}", val));
            }
            ExprKind::Identifier(name) => {
                if let Some(index) = self.variables.get(name) {
                    self.emit_instruction(&format!("iload {}", index));
                } else {
                    return Err(LoweringError::UndefinedVariable(name.clone()));
                }
            }
            ExprKind::BinaryOp { op, lhs, rhs, .. } => {
                let is_commutative = matches!(op, BinOp::Add | BinOp::Mul);

                // Left subtree is bigger, then we swap the order of calculations.
                if rhs.get_weight().expect("Weight not calculated for rhs")
                    > lhs.get_weight().expect("Weight not calculated for lhs")
                {
                    self.lower_expression(lhs)?;
                    self.lower_expression(rhs)?;
                } else {
                    self.lower_expression(rhs)?;
                    self.lower_expression(lhs)?;

                    // For non commutative ops we swap the order on the stack.
                    if !is_commutative {
                        self.emit_instruction("swap");
                    }
                }

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
        // TODO: Modify this to use iconst, iload, etc.
        self.output.push_str("    ");
        self.output.push_str(instruction);
        self.output.push('\n');
    }

    fn emit(&mut self, instruction: &str) {
        self.output.push_str(instruction);
        self.output.push('\n');
    }

    fn emit_preambule(&mut self) {
        self.emit(format!(".class public {}", self.class_name).as_str());
        self.emit(".super java/lang/Object");
        self.emit("");
        self.emit(".method public static main([Ljava/lang/String;)V");
        // TODO: Change that to use actual numbers.
        self.emit(" .limit stack 100");
        self.emit(" .limit locals 100");
        self.emit("");
    }

    fn emit_print(&mut self) {
        self.emit_instruction("getstatic java/lang/System/out Ljava/io/PrintStream;");
        self.emit_instruction("swap");
        self.emit_instruction("invokevirtual java/io/PrintStream/println(I)V");
    }

    fn emit_postambule(&mut self) {
        self.emit("");
        self.emit("return");
        self.emit(".end method");
    }
}
