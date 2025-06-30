use test_serial_echo::Log::Log;
use launch_operation_gui::serial_communication;
use launch_operation_gui::traits::{Receiver, Sender};
use std::io;
use std::time::Duration;
use std::thread::sleep;
use launch_operation_gui::traits::Sendable;

fn main() {
    println!("Enter the port name: ");
    let log = Log {
        var1: 1.0,
        var2: 2.0,
    };
    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    port = port.trim().to_string();
    let (mut sender, mut receiver) = serial_communication::new_pair(&port, &port, 115200, Duration::from_secs(1)).unwrap();
    sleep(Duration::from_millis(2000));
    sender.send(log).unwrap();
    let received_log : Log = receiver.try_receive().unwrap();
    assert_eq!(received_log.var1, 1.0);
    assert_eq!(received_log.var2, 2.0);
}
