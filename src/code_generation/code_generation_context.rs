use crate::code_generation::var_type::VarType;
use crate::code_generation::config_parser::{DataDefinition, VariableDefinition};
pub struct VariableInformation {
    pub name: String,
    pub var_type: VarType,
    pub offset_bytes: usize,
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
            total_bytes += var_type.bytes();
            variable_information.push(VariableInformation {
                name,
                var_type,
                offset_bytes,
            });
        }
        CodeGenerationContext {
            name: data_definition.name.clone(),
            variable_information,
            total_bytes,
        }
    }
}