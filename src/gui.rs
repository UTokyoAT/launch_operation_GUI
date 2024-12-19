use std::time::Duration;

use iced::{time, Task};
use iced::widget::{button, container, Row, Column, Text};
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

pub struct GUI<TCommand : Clone, TLog : Clone, EReceiver : 'static> {
    components : Vec<GUIComponent<TCommand,TLog>>,
    send_command : Box<dyn FnMut(TCommand)>,
    try_receive_log : Box<dyn FnMut() -> Result<TLog, EReceiver>>,
    receive_interval : Duration,
    title : String
}

impl<TCommand : Clone + 'static + std::marker::Send + std::fmt::Debug, TLog : Clone + 'static, EReceiver : 'static> GUI<TCommand,TLog, EReceiver> {
    pub fn new(components : Vec<GUIComponent<TCommand,TLog>>, send_command : Box<dyn FnMut(TCommand)>, try_receive_log : Box<dyn FnMut() -> Result<TLog, EReceiver>>, receive_interval : Duration, title : String) -> Self {
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
            Message::Tick => {
                let result = (self.try_receive_log)();
                if let Ok(log) = result {
                    self.accept_log(log);
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message<TCommand>> {
        time::every(self.receive_interval).map(|_| Message::Tick)
    }

    pub fn run(self) -> Result<(), iced::Error> {
        iced::application(GUI::title, GUI::update, GUI::view).subscription(|x| x.subscription()).run_with(move || (self, Task::none()))
    }

    pub fn accept_log(&mut self, log : TLog) {
        for component in &mut self.components {
            match component {
                GUIComponent::TextView { label : _ , new_text, text } => {
                    *text = new_text(log.clone(),text);
                }
                _ => {}
            }
        }
    }
}

pub mod test_manual {
    use super::GUIComponent;
    #[derive(Clone,Debug)]
    pub struct TestData {
        pub data : String
    }

    pub fn view_test() {
        let components: Vec<GUIComponent<TestData, TestData>> = vec![
            GUIComponent::TextView { label : String::from("label"), new_text: Box::new(|log, _| log.data.clone()), text: String::from("test1") },
            GUIComponent::CommandSendButton { label: String::from("test button"), command_to_send: TestData { data: String::from("test2") } }
        ];
        let mut gui: super::GUI<TestData, TestData, ()> = super::GUI::new(components, Box::new(|c| { println!("{:?}",c)}), Box::new(|| { println!("receive"); Ok(TestData{ data : String::from("test3")}) }), std::time::Duration::from_secs(1), String::from("test"));
        gui.run().unwrap();
    }
}

#[cfg(test)]
pub mod test {
    use std::{cell::RefCell, rc::Rc};
    use crate::gui::GUIComponent;

    use super::test_manual::TestData;

    #[test]
    fn test_send() {
        let components = vec![];
        let send_data = Rc::new(RefCell::new(None));
        let send_data_clone = send_data.clone();
        let mut gui: super::GUI<TestData, TestData, ()> = super::GUI::new(components, Box::new(move |c| { *send_data_clone.borrow_mut() = Some(c)}), Box::new(|| Ok(TestData{data : String::new()})), std::time::Duration::from_secs(1), String::from("test"));
        gui.update(super::Message::SendCommand(TestData { data: String::from("test") }));
        assert_eq!(send_data.borrow().as_ref().unwrap().data, "test");
    }

    #[test]
    fn test_receive() {
        let components = vec![
            GUIComponent::TextView { label: String::from("test"), new_text: Box::new(|log : TestData, _| log.data), text: String::from("ng") }
        ];
        let receive_count = Rc::new(RefCell::new(0));
        let receive_count_clone = receive_count.clone();
        let mut gui: super::GUI<TestData, TestData, ()> = super::GUI::new(components, Box::new(|_| {}), Box::new(move || { *receive_count_clone.borrow_mut() += 1; Ok(TestData{data : String::from("ok")})}), std::time::Duration::from_secs(1), String::from("test"));
        gui.update(super::Message::Tick);
        if let GUIComponent::TextView{text, ..} = &gui.components[0] {
            assert_eq!(text,"ok");
        } else {
            panic!();
        }
        gui.update(super::Message::Tick);
        assert_eq!(*receive_count.borrow(), 2);
    }
}

