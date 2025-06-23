use crate::traits::{Receiver, Sender};
use log;
use serialport;
use std::fmt::Debug;
use std::io::Error;
use std::time::Duration;

pub struct SerialSender {
    port: Box<dyn serialport::SerialPort>,
}

impl SerialSender {
    pub fn new(port_name: String, baut_rate: u32, timeout: Duration) -> SerialSender {
        let port = serialport::new(&port_name, baut_rate)
            .timeout(timeout)
            .open().unwrap();
        SerialSender {
            port
        }
    }
}

impl Sender<Error> for SerialSender {
    fn send<T: crate::traits::Sendable + Debug>(&mut self, data: T) -> Result<(), Error> {
        log::info!("send data : {:?}", data);
        let output = data.serialize();
        log::info!("send data serialized : {:?}", output);
        self.port.write(&output)?;
        Ok(())
    }
}

pub struct SerialReceiver {
    port: Box<dyn serialport::SerialPort>,
}

impl SerialReceiver {
    pub fn new(port_name: String, baut_rate: u32, timeout: Duration) -> SerialReceiver {
        let port = serialport::new(&port_name, baut_rate)
            .timeout(timeout)
            .open().unwrap();
        SerialReceiver {
            port
        }
    }
}

impl Receiver<Error> for SerialReceiver {
    fn try_receive<T: crate::traits::Sendable + Debug>(&mut self) -> Result<T, Error> {
        let mut buf: Vec<u8> = vec![0; T::serialized_size()];
        self.port.read(buf.as_mut_slice())?;
        log::info!("receive data serialized : {:?}", buf);
        let data = T::deserialize(&buf);
        log::info!("receive data : {:?}", data);
        Ok(data)
    }
}
#[cfg(test)]
mod test {
    use crate::traits::{Receiver, Sendable, Sender};
    use std::{io, time::Duration};

    use super::{SerialReceiver, SerialSender};
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
    //標準入力でポートを指定する必要がある
    //仮想シリアルポートを使う場合は $ socat -d -d pty,raw,echo=0 pty,raw,echo=0
    #[test]
    #[ignore]
    pub fn serial() {
        let data = TestData { x: 10 };
        let mut port_send = String::new();
        let mut port_receive = String::new();
        io::stdin().read_line(&mut port_send).unwrap();
        port_send = String::from(port_send.trim());
        io::stdin().read_line(&mut port_receive).unwrap();
        port_receive = String::from(port_receive.trim());
        //baut_rate=0はエラー回避のため(https://github.com/serialport/serialport-rs/pull/58)
        let mut sender = SerialSender::new(port_send, 0, Duration::from_secs(1));
        let mut receiver = SerialReceiver::new(port_receive, 0, Duration::from_secs(1));
        sender.send(data).unwrap();
        let response: TestData = receiver.try_receive().unwrap();
        assert_eq!(response.x, 10);
    }
}
