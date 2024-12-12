use hw10::{
    AlwaysFailDecorator, EmailMessage, EmailSender, LogDecorator, Sender, SmsMessage, SmsSender,
};

fn main() {
    println!("EMAIL SENDER EXAMPLE WITH LOGGER DECORATOR EXAMPLE\n");
    let email_sender = EmailSender;
    let email_sender_with_logger = LogDecorator::new(&email_sender);
    let _ = email_sender_with_logger.send(
        "from@test.com",
        "to@test.com",
        EmailMessage {
            subject: "test subject".to_owned(),
            body: "test body".to_owned(),
        },
    );

    println!();

    println!("SMS SENDER EXAMPLE WITH LOGGER DECORATOR EXAMPLE\n");
    let sms_sender = SmsSender;
    let sms_sender_with_logger = LogDecorator::new(&sms_sender);
    let _ = sms_sender_with_logger.send(
        "+79151428460",
        "+79211328490",
        SmsMessage {
            text: "test text".to_owned(),
        },
    );

    println!("EMAIL SENDER EXAMPLE WITH ALWAYS FAIL AND LOGGER DECORATOR EXAMPLE\n");
    let email_always_fail_sender = AlwaysFailDecorator::new(&email_sender);
    let email_sender_with_logger = LogDecorator::new(&email_always_fail_sender);
    let _ = email_sender_with_logger.send(
        "from2@test.com",
        "to2@test.com",
        EmailMessage {
            subject: "test subject 2".to_owned(),
            body: "test body 2".to_owned(),
        },
    );

    println!();

    println!("SMS SENDER EXAMPLE WITH ALWAYS FAIL AND LOGGER DECORATOR EXAMPLE\n");
    let sms_always_fail_sender = AlwaysFailDecorator::new(&sms_sender);
    let sms_sender_with_logger = LogDecorator::new(&sms_always_fail_sender);
    let _ = sms_sender_with_logger.send(
        "+79151428460",
        "+79211328490",
        SmsMessage {
            text: "test text".to_owned(),
        },
    );
}
