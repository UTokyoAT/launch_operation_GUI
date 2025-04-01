#[derive(Debug,PartialEq)]
pub enum VarType {
    Float,
    Double,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Bool,
}

impl VarType {
    pub fn new(s: &str) -> Option<Self> {
        match s {
            "float" => Some(VarType::Float),
            "double" => Some(VarType::Double),
            "i8" => Some(VarType::I8),
            "i16" => Some(VarType::I16),
            "i32" => Some(VarType::I32),
            "i64" => Some(VarType::I64),
            "u8" => Some(VarType::U8),
            "u16" => Some(VarType::U16),
            "u32" => Some(VarType::U32),
            "u64" => Some(VarType::U64),
            "bool" => Some(VarType::Bool),
            _ => None,
        }
    }

    fn bytes(&self) -> usize {
        match self {
            VarType::Float => 4,
            VarType::Double => 8,
            VarType::I8 => 1,
            VarType::I16 => 2,
            VarType::I32 => 4,
            VarType::I64 => 8,
            VarType::U8 => 1,
            VarType::U16 => 2,
            VarType::U32 => 4,
            VarType::U64 => 8,
            VarType::Bool => 1,
        }
    }
}