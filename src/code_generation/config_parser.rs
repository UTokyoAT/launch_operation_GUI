use core::panic;

use regex::Regex;

#[derive(Debug,PartialEq)]
pub enum Type {
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

impl Type {
    fn new(s: &str) -> Option<Self> {
        match s {
            "float" => Some(Type::Float),
            "double" => Some(Type::Double),
            "i8" => Some(Type::I8),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "bool" => Some(Type::Bool),
            _ => None,
        }
    }

    fn bytes(&self) -> usize {
        match self {
            Type::Float => 4,
            Type::Double => 8,
            Type::I8 => 1,
            Type::I16 => 2,
            Type::I32 => 4,
            Type::I64 => 8,
            Type::U8 => 1,
            Type::U16 => 2,
            Type::U32 => 4,
            Type::U64 => 8,
            Type::Bool => 1,
        }
    }
}

fn is_valid_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    re.is_match(name)
}

pub struct VariableDefinition {
    name: String,
    var_type: Type
}

impl VariableDefinition {
    fn new(name: &str, var_type: Type) -> Option<Self> {
        if !is_valid_name(name) {
            return None;
        }
        Some(VariableDefinition {
            name: name.to_string(),
            var_type,
        })
    }
}

pub struct DataDefinition {
    name: String,
    variable_definitions: Vec<VariableDefinition>,
}

impl DataDefinition {
    fn new(name: &str, variable_definitions: Vec<VariableDefinition>) -> Option<Self> {
        if !is_valid_name(name) {
            return None;
        }
        Some(DataDefinition {
            name: name.to_string(),
            variable_definitions,
        })
    }
}

fn parse_type(type_definition: &str) -> Type {
    Type::new(type_definition).expect(&format!(
        "Invalid type definition: {}",
        type_definition
    ))
}

fn parse_variable(variable_definition: &str) -> VariableDefinition {
    let parts: Vec<&str> = variable_definition.trim().split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Invalid variable definition: {}", variable_definition);
    }
    let var_type = parse_type(parts[0]);
    let name = parts[1];
    if !is_valid_name(name) {
        panic!("Invalid variable name: {}", name);
    }
    VariableDefinition::new(name, var_type).unwrap()
}

fn parse_data(data_definition: &str) -> DataDefinition {
    let re = Regex::new(r"(?m)(\w+)\s*\{([^}]*)\}").unwrap();
    let caps = re.captures(data_definition).expect("Invalid data definition format");
    let name = caps.get(1).expect("Missing data name").as_str();
    let body = caps.get(2).expect("Missing data body").as_str().trim();
    if !is_valid_name(name) {
        panic!("Invalid data name: {}", name);
    }
    let variables = body
        .split(';')
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .map(|line| parse_variable(line))
        .collect();
    
    DataDefinition::new(name, variables).unwrap()
}

pub fn parse_config(config: &str) -> Vec<DataDefinition> {
    config
        .split('}')
        .filter_map(|data| {
            let trimmed = data.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(parse_data(&(trimmed.to_string() + "}")))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable() {
        let variable_definition = "i8 x";
        let parsed = parse_variable(variable_definition);
        assert_eq!(parsed.name, "x");
        assert_eq!(parsed.var_type, Type::I8);
    }

    #[test]
    fn test_parse_data() {
        let data_definition = "Data { i32 x; float y; }";
        let parsed = parse_data(data_definition);
        assert_eq!(parsed.name, "Data");
        assert_eq!(parsed.variable_definitions.len(), 2);
        assert_eq!(parsed.variable_definitions[0].name, "x");
        assert_eq!(parsed.variable_definitions[0].var_type, Type::I32);
        assert_eq!(parsed.variable_definitions[1].name, "y");
        assert_eq!(parsed.variable_definitions[1].var_type, Type::Float);
    }

    #[test]
    fn test_parse_config() {
        let config = "Data1 \n { \n double x123; \n u32 y_; \n } \n Data2 \n { u64 _z1; }";
        let parsed = parse_config(config);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, "Data1");
        assert_eq!(parsed[1].name, "Data2");
        assert_eq!(parsed[0].variable_definitions.len(), 2);
        assert_eq!(parsed[1].variable_definitions.len(), 1);
    }
}