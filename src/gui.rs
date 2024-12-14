use std::time::Duration;

use iced::{time, Task, Theme};
use iced::widget::{button, container, text,Row, Column, Text};
use iced::Subscription;

pub enum GUIComponent<TCommand : Clone,TLog : Clone> {
    CommandSendButton {
        label : String,
        command_to_send : TCommand
    },
    TextView {
        label : String,
        new_text : Box<dyn Fn(TLog,&mut String) -> String>,
        text : String
    }
}

#[derive(Debug, Clone)]
pub enum Message<TCommand> {
    SendCommand(TCommand),
    Tick
}

pub struct GUI<TCommand : Clone, TLog : Clone> {
    components : Vec<GUIComponent<TCommand,TLog>>,
    send_command : Box<dyn FnMut(TCommand)>,
    try_receive_log : Box<dyn FnMut()>,
    receive_interval : Duration,
    title : String
}

impl<TCommand : Clone + 'static + std::marker::Send + std::fmt::Debug, TLog : Clone + 'static> GUI<TCommand,TLog> {
    pub fn new(components : Vec<GUIComponent<TCommand,TLog>>, send_command : Box<dyn FnMut(TCommand)>, try_receive_log : Box<dyn FnMut()>, receive_interval : Duration, title : String) -> Self {
        GUI {
            components,
            send_command,
            try_receive_log,
            receive_interval,
            title
        }
    }

    pub fn view(&self) -> Column<Message<TCommand>> {
        let mut column = Column::new();
        for component in &self.components {
            match component {
                GUIComponent::CommandSendButton { label, command_to_send } => {
                    column = column.push(button(Text::new(label)).on_press(Message::SendCommand(command_to_send.clone())));
                }
                GUIComponent::TextView { label, new_text : _ , text} => {
                    let c = container(Row::new().spacing(10).push(Text::new(label.clone())).push(Text::new(text.clone())));
                    column = column.push(c);
                }
            }
        }
        column
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn update(&mut self, message : Message<TCommand>) {
        match message {
            Message::SendCommand(c) => (self.send_command)(c),
            Message::Tick => (self.try_receive_log)()
        }
    }

    pub fn subscription(&self) -> Subscription<Message<TCommand>> {
        time::every(self.receive_interval).map(|_| Message::Tick)
    }

    pub fn run(self) -> Result<(), iced::Error> {
        iced::application(GUI::title, GUI::update, GUI::view).subscription(|x| x.subscription()).run_with(move || (self, Task::none()))
    }
}

pub mod test {
    use super::GUIComponent;

    #[derive(Clone,Debug)]
    struct TestData {
        data : String
    }

    pub fn view_test() {
        let components: Vec<GUIComponent<TestData, TestData>> = vec![
            GUIComponent::TextView { label : String::from("label"), new_text: Box::new(|log, _| log.data.clone()), text: String::from("test1") },
            GUIComponent::CommandSendButton { label: String::from("test button"), command_to_send: TestData { data: String::from("test2") } }
        ];
        let mut gui = super::GUI::new(components, Box::new(|c| { println!("{:?}",c)}), Box::new(|| println!("receive")), std::time::Duration::from_secs(1), String::from("test"));
        gui.run().unwrap();
    }
}

