use std::fmt::Display;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SendError;

pub trait Sender<A: Display, M: Display> {
    fn kind() -> String;
    fn send(&self, from: A, to: A, message: M) -> Result<(), SendError>;
}

pub struct EmailMessage {
    pub subject: String,
    pub body: String,
}

impl Display for EmailMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SUBJECT: {}", self.subject)?;
        write!(f, "BODY: {}", self.body)?;

        Ok(())
    }
}

pub struct EmailSender;

impl<A> Sender<A, EmailMessage> for EmailSender
where
    A: Display,
{
    fn kind() -> String {
        return "EMAIL".to_owned();
    }

    fn send(&self, _from: A, _to: A, _message: EmailMessage) -> Result<(), SendError> {
        Ok(())
    }
}

pub struct SmsMessage {
    pub text: String,
}

impl Display for SmsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEXT: {}", self.text)?;
        Ok(())
    }
}

pub struct SmsSender;

impl<A> Sender<A, SmsMessage> for SmsSender
where
    A: Display,
{
    fn kind() -> String {
        return "SMS".to_owned();
    }

    fn send(&self, _from: A, _to: A, _message: SmsMessage) -> Result<(), SendError> {
        Ok(())
    }
}

pub struct AlwaysFailDecorator<'a, A, M, S>
where
    A: Display,
    M: Display,
    S: Sender<A, M>,
{
    _sender: &'a S,
    _a: PhantomData<A>,
    _m: PhantomData<M>,
}

impl<'a, A, M, S> AlwaysFailDecorator<'a, A, M, S>
where
    A: Display,
    M: Display,
    S: Sender<A, M>,
{
    pub fn new(sender: &'a S) -> Self {
        Self {
            _sender: sender,
            _a: Default::default(),
            _m: Default::default(),
        }
    }
}

impl<'a, A, M, S> Sender<A, M> for AlwaysFailDecorator<'a, A, M, S>
where
    A: Display,
    M: Display,
    S: Sender<A, M>,
{
    fn kind() -> String {
        return S::kind();
    }

    fn send(&self, _from: A, _to: A, _message: M) -> Result<(), SendError> {
        Err(SendError)
    }
}

pub struct LogDecorator<'a, A, M, S>
where
    A: Display,
    M: Display,
    S: Sender<A, M>,
{
    sender: &'a S,
    _a: PhantomData<A>,
    _m: PhantomData<M>,
}

impl<'a, A, M, S> LogDecorator<'a, A, M, S>
where
    A: Display,
    M: Display,
    S: Sender<A, M>,
{
    pub fn new(sender: &'a S) -> Self {
        Self {
            sender: sender,
            _a: Default::default(),
            _m: Default::default(),
        }
    }
}

impl<'a, A, M, S> Sender<A, M> for LogDecorator<'a, A, M, S>
where
    A: Display,
    M: Display,
    S: Sender<A, M>,
{
    fn kind() -> String {
        return S::kind();
    }

    fn send(&self, from: A, to: A, message: M) -> Result<(), SendError> {
        println!("TRY SEND MESSAGE");
        println!("SENDER KIND: {}", S::kind());
        println!("FROM: {}", from);
        println!("TO: {}", to);
        println!("{}", message);
        match self.sender.send(from, to, message) {
            Err(e) => println!("STATUS: FAIL\nERROR: {:?}", e),
            Ok(_) => println!("STATUS: SUCCESS"),
        }

        Ok(())
    }
}
