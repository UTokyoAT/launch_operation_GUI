use crate::traits::{Receiver, Sender};
use log;
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use tokio_serial::{SerialStream, SerialPortBuilder};
use async_trait::async_trait;
use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::io::{ReadHalf, WriteHalf};

pub struct SerialSender {
    stream: WriteHalf<SerialStream>,
}

impl SerialSender {
    pub fn new(stream: WriteHalf<SerialStream>) -> SerialSender {
        SerialSender {
            stream
        }
    }
}

#[async_trait]
impl Sender for SerialSender {
    async fn send<T: crate::traits::Sendable + Debug + Send>(&mut self, data: T) -> Result<()> {
        log::info!("send data : {:?}", data);
        let output = data.serialize();
        log::info!("send data serialized : {:?}", output);
        self.stream.write_all(&output).await?;
        Ok(())
    }
}

pub struct SerialReceiver {
    stream: ReadHalf<SerialStream>,
}

impl SerialReceiver {
    pub fn new(stream: ReadHalf<SerialStream>) -> SerialReceiver {
        SerialReceiver {
            stream
        }
    }
}

#[async_trait]
impl Receiver for SerialReceiver {
    async fn receive<T: crate::traits::Sendable + Debug + Send>(&mut self) -> Result<T> {
        let mut buf: Vec<u8> = vec![0; T::serialized_size()];
        self.stream.read_exact(&mut buf).await?;
        log::info!("receive data serialized : {:?}", buf);
        let data = T::deserialize(&buf);
        log::info!("receive data : {:?}", data);
        Ok(data)
    }
}

pub fn new_pair(
    port_name: &str,
    baut_rate: u32,
) -> Result<(SerialSender, SerialReceiver)> {
    let builder = tokio_serial::new(port_name, baut_rate);
    let stream = SerialStream::open(&builder)?;
    let (reader, writer) = tokio::io::split(stream);
    Ok((SerialSender::new(writer), SerialReceiver::new(reader)))
}

pub fn new_pair_mock() -> Result<(SerialSender, SerialReceiver)> {
    let (stream1, stream2) = SerialStream::pair()?;
    let (_, writer) = tokio::io::split(stream1);
    let (reader, _) = tokio::io::split(stream2);
    Ok((SerialSender::new(writer), SerialReceiver::new(reader)))
}
#[cfg(test)]
mod test {
    use crate::traits::{Receiver, Sendable, Sender};
    use std::{io, time::Duration};

    #[derive(Debug)]
    struct TestData {
        x: u8,
    }
    impl Sendable for TestData {
        fn serialize(&self) -> Vec<u8> {
            vec![self.x]
        }

        fn deserialize(bytes: &Vec<u8>) -> Self {
            TestData { x: bytes[0] }
        }

        fn serialized_size() -> usize {
            1
        }
    }
    #[tokio::test]
    pub async fn serial() {
        let test_data = TestData { x: 10 };
        let (mut sender, mut receiver) = super::new_pair_mock().unwrap();
        sender.send(test_data).await.unwrap();
        let response: TestData = receiver.receive().await.unwrap();
        assert_eq!(response.x, 10);
    }
}
