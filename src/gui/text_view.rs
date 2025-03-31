use crate::gui::gui::GUIComponent;
use iced::{widget::{Row, Text}, Element};
struct TextView<TLog> {
    label: String,
    new_text: Box<dyn Fn(TLog, &mut String) -> String>,
    text: String,
}

impl<TLog> TextView<TLog> {
    pub fn new(label: String, new_text: Box<dyn Fn(TLog, &mut String) -> String>, text: String) -> Self {
        TextView {
            label,
            new_text,
            text,
        }
    }
}

impl<TCommand,TLog> GUIComponent<TCommand, TLog> for TextView<TLog> {
    fn view(&self) -> Element<Message<TCommand>> {
        container(
            Row::new()
                .spacing(10)
                .push(Text::new(label.clone()))
                .push(Text::new(text.clone())),
        )
    }

    fn update(&mut self, log: TLog) {
        self.text = (self.new_text)(log, &mut self.text);
    }
}