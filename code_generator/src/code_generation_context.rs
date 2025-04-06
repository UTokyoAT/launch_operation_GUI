use crate::var_type::VarType;
use crate::config_parser::{DataDefinition, VariableDefinition};
pub struct VariableInformation {
    pub name: String,
    pub var_type: VarType,
    pub offset_bytes: usize,
    pub size_bytes: usize,
}
pub struct CodeGenerationContext {
    pub name: String,
    pub variable_information: Vec<VariableInformation>,
    pub total_bytes: usize,
}

impl CodeGenerationContext {
    pub fn new(data_definition: DataDefinition) -> Self {
        let mut total_bytes = 0;
        let mut variable_information = Vec::new();
        for VariableDefinition {
            name,
            var_type,
        } in data_definition.variable_definitions
        {
            let offset_bytes = total_bytes;
            let size_bytes = var_type.bytes();
            total_bytes += size_bytes;
            variable_information.push(VariableInformation {
                name,
                var_type,
                offset_bytes,
                size_bytes
            });
        }
        CodeGenerationContext {
            name: data_definition.name.clone(),
            variable_information,
            total_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::var_type::VarType;

    #[test]
    fn test_code_generation_context() {
        let variable_definitions = vec![
            VariableDefinition::new("var1", VarType::I8).unwrap(),
            VariableDefinition::new("var2", VarType::Float).unwrap(),
            VariableDefinition::new("var3", VarType::I32).unwrap(),
        ];
        let data_definition = DataDefinition::new("TestData", variable_definitions).unwrap();
        let context = CodeGenerationContext::new(data_definition);
        assert_eq!(context.name, "TestData");
        assert_eq!(context.total_bytes, 9);
        assert_eq!(context.variable_information.len(), 3);
        assert_eq!(context.variable_information[0].name, "var1");
        assert_eq!(context.variable_information[0].var_type, VarType::I8);
        assert_eq!(context.variable_information[0].offset_bytes, 0);
        assert_eq!(context.variable_information[0].size_bytes, 1);
        assert_eq!(context.variable_information[1].name, "var2");
        assert_eq!(context.variable_information[1].var_type, VarType::Float);
        assert_eq!(context.variable_information[1].offset_bytes, 1);
        assert_eq!(context.variable_information[1].size_bytes, 4);
        assert_eq!(context.variable_information[2].name, "var3");
        assert_eq!(context.variable_information[2].var_type, VarType::I32);
        assert_eq!(context.variable_information[2].offset_bytes, 5);
        assert_eq!(context.variable_information[2].size_bytes, 4);
    }
}