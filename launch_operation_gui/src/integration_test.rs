use crate::{
    gui::gui,
    gui::gui::GUIComponent,
    serial_communication,
    service::{ReceiveService, SendService},
    traits::Sendable,
};
use std::{io, time::Duration};

#[derive(Clone, Debug)]
struct TestData {
    data: i32,
}

impl Sendable for TestData {
    fn serialize(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }

    fn deserialize(bytes: &Vec<u8>) -> Self {
        TestData {
            data: i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        }
    }

    fn serialized_size() -> usize {
        4
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn integration_test() {
    let components = vec![
        GUIComponent::CommandSendButton {
            label: String::from("send 1"),
            command_to_send: TestData { data: 1 },
        },
        GUIComponent::CommandSendButton {
            label: String::from("send 2"),
            command_to_send: TestData { data: 2 },
        },
        GUIComponent::TextView {
            label: String::from("sended data"),
            new_text: Box::new(|log: TestData, _| log.data.to_string()),
            text: String::from("0"),
        },
    ];
    let (sender, receiver) = serial_communication::new_pair("COM10", "COM11", 0, Duration::from_secs(1)).unwrap();
    let listener: Box<dyn FnMut(&TestData)> = Box::new(|data: &TestData| {
        println!("lister : {:?}", data);
    });
    let mut send_service = SendService::new(sender, Box::new(|e| println!("sendError : {:?}", e)));
    let mut receive_service = ReceiveService::new(
        receiver,
        listener,
        Box::new(|e| println!("ReceiveError{:?}", e)),
    );
    let gui = gui::GUI::new(
        components,
        Box::new(move |c| send_service.send(c).unwrap()),
        Box::new(move || receive_service.try_receive()),
        std::time::Duration::from_secs(1),
        String::from("test"),
    );
    gui.run().unwrap();
}
