use std::collections::HashMap;
use anyhow::Result;

pub struct CommandParser<T: Clone> {
    command_map: HashMap<String, T>,
}

impl<T: Clone> CommandParser<T> {
    pub fn new() -> CommandParser<T> {
        CommandParser {
            command_map: HashMap::new()
        }
    }

    pub fn add_command(&mut self, name: String, command: T) {
        self.command_map.insert(name, command);
    }

    pub fn parse(&self, name: &str) -> Result<T> {
        let command = self.command_map.get(name).ok_or(anyhow::anyhow!("Command not found: {}", name))?;
        Ok(command.clone())
    }

    pub fn accept_names(&self) -> Vec<String> {
        self.command_map.keys().cloned().collect()
    }
}

mod test {
    use super::*;

    #[test]
    fn test_command_parser() {
        let mut command_parser = CommandParser::new();
        command_parser.add_command("test".to_string(), "test".to_string());
    }

    #[test]
    fn test_accept_names() {
        let mut command_parser = CommandParser::new();
        command_parser.add_command("test".to_string(), "test".to_string());
        assert_eq!(command_parser.accept_names(), vec!["test".to_string()]);
    }

    #[test]
    fn test_parse() {
        let mut command_parser = CommandParser::new();
        command_parser.add_command("test".to_string(), "test".to_string());
        assert_eq!(command_parser.parse("test").unwrap(), "test".to_string());
    }
}