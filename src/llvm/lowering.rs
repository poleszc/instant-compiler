use crate::ast::{BinOp, Expr, Program, Stmt};
use std::collections::HashMap;

#[derive(Debug)]
pub enum LoweringError {
    UndefinedVariable(String),
    ZeroDivision,
}

pub type RegisterName = String;
pub type PointerName = String;
pub type Constant = String;

#[derive(Debug)]
pub enum LLVMInstruction {
    Add {
        location: RegisterName,
        left: RegisterName,
        right: RegisterName,
    },
    Sub {
        location: RegisterName,
        left: RegisterName,
        right: RegisterName,
    },
    Div {
        location: RegisterName,
        left: RegisterName,
        right: RegisterName,
    },
    Mul {
        location: RegisterName,
        left: RegisterName,
        right: RegisterName,
    },
    Load {
        destination: RegisterName,
        source: PointerName,
    },
    Store {
        value: RegisterName,
        destination: PointerName,
    },
    Alloca {
        destination: PointerName,
    },
    Ret {
        value: RegisterName,
    },
    CallPrint {
        value: RegisterName,
    },
}
impl LLVMInstruction {
    fn stringify(&self) -> String {
        match self {
            LLVMInstruction::Add {
                location,
                left,
                right,
            } => format!("{} = add i32 {}, {}", location, left, right),
            LLVMInstruction::Sub {
                location,
                left,
                right,
            } => format!("{} = sub i32 {}, {}", location, left, right),
            LLVMInstruction::Mul {
                location,
                left,
                right,
            } => format!("{} = mul i32 {}, {}", location, left, right),
            LLVMInstruction::Div {
                location,
                left,
                right,
            } => format!("{} = div i32 {}, {}", location, left, right),
            LLVMInstruction::Load {
                destination,
                source,
            } => format!("{} = load i32, ptr {}", destination, source),
            LLVMInstruction::Store { value, destination } => {
                format!("store i32 {}, ptr {}", value, destination)
            }
            LLVMInstruction::Alloca { destination } => {
                format!("{} = alloca i32", destination)
            }
            LLVMInstruction::CallPrint { value } => {
                format!("call void @printInt(i32 {})", value)
            }
            LLVMInstruction::Ret { value } => format!("ret i32 {value}"),
        }
    }
}

#[derive(Debug, Default)]
pub struct LLVMGenerator {
    output: String,
    // Mapping of local variables to their pointer name in generated code.
    variables: HashMap<String, PointerName>,
    // TODO": Register index
    register_index: u32,
}

impl LLVMGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            variables: HashMap::new(),
            register_index: 0,
        }
    }

    fn new_register(&mut self) -> RegisterName {
        let name = format!("%{}", self.register_index);
        self.register_index += 1;
        name
    }

    fn get_or_create_variable(&mut self, name: &str) -> PointerName {
        if let Some(ptr_name) = self.variables.get(name) {
            ptr_name.clone()
        } else {
            let ptr_name = format!("%{}", name);
            let instr = LLVMInstruction::Alloca {
                destination: ptr_name.clone(),
            };
            self.emit_instruction(&instr);
            self.variables.insert(name.to_string(), ptr_name.clone());
            ptr_name
        }
    }

    // Performs the lowering for the whole program.
    pub fn lower_program(&mut self, program: &mut Program) -> Result<String, LoweringError> {
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
                let val_reg = self.lower_expression(value)?;
                let ptr_name = self.get_or_create_variable(name);
                let instr = LLVMInstruction::Store {
                    value: val_reg,
                    destination: ptr_name,
                };
                self.emit_instruction(&instr);
            }
            Stmt::Expression { value } => {
                let val_reg = self.lower_expression(value)?;
                let instr = LLVMInstruction::CallPrint { value: val_reg };
                self.emit_instruction(&instr);
            }
        }

        self.emit("\n");
        Ok(())
    }

    // Performs the lowering of a single expression.
    fn lower_expression(&mut self, expr: &Expr) -> Result<RegisterName, LoweringError> {
        match expr {
            Expr::Literal(val) => Ok(val.to_string()),
            Expr::Identifier(name) => {
                if let Some(ptr_name) = self.variables.get(name).cloned() {
                    let dest_reg = self.new_register();
                    let instr = LLVMInstruction::Load {
                        destination: dest_reg.clone(),
                        source: ptr_name,
                    };

                    self.emit_instruction(&instr);
                    Ok(dest_reg)
                } else {
                    Err(LoweringError::UndefinedVariable(name.clone()))
                }
            }
            Expr::BinaryOp { op, lhs, rhs, .. } => {
                let lhs_reg = self.lower_expression(lhs)?;
                let rhs_reg = self.lower_expression(rhs)?;

                let location = self.new_register();

                let instr = match op {
                    BinOp::Add => LLVMInstruction::Add {
                        location: location.clone(),
                        left: lhs_reg,
                        right: rhs_reg,
                    },
                    BinOp::Sub => LLVMInstruction::Sub {
                        location: location.clone(),
                        left: lhs_reg,
                        right: rhs_reg,
                    },
                    BinOp::Mul => LLVMInstruction::Mul {
                        location: location.clone(),
                        left: lhs_reg,
                        right: rhs_reg,
                    },
                    // TODO: Check for zero division.
                    BinOp::Div => LLVMInstruction::Div {
                        location: location.clone(),
                        left: lhs_reg,
                        right: rhs_reg,
                    },
                };
                self.emit_instruction(&instr);
                Ok(location)
            }
        }
    }

    fn emit_instruction(&mut self, instruction: &LLVMInstruction) {
        self.output.push_str("    ");
        self.output.push_str(instruction.stringify().as_str());
        self.output.push('\n');
    }

    fn emit(&mut self, instruction: &str) {
        self.output.push_str(instruction);
        self.output.push('\n');
    }

    fn emit_preambule(&mut self) {
        self.emit("declare void @printInt(i32)"); // Define print from external module.
        self.emit(""); // Define print from external module.
        self.emit("define i32 @main() {");
    }

    fn emit_postambule(&mut self) {
        self.emit("");
        self.emit_instruction(&LLVMInstruction::Ret {
            value: 0.to_string(),
        });
        self.emit("}");
    }
}
