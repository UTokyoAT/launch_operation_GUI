use launch_operation_gui::traits::Sendable;
use serde::{Serialize, Deserialize};
use launch_operation_gui::serial_communication;
use launch_operation_gui::server::Server;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    data: i32,
    data2: i32,
}

impl Sendable for Data {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.data.to_le_bytes());
        bytes.extend_from_slice(&self.data2.to_le_bytes());
        bytes
    }

    fn deserialize(bytes: &Vec<u8>) -> Self {
        Self { data: i32::from_le_bytes(bytes[..4].try_into().unwrap()), data2: i32::from_le_bytes(bytes[4..8].try_into().unwrap()) }
    }

    fn serialized_size() -> usize {
        8
    }
}

#[tokio::main]
async fn main() {
    let data1 = Data { data: 10, data2: 20 };
    let data2 = Data { data: 30, data2: 40 };
    let (sender, receiver) = serial_communication::new_pair_mock().unwrap();
    Server::new().add_command("data1", data1).add_command("data2", data2).run(sender, receiver).await;
}