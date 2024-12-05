use crate::traits::{Sendable,Sender,Receiver};
use std::{thread,time::Duration,fmt::Debug};
pub struct Service<ESender : Debug, EReceiver : Debug,TSender : Sender<ESender>,TReceiver : Receiver<EReceiver>,TReceiveData : Sendable> {
    sender : TSender,
    receiver : TReceiver,
    listener : Box<dyn FnMut(TReceiveData)>,
    on_sender_error : Box<dyn FnMut(ESender)>,
    on_receiver_error : Box<dyn FnMut(EReceiver)>
}

impl<ESender : Debug,EReceiver : Debug,TSender : Sender<ESender>,TReceiver : Receiver<EReceiver>,TReceiveData : Sendable> Service<ESender,EReceiver,TSender,TReceiver,TReceiveData> {
    pub fn new(sender : TSender, receiver : TReceiver, listener : Box<dyn FnMut(TReceiveData)>, on_sender_error : Box<dyn FnMut(ESender)>, on_receiver_error : Box<dyn FnMut(EReceiver)>)
        -> Service<ESender,EReceiver,TSender,TReceiver,TReceiveData> {
        Service{
            sender,
            receiver,
            listener,
            on_sender_error,
            on_receiver_error
        }
    }

    pub fn send<T : Sendable>(&mut self, data : T) {
        let result = self.sender.send(data);
        if let Err(e) = result {
            (self.on_sender_error)(e);
        }
    }

    pub fn start_receive(&mut self,interval : Duration) {
        loop {
            let result = self.receiver.try_receive::<TReceiveData>();
            match result {
                Ok(x) => (self.listener)(x),
                Err(e) => (self.on_receiver_error)(e)
            }
            thread::sleep(interval);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc, sync::mpsc, thread, time::Duration};

    use crate::traits::{Receiver, Sendable, Sender};

    use super::Service;

    #[derive(Clone)]
    struct TestSender {
        sended : Option<Vec<u8>>
    }

    impl Sender<()> for TestSender {
        fn send<T : crate::traits::Sendable>(&mut self, data: T) -> Result<(), ()> {
            match self.sended {
                None => {
                    self.sended = Some(data.serialize());
                    Ok(())
                },
                Some(_) => {
                    Err(())
                }
            }
        }
    }

    struct TestReceiver {
        err : bool
    }

    impl Receiver<()> for TestReceiver {
        fn try_receive<T : crate::traits::Sendable>(&mut self) -> Result<T, ()> {
            if self.err {
                Err(())
            } else {
                Ok(T::deserialize(&vec![0]))
            }
        }
    }

    #[derive(PartialEq,Debug)]
    struct TestSendable {
        data : u8
    }

    impl Sendable for TestSendable {
        fn serialize(&self) -> Vec<u8> {
            vec![self.data]
        }

        fn deserialize(bytes: &Vec<u8>) -> Self {
            TestSendable{data : bytes[0]}
        }

        fn serialized_size() -> usize {
            1
        }
    }

    #[test]
    fn send() {
        let send_error = Rc::new(RefCell::new(false));
        let sender = TestSender{ sended : None};
        let receiver = TestReceiver{ err : false };
        let listener = Box::new(|_ : TestSendable| return);
        let send_error2 = Rc::clone(&send_error);
        let on_sender_error = Box::new(move |_| { *send_error2.borrow_mut() = true; });
        let on_receiver_error = Box::new(|_| return);

        let mut service = Service::new(sender,receiver,listener,on_sender_error,on_receiver_error);
        service.send(TestSendable{data : 1});
        assert_eq!(service.sender.sended, Some(vec![1]));
        assert!(!*send_error.borrow());
        service.send(TestSendable{data : 1});
        assert!(*send_error.borrow());
    }

    #[test]
    fn receive_ok() {
        let (ok_tx,ok_rx) = mpsc::channel();
        let (err_tx,err_rx) = mpsc::channel();
        thread::spawn(move || {
            let sender = TestSender{ sended :None};
            let receiver_ok = TestReceiver{ err : false };
            let listener = Box::new( move |x : TestSendable| ok_tx.send(x).unwrap());
            let on_sender_error = Box::new(|_| return);
            let on_receiver_error = Box::new(move |e| err_tx.send(e).unwrap());
            let mut service = Service::new(sender,receiver_ok,listener,on_sender_error,on_receiver_error);
            service.start_receive(Duration::from_micros(1));
        });
        thread::sleep(Duration::from_millis(10));
        let received = ok_rx.recv().unwrap();
        assert_eq!(received,TestSendable{data : 0});
        let error = err_rx.try_recv();
        assert_eq!(error,Err(mpsc::TryRecvError::Empty));
    }

    #[test]
    fn receive_error() {
        let (ok_tx, ok_rx) = mpsc::channel();
        let (err_tx,err_rx) = mpsc::channel();
        thread::spawn(move || {
            let sender = TestSender{ sended :None};
            let receiver_ok = TestReceiver{ err : true };
            let listener = Box::new( move |x : TestSendable| ok_tx.send(x).unwrap());
            let on_sender_error = Box::new(|_| return);
            let on_receiver_error = Box::new(move |e| err_tx.send(e).unwrap());
            let mut service = Service::new(sender,receiver_ok,listener,on_sender_error,on_receiver_error);
            service.start_receive(Duration::from_micros(1));
        });
        thread::sleep(Duration::from_millis(10));
        let received = ok_rx.try_recv();
        assert_eq!(received,Err(mpsc::TryRecvError::Empty));
        let error = err_rx.try_recv();
        assert_eq!(error,Ok(()));
    }
}