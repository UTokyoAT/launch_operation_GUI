use crate::traits::{Receiver,Sender};
use std::io::{Error};
use serialport;
use std::time::Duration;

pub struct SerialSender {
    port_name : String,
    baut_rate : u32,
    timeout : Duration
}

impl SerialSender {
    pub fn new(port_name : String, baut_rate : u32,timeout : Duration) -> SerialSender {
        SerialSender {
            port_name,
            baut_rate,
            timeout
        }
    }
}

impl Sender<Error> for SerialSender {
    fn send<T : crate::traits::Sendable>(&mut self, data: T) -> Result<(), Error> {
        let mut port = serialport::new(&self.port_name,self.baut_rate).timeout(self.timeout).open()?;
        let output = data.serialize();
        port.write(&output)?;
        Ok(())
    }
}

pub struct SerialReceiver {
    port_name : String,
    baut_rate : u32,
    timeout : Duration
}

impl SerialReceiver {
    pub fn new(port_name : String, baut_rate : u32,timeout : Duration) -> SerialReceiver {
        SerialReceiver {
            port_name,
            baut_rate,
            timeout
        }
    }
}

impl Receiver<Error> for SerialReceiver {
    fn try_receive<T : crate::traits::Sendable>(&mut self) -> Result<T, Error> {
        let mut port = serialport::new(&self.port_name,self.baut_rate).timeout(self.timeout).open()?;
        let mut buf : Vec<u8> = vec![0; T::serialized_size()];
        port.read(buf.as_mut_slice())?;
        let data = T::deserialize(&buf);
        Ok(data)
    }
}
#[cfg(test)]
mod test {
    use std::{io, time::Duration};
    use crate::traits::{Sendable, Sender,Receiver};

    use super::{SerialReceiver, SerialSender};

    struct TestData {
        x : u8
    }
    impl Sendable for TestData {
        fn serialize(&self) -> Vec<u8> {
            vec![self.x]
        }

        fn deserialize(bytes: &Vec<u8>) -> Self {
            TestData{x : bytes[0]}
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
        let data = TestData { x : 10 };
        let mut port_send = String::new();
        let mut port_receive = String::new();
        io::stdin().read_line(&mut port_send).unwrap();
        port_send = String::from(port_send.trim());
        io::stdin().read_line(&mut port_receive).unwrap();
        port_receive = String::from(port_receive.trim());
        //baut_rate=0はエラー回避のため(https://github.com/serialport/serialport-rs/pull/58)
        let mut sender = SerialSender::new(port_send,0,Duration::from_secs(1));
        let mut receiver = SerialReceiver::new(port_receive,0,Duration::from_secs(1));
        sender.send(data).unwrap();
        let response : TestData = receiver.try_receive().unwrap();
        assert_eq!(response.x,10);
    }
}