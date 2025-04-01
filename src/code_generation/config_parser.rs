use core::panic;
use crate::code_generation::var_type::VarType;

use regex::Regex;

fn is_valid_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    re.is_match(name)
}

pub struct VariableDefinition {
    pub name: String,
    pub var_type: VarType
}

impl VariableDefinition {
    pub fn new(name: &str, var_type: VarType) -> Option<Self> {
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
    pub name: String,
    pub variable_definitions: Vec<VariableDefinition>,
}

impl DataDefinition {
    pub fn new(name: &str, variable_definitions: Vec<VariableDefinition>) -> Option<Self> {
        if !is_valid_name(name) {
            return None;
        }
        Some(DataDefinition {
            name: name.to_string(),
            variable_definitions,
        })
    }
}

fn parse_type(type_definition: &str) -> VarType {
    VarType::new(type_definition).expect(&format!(
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
        assert_eq!(parsed.var_type, VarType::I8);
    }

    #[test]
    fn test_parse_data() {
        let data_definition = "Data { i32 x; float y; }";
        let parsed = parse_data(data_definition);
        assert_eq!(parsed.name, "Data");
        assert_eq!(parsed.variable_definitions.len(), 2);
        assert_eq!(parsed.variable_definitions[0].name, "x");
        assert_eq!(parsed.variable_definitions[0].var_type, VarType::I32);
        assert_eq!(parsed.variable_definitions[1].name, "y");
        assert_eq!(parsed.variable_definitions[1].var_type, VarType::Float);
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