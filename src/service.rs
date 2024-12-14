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

    pub fn try_receive(&mut self) {
        let result = self.receiver.try_receive::<TReceiveData>();
        match result {
            Ok(x) => (self.listener)(x),
            Err(e) => (self.on_receiver_error)(e)
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
        let receive_error = Rc::new(RefCell::new(false));
        let receive_error2 = Rc::clone(&receive_error);
        let received = Rc::new(RefCell::new(None));
        let received2 = Rc::clone(&received);
        let sender = TestSender{ sended :None};
        let receiver_ok = TestReceiver{ err : false };
        let listener = Box::new( move |x : TestSendable| *received2.borrow_mut() = Some(x));
        let on_sender_error = Box::new(|_| return);
        let on_receiver_error = Box::new(move |_| *receive_error2.borrow_mut() = true);
        let mut service = Service::new(sender,receiver_ok,listener,on_sender_error,on_receiver_error);
        service.try_receive();
        assert_eq!(*received.borrow(),Some(TestSendable{data : 0}));
        assert_eq!(*receive_error.borrow(),false);
    }

    #[test]
    fn receive_error() {
        let receive_error = Rc::new(RefCell::new(false));
        let receive_error2 = Rc::clone(&receive_error);
        let received = Rc::new(RefCell::new(None));
        let received2 = Rc::clone(&received);
        let sender = TestSender{ sended :None};
        let receiver_err = TestReceiver{ err : true };
        let listener = Box::new( move |x : TestSendable| *received2.borrow_mut() = Some(x));
        let on_sender_error = Box::new(|_| return);
        let on_receiver_error = Box::new(move |_| *receive_error2.borrow_mut() = true);
        let mut service = Service::new(sender,receiver_err,listener,on_sender_error,on_receiver_error);
        service.try_receive();
        assert_eq!(*received.borrow(), None);
        assert_eq!(*receive_error.borrow(), true);
    }
}