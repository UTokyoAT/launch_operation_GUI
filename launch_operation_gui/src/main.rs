use launch_operation_gui::traits::Sendable;
use serde::{Serialize, Deserialize};
use launch_operation_gui::serial_communication;
use launch_operation_gui::server::Server;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    data: i32,
}

impl Sendable for Data {
    fn serialize(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }

    fn deserialize(bytes: &Vec<u8>) -> Self {
        Self { data: i32::from_le_bytes(bytes[..4].try_into().unwrap()) }
    }

    fn serialized_size() -> usize {
        4
    }
}

#[tokio::main]
async fn main() {
    let data = Data { data: 10 };
    let (sender, receiver) = serial_communication::new_pair_mock().unwrap();
    Server::new().add_command("data", data).run(sender, receiver).await;
}