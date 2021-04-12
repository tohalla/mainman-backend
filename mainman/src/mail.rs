use lettre::{
    address::Address,
    message::{header::ContentType, Mailbox, MultiPart, SinglePart},
    transport::smtp::{self, authentication::Credentials},
    Message, SmtpTransport, Transport,
};

use crate::template::TEMPLATES;

fn mailer() -> SmtpTransport {
    SmtpTransport::relay("email-smtp.eu-west-1.amazonaws.com")
        .unwrap()
        .credentials(Credentials::new(
            std::env::var("SMTP_USERNAME").unwrap(),
            std::env::var("SMTP_PASSWORD").unwrap(),
        ))
        .build()
}

pub fn send(
    message: &Message,
) -> Result<smtp::response::Response, smtp::Error> {
    mailer().send(message)
}

pub fn support() -> Mailbox {
    Mailbox {
        name: Some("Support".to_owned()),
        email: Address::new("hallasmaa.touko", "gmail.com").unwrap(),
    }
}

pub fn from_template(name: &str, ctx: &tera::Context) -> MultiPart {
    MultiPart::alternative()
        .singlepart(
            SinglePart::builder().header(ContentType::plaintext()).body(
                TEMPLATES
                    .render(&format!("email/en/{}.txt", name), ctx)
                    .unwrap_or_default(),
            ),
        )
        .singlepart(
            SinglePart::builder().header(ContentType::html()).body(
                TEMPLATES
                    .render(&format!("email/en/{}.html", name), ctx)
                    .unwrap_or_default(),
            ),
        )
}
