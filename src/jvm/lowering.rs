/// Main file containing the JVM code generation logic.
use crate::error::SemanticError;
use crate::jvm::JVMInstruction;
use crate::parser::ast::{BinOp, Expr, ExprKind, Program, Stmt, StmtKind};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct JasmineGenerator {
    class_name: String,
    output: String,
    // Mapping of local variables to their index on the JVM stack.
    variables: HashMap<String, u32>,
    stack_size: i32,
    limit_locals: u32,
    limit_stack: u32,
}

impl JasmineGenerator {
    pub fn new(class_name: &str) -> Self {
        Self {
            class_name: class_name.to_string(),
            output: String::new(),
            variables: HashMap::new(),
            stack_size: 0,
            limit_locals: 1, // Starting from 1, since args exist.
            limit_stack: 0,
        }
    }

    /// Performs the lowering for the whole program.
    pub fn lower_program(&mut self, program: &mut Program) -> Result<String, SemanticError> {
        program.calculate_weights();

        // Emit the method body first.
        let mut method_body = String::new();
        std::mem::swap(&mut self.output, &mut method_body);

        for stmt in &program.0 {
            self.lower_statement(stmt)?;
        }
        std::mem::swap(&mut self.output, &mut method_body);

        // Now emit the preambule with the calculated limit_stack and
        // limit_locals, then append the method body and the postambule.
        self.emit_preambule();
        self.output.push_str(&method_body);
        self.emit_postambule();

        Ok(self.output.clone())
    }

    /// Performs the lowering of a single statement.
    fn lower_statement(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        match &stmt.kind {
            StmtKind::Assignment { name, value } => {
                self.lower_expression(value)?;
                let var_index = self.get_or_create_variable_index(name);
                self.emit_instruction(JVMInstruction::IStore { index: var_index });
            }
            StmtKind::Expression { value } => {
                self.lower_expression(value)?;
                self.emit_print();
            }
        }
        Ok(())
    }

    /// Performs the lowering of a single expression.
    fn lower_expression(&mut self, expr: &Expr) -> Result<(), SemanticError> {
        match &expr.kind {
            ExprKind::Literal(val) => {
                self.emit_instruction(JVMInstruction::IConst { value: *val });
            }
            ExprKind::Identifier(name) => {
                if let Some(index) = self.variables.get(name) {
                    self.emit_instruction(JVMInstruction::ILoad { index: *index });
                } else {
                    return Err(SemanticError::UndefinedVariable {
                        name: name.clone(),
                        position: expr.position,
                    });
                }
            }
            ExprKind::BinaryOp { op, lhs, rhs, .. } => {
                let is_commutative = matches!(op, BinOp::Add | BinOp::Mul);

                if let ExprKind::Literal(0) = rhs.kind
                    && matches!(op, BinOp::Div)
                {
                    return Err(SemanticError::ZeroDivision {
                        position: rhs.position,
                    });
                }

                // Left subtree is bigger, then we swap the order of calculations.
                if rhs.get_weight().expect("Weight not calculated for rhs")
                    > lhs.get_weight().expect("Weight not calculated for lhs")
                {
                    self.lower_expression(rhs)?;
                    self.lower_expression(lhs)?;

                    // For non commutative ops we swap the order on the stack.
                    if !is_commutative {
                        self.emit_instruction(JVMInstruction::Swap);
                    }
                } else {
                    self.lower_expression(lhs)?;
                    self.lower_expression(rhs)?;
                }

                let instr = match op {
                    BinOp::Add => JVMInstruction::IAdd,
                    BinOp::Sub => JVMInstruction::ISub,
                    BinOp::Mul => JVMInstruction::IMul,
                    BinOp::Div => JVMInstruction::IDiv,
                };
                self.emit_instruction(instr);
            }
        }
        Ok(())
    }

    /// If a variable exists returns it's index on the stack.
    /// If not, creates a new variable on the stack and returns it's index.
    fn get_or_create_variable_index(&mut self, name: &str) -> u32 {
        if let Some(index) = self.variables.get(name) {
            *index
        } else {
            let index = self.limit_locals;
            self.limit_locals += 1;
            self.variables.insert(name.to_string(), index);
            index
        }
    }

    /// Update the stack by the `change` and update `limit_stack` if necessary.
    fn update_stack(&mut self, change: i32) {
        self.stack_size += change;
        assert!(self.stack_size >= 0);
        if self.stack_size as u32 > self.limit_stack {
            self.limit_stack = self.stack_size as u32;
        }
    }

    fn emit_instruction(&mut self, instruction: JVMInstruction) {
        // Calculate the stack change for each operation.
        let stack_change = match instruction {
            JVMInstruction::IAdd
            | JVMInstruction::ISub
            | JVMInstruction::IMul
            | JVMInstruction::IDiv => -1, // Pops two, pushes one.
            JVMInstruction::IStore { .. } => -1,
            JVMInstruction::ILoad { .. } => 1,
            JVMInstruction::IConst { .. } => 1,
            JVMInstruction::GetStatic { .. } => 1,
            JVMInstruction::InvokeVirtual { .. } => -2, // Pops the obj and the value form the stack. Pushes nothing.
            JVMInstruction::Swap => 0,
            JVMInstruction::Return => 0,
        };
        self.update_stack(stack_change);

        self.output.push_str("    ");
        self.output.push_str(instruction.stringify().as_str());
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
        self.emit(&format!(" .limit stack {}", self.limit_stack));
        self.emit(&format!(" .limit locals {}", self.limit_locals));
        self.emit("");
    }

    fn emit_print(&mut self) {
        self.emit_instruction(JVMInstruction::GetStatic {
            object: "java/lang/System/out Ljava/io/PrintStream;".to_string(),
        });
        self.emit_instruction(JVMInstruction::Swap);
        self.emit_instruction(JVMInstruction::InvokeVirtual {
            method: "java/io/PrintStream/println(I)V".to_string(),
        });
    }

    fn emit_postambule(&mut self) {
        self.emit("");
        self.emit_instruction(JVMInstruction::Return);
        self.emit(".end method");
    }
}
