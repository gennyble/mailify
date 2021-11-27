mod cli;

use std::process::exit;

use cli::CliArgs;
use lettre::{
    transport::smtp::{self, authentication::Credentials},
    Message, SmtpTransport, Transport,
};

fn main() {
    let cli = match CliArgs::parse() {
        Some(c) => c,
        None => exit(1),
    };

    match notify(cli) {
        Ok(o) => {
            println!("Message sent! Server response code {}", o.code())
        }
        Err(e) => {
            eprintln!("Message send failure: {}", e);
            exit(1);
        }
    }
}

fn notify(cli: CliArgs) -> Result<smtp::response::Response, smtp::Error> {
    let email = Message::builder()
        .from(cli.from)
        .to(cli.to)
        .subject(cli.subject)
        .body(cli.body.replace("\\n", "\n"))
        .unwrap();

    let creds = Credentials::new(
        cli.config.child_parse("Username").unwrap(),
        cli.config.child_parse("Password").unwrap(),
    );

    let mailer = SmtpTransport::relay(cli.config.child_value("MailServer").unwrap())
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email)
}
