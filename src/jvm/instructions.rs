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
    pub fn stringify(&self) -> String {
        match self {
            JVMInstruction::IAdd => "iadd".to_string(),
            JVMInstruction::ISub => "isub".to_string(),
            JVMInstruction::IMul => "imul".to_string(),
            JVMInstruction::IDiv => "idiv".to_string(),
            JVMInstruction::Swap => "swap".to_string(),
            JVMInstruction::ILoad { index } => {
                if *index <= 3 {
                    format!("iload_{}", index)
                } else {
                    format!("iload {}", index)
                }
            }
            JVMInstruction::IStore { index } => {
                if *index <= 3 {
                    format!("istore_{}", index)
                } else {
                    format!("istore {}", index)
                }
            }
            JVMInstruction::IConst { value } => {
                if *value <= 5 {
                    format!("iconst_{}", value)
                } else if *value <= 127 {
                    format!("bipush {}", value)
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
