use crate::{
    gui,
    gui::GUIComponent,
    serial_communication::{SerialReceiver, SerialSender},
    service::{ReceiveService, SendService},
    traits::Sendable,
};
use std::io;

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
    let sender = SerialSender::new(read_stdin(), 0, std::time::Duration::from_secs(1));
    let receiver = SerialReceiver::new(read_stdin(), 0, std::time::Duration::from_millis(1));
    let listener: Box<dyn FnMut(&TestData)> = Box::new(|data: &TestData| {
        println!("lister : {:?}", data);
    });
    // let mut service = Service::new(sender, receiver, listener, Box::new(|e| print!("{:?}",e)), Box::new(|e| { print!("{:?}",e)}));
    let mut send_service = SendService::new(sender, Box::new(|e| println!("sendError : {:?}", e)));
    let mut receive_service = ReceiveService::new(
        receiver,
        listener,
        Box::new(|e| println!("ReceiveError{:?}", e)),
    );
    let mut gui = gui::GUI::new(
        components,
        Box::new(move |c| send_service.send(c).unwrap()),
        Box::new(move || receive_service.try_receive()),
        std::time::Duration::from_secs(1),
        String::from("test"),
    );
    gui.run().unwrap();
}
