use crate::ast::{BinOp, Expr, ExprKind, Program, Stmt, StmtKind};
use crate::error::LoweringError;
use std::collections::HashMap;

pub enum JVMInstruction {
    IAdd,
    IMul,
    ISub,
    IDiv,
    Swap,
    IStore { index: u32 },
    ILoad { index: u32 },
    IConst { value: i32 },
    GetStatic { object: String },
    InvokeVirtual { method: String },
    Return,
}

impl JVMInstruction {
    fn stringify(&self) -> String {
        match self {
            JVMInstruction::IAdd => "iadd".to_string(),
            JVMInstruction::ISub => "isub".to_string(),
            JVMInstruction::IMul => "imul".to_string(),
            JVMInstruction::IDiv => "idiv".to_string(),
            JVMInstruction::Swap => "swap".to_string(),
            JVMInstruction::ILoad { index } => format!("iload {}", index),
            JVMInstruction::IStore { index } => format!("istore {}", index),
            JVMInstruction::IConst { value } => {
                if *value <= 5 {
                    format!("iconst_{}", value)
                } else {
                    format!("ldc {}", value)
                }
            }
            JVMInstruction::GetStatic { object } => format!("getstatic {}", object),
            JVMInstruction::InvokeVirtual { method } => format!("invokevirtual {}", method),
            JVMInstruction::Return => "return".to_string(),
        }
    }
}
type VarIndex = u32;
#[derive(Debug, Default)]
pub struct JasmineGenerator {
    class_name: String,
    output: String,
    // Mapping of local variables to their index on the JVM stack.
    variables: HashMap<String, u32>,
    // TODO: Use type VarIndex = u32 instead of that.
    stack_index: VarIndex,
}

impl JasmineGenerator {
    pub fn new(class_name: &str) -> Self {
        Self {
            class_name: class_name.to_string(),
            output: String::new(),
            variables: HashMap::new(),
            stack_index: 1, // Starting from 1, since 0 is reserved for args.
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
                self.emit_instruction(JVMInstruction::IStore { index: var_index });
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
                self.emit_instruction(JVMInstruction::IConst { value: *val });
            }
            ExprKind::Identifier(name) => {
                if let Some(index) = self.variables.get(name) {
                    self.emit_instruction(JVMInstruction::ILoad { index: *index });
                } else {
                    return Err(LoweringError::UndefinedVariable {
                        name: name.clone(),
                        position: expr.position,
                    });
                }
            }
            ExprKind::BinaryOp { op, lhs, rhs, .. } => {
                let is_commutative = matches!(op, BinOp::Add | BinOp::Mul);

                if let ExprKind::Literal(0) = rhs.kind {
                    if matches!(op, BinOp::Div) {
                        return Err(LoweringError::ZeroDivision {
                            position: rhs.position,
                        });
                    }
                }

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
                        self.emit_instruction(JVMInstruction::Swap);
                    }
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

    fn emit_instruction(&mut self, instruction: JVMInstruction) {
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
        // TODO: Change that to use actual numbers.
        self.emit(" .limit stack 100");
        self.emit(" .limit locals 100");
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
