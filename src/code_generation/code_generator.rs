use minijinja::{Environment, context, value::Value};
use std::collections::HashMap;
use crate::code_generation::var_type::VarType;

use super::code_generation_context::{self, CodeGenerationContext};

fn to_minijinja_value(code_generation_context: CodeGenerationContext, var_type_to_string : Box<dyn Fn(VarType) -> String>) -> Value {
    let mut variable_information = Vec::new();
    for info in code_generation_context.variable_information {
        let var_info = HashMap::from([
            ("name".to_string(), Value::from(info.name)),
            ("var_type".to_string(), Value::from(var_type_to_string(info.var_type))),
            ("offset_bytes".to_string(), Value::from(info.offset_bytes)),
        ]);
        variable_information.push(var_info);
    }
    let context = HashMap::from([
        ("name".to_string(), Value::from(code_generation_context.name)),
        ("variable_information".to_string(), Value::from(variable_information)),
        ("total_bytes".to_string(), Value::from(code_generation_context.total_bytes)),
    ]);
    Value::from(context)
}

pub fn render_template(template: &str, var_type_to_string : Box<dyn Fn(VarType) -> String>, code_generation_context: CodeGenerationContext) -> String {
    let context = to_minijinja_value(code_generation_context, var_type_to_string);
    let env = Environment::new();
    env.render_str(template, context).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code_generation::var_type::VarType;
    use crate::code_generation::code_generation_context::VariableInformation;
    use crate::code_generation::code_generation_context::CodeGenerationContext;

    fn code_generation_context() -> CodeGenerationContext {
        let variable_information = vec![
            VariableInformation {
                name: "var1".to_string(),
                var_type: VarType::I8,
                offset_bytes: 0,
            },
        ];
        CodeGenerationContext {
            name: "TestData".to_string(),
            variable_information,
            total_bytes: 1,
        }
    }

    fn var_type_to_string() -> Box<dyn Fn(VarType) -> String> {
        Box::new(|var_type| match var_type {
            VarType::I8 => "i8".to_string(),
            VarType::I16 => "i16".to_string(),
            VarType::I32 => "i32".to_string(),
            VarType::I64 => "i64".to_string(),
            VarType::U8 => "u8".to_string(),
            VarType::U16 => "u16".to_string(),
            VarType::U32 => "u32".to_string(),
            VarType::U64 => "u64".to_string(),
            VarType::Float => "float".to_string(),
            VarType::Double => "double".to_string(),
            VarType::Bool => "bool".to_string(),
        })
    }

    #[test]
    fn test_to_minijinja_value() {
        let value = to_minijinja_value(code_generation_context(), var_type_to_string());
        assert_eq!(value.get_attr("name").unwrap().to_string(), "TestData");
        assert_eq!(value.get_attr("total_bytes").unwrap().to_string(), "1");
        let variable_information = value.get_attr("variable_information").unwrap();
        assert_eq!(variable_information.len().unwrap(), 1);
        let var_info = variable_information.get_item_by_index(0).unwrap();
        assert_eq!(var_info.get_attr("name").unwrap().to_string(), "var1");
        assert_eq!(var_info.get_attr("var_type").unwrap().to_string(), "i8");
        assert_eq!(var_info.get_attr("offset_bytes").unwrap().to_string(), "0");
    }

    #[test]
    fn test_render_template() {
        let template = r#"
        struct {{ name }} {
            {% for var in variable_information %}
            {{ var.name }}: {{ var.var_type }},
            {% endfor %}
        }
        "#;
        let rendered = render_template(template, var_type_to_string(), code_generation_context());
        assert!(rendered.contains("struct TestData"));
        assert!(rendered.contains("var1: i8"));
    }
}

