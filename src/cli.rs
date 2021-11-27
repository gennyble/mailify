use std::{fs::read_to_string, path::PathBuf};

use confindent::Confindent;
use getopts::Options;
use lettre::message::Mailbox;

pub struct CliArgs {
    pub config: Confindent,
    pub from: Mailbox,
    pub to: Mailbox,
    pub subject: String,
    pub body: String,
}

impl CliArgs {
    fn usage<S: AsRef<str>>(program_name: S, opts: &Options) {
        let brief = format!("usage: {} FILE [options]", program_name.as_ref());
        eprint!("{}", opts.usage(&brief));
    }

    pub fn parse() -> Option<Self> {
        let prgm = std::env::args().next().unwrap();
        let args: Vec<String> = std::env::args().skip(1).collect();

        let mut opts = Options::new();
        opts.optopt("c", "config", "Path to read the config from", "PATH");
        opts.optopt(
            "f",
            "from",
            "The message sender.\nFrom Name <email@example.com>",
            "MAILBOX",
        );
        opts.optopt(
            "t",
            "to",
            "The message receiver.\nTo Name <email@example.com>",
            "MAILBOX",
        );
        opts.optopt("s", "subject", "The message subject", "STRING");
        opts.optopt(
            "b",
            "body",
            "The message body\nOverrides the body-file option",
            "STRING",
        );
        opts.optopt(
            "",
            "body-file",
            "Path to read the message body from\nOverridden by the body option",
            "FILE",
        );
        opts.optflag("h", "help", "Print this help message");
        let matches = match opts.parse(&args) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("error: {}", e);
                return None;
            }
        };

        if matches.opt_present("help") {
            Self::usage(prgm, &opts);
            return None;
        }

        let config = match matches.opt_str("config") {
            None => {
                eprintln!("Needs a config path!");
                return None;
            }
            Some(path) => Confindent::from_file(path).unwrap(),
        };

        let from = match matches.opt_get("from") {
            Ok(mbox_opt) => {
                if let Some(mbox) = mbox_opt {
                    mbox
                } else {
                    eprintln!("Needs a from address!");
                    return None;
                }
            }
            Err(e) => {
                eprintln!("Failed to parse the from address: {}", e);
                return None;
            }
        };

        let to = match matches.opt_get("to") {
            Ok(mbox_opt) => {
                if let Some(mbox) = mbox_opt {
                    mbox
                } else {
                    eprintln!("Needs a to address!");
                    return None;
                }
            }
            Err(e) => {
                eprintln!("Failed to parse the to address: {}", e);
                return None;
            }
        };

        let subject = match matches.opt_str("subject") {
            Some(s) => s,
            None => {
                eprintln!("Needs a subject!");
                return None;
            }
        };

        let body = match matches.opt_str("body") {
            Some(body) => body,
            None => match matches.opt_str("body-file") {
                None => {
                    eprintln!("Needs a body!");
                    return None;
                }
                Some(path) => read_to_string(path).unwrap(),
            },
        };

        Some(Self {
            config,
            from,
            to,
            subject,
            body,
        })
    }
}
