use crate::traits::{Sendable,Sender,Receiver};
use std::fmt::Debug;

pub struct SendService<ESender : Debug, TSender : Sender<ESender>> {
    sender : TSender,
    on_sender_error : Box<dyn FnMut(&ESender)>
}

impl<ESender : Debug, TSender : Sender<ESender>> SendService<ESender,TSender> {
    pub fn new(sender : TSender, on_sender_error : Box<dyn FnMut(&ESender)>) -> SendService<ESender,TSender> {
        SendService {
            sender,
            on_sender_error
        }
    }

    pub fn send<T : Sendable>(&mut self, data : T) -> Result<(),ESender> {
        let result = self.sender.send(data);
        if let Err(e) = &result {
            (self.on_sender_error)(e);
        }
        result
    }
}

pub struct ReceiveService<EReceiver : Debug, TReceiver : Receiver<EReceiver>, TReceiveData : Sendable> {
    receiver : TReceiver,
    listener : Box<dyn FnMut(&TReceiveData)>,
    on_receiver_error : Box<dyn FnMut(&EReceiver)>
}

impl<EReceiver : Debug, TReceiver : Receiver<EReceiver>, TReceiveData : Sendable> ReceiveService<EReceiver,TReceiver,TReceiveData> {
    pub fn new(receiver : TReceiver, listener : Box<dyn FnMut(&TReceiveData)>, on_receiver_error : Box<dyn FnMut(&EReceiver)>) -> ReceiveService<EReceiver,TReceiver,TReceiveData> {
        ReceiveService {
            receiver,
            listener,
            on_receiver_error
        }
    }

    pub fn try_receive(&mut self) -> Result<TReceiveData, EReceiver> {
        let result = self.receiver.try_receive::<TReceiveData>();
        match &result {
            Ok(x) => (self.listener)(x),
            Err(e) => (self.on_receiver_error)(e)
        }
        result
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::traits::{Receiver, Sendable, Sender};

    use super::{ReceiveService, SendService};

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

    #[derive(PartialEq,Debug,Clone)]
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
        let send_error2 = Rc::clone(&send_error);
        let on_sender_error: Box<dyn FnMut(&())> = Box::new(move |_| { *send_error2.borrow_mut() = true; });

        let mut service = SendService::new(sender,on_sender_error);
        let result = service.send(TestSendable{data : 1});
        assert_eq!(result, Ok(()));
        assert_eq!(service.sender.sended, Some(vec![1]));
        assert!(!*send_error.borrow());
        let result2 = service.send(TestSendable{data : 1});
        assert_eq!(result2, Err(()));
        assert!(*send_error.borrow());
    }

    #[test]
    fn receive_ok() {
        let receive_error = Rc::new(RefCell::new(false));
        let receive_error2 = Rc::clone(&receive_error);
        let received = Rc::new(RefCell::new(None));
        let received2 = Rc::clone(&received);
        let receiver_ok = TestReceiver{ err : false };
        let listener = Box::new( move |x : &TestSendable| *received2.borrow_mut() = Some(x.clone()));
        let on_receiver_error: Box<dyn FnMut(&())> = Box::new(move |_| *receive_error2.borrow_mut() = true);
        let mut service = ReceiveService::new(receiver_ok,listener,on_receiver_error);
        let result = service.try_receive();
        assert_eq!(result,Ok(TestSendable{data : 0}));
        assert_eq!(*received.borrow(),Some(TestSendable{data : 0}));
        assert_eq!(*receive_error.borrow(),false);
    }

    #[test]
    fn receive_error() {
        let receive_error = Rc::new(RefCell::new(false));
        let receive_error2 = Rc::clone(&receive_error);
        let received = Rc::new(RefCell::new(None));
        let received2 = Rc::clone(&received);
        let receiver_err = TestReceiver{ err : true };
        let listener = Box::new( move |x : &TestSendable| *received2.borrow_mut() = Some(x.clone()));
        let on_receiver_error: Box<dyn FnMut(&())> = Box::new(move |_| *receive_error2.borrow_mut() = true);
        let mut service = ReceiveService::new(receiver_err,listener,on_receiver_error);
        let result = service.try_receive();
        assert_eq!(result,Err(()));
        assert_eq!(*received.borrow(), None);
        assert_eq!(*receive_error.borrow(), true);
    }
}