use crate::var_type::VarType;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct TypeToString {
    float_: String,
    double_: String,
    i8_: String,
    i16_: String,
    i32_: String,
    i64_: String,
    u8_: String,
    u16_: String,
    u32_: String,
    u64_: String,
    bool_: String,
}

impl TypeToString {
    pub fn read_json(path: Box<Path>) -> Self {
        let file = File::open(path).expect("Unable to open file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Unable to parse JSON")
    }

    pub fn to_fn(self) -> Box<dyn Fn(VarType) -> String> {
        Box::new(move |var_type| match var_type {
            VarType::I8 => self.i8_.clone(),
            VarType::I16 => self.i16_.clone(),
            VarType::I32 => self.i32_.clone(),
            VarType::I64 => self.i64_.clone(),
            VarType::U8 => self.u8_.clone(),
            VarType::U16 => self.u16_.clone(),
            VarType::U32 => self.u32_.clone(),
            VarType::U64 => self.u64_.clone(),
            VarType::Float => self.float_.clone(),
            VarType::Double => self.double_.clone(),
            VarType::Bool => self.bool_.clone(),
        })
    }
}

pub fn read_template(path: Box<Path>) -> String {
    std::fs::read_to_string(path).expect("Unable to read file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::var_type::VarType;

    #[test]
    fn test_read() {
        let path = Path::new("config/template/rust/types.json");
        let type_to_string = TypeToString::read_json(Box::from(path));
        assert_eq!(type_to_string.float_, "f32");
        assert_eq!(type_to_string.double_, "f64");
        let var_type_to_string = type_to_string.to_fn();
        assert_eq!(var_type_to_string(VarType::I8), "i8");
        assert_eq!(var_type_to_string(VarType::I16), "i16");
    }

    #[test]
    fn test_read_template() {
        let path = Path::new("config/template/rust/template.txt");
        let template = read_template(Box::from(path));
        assert!(!template.is_empty());
    }
}
