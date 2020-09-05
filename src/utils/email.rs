use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use std::env;

pub fn notification(title: &str, content: &str) {
    let smtp_address = "smtp.gmail.com";
    let email_sender = env::var("EMAIL_SENDER").expect("EMAIL SENDER must be set.");
    let email_password = env::var("EMAIL_PASSWORD").expect("EMAIL PASSWORD must be set.");
    let email_receiver = env::var("EMAIL_RECEIVER").expect("EMAIL RECEIVER must be set.");

    let email = EmailBuilder::new()
        .to(email_receiver)
        .from(email_sender.clone())
        .subject(title.to_string())
        .text(content.to_string())
        .build()
        .unwrap()
        .into();

    let credentials = (email_sender, email_password).into_credentials();

    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();

    let _result = client.send(email);
}
