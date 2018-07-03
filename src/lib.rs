#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate subprocess;
use subprocess::{Exec, PopenError, Redirection};

pub struct Notmuch {
    basecommand: String,
    command: NotmuchCommand,
    output: NotmuchOutputType,
    args: Vec<String>,
    caller_args: Option<Vec<String>>,
}

pub enum NotmuchCommand {
    Search,
}

#[derive(Debug)]
pub enum NotmuchOutput {
    Summary(Vec<SummaryData>),
}
pub enum NotmuchOutputType {
    Summary
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryData {
    thread: String,
    timestamp: usize,
    date_relative: String,
    matched: usize,
    total: usize,
    authors: String,
    subject: String,
    tags: Vec<String>,
    query: Vec<Option<String>>,
}

impl Notmuch {
    pub fn new(command: NotmuchCommand, output: NotmuchOutputType) -> Notmuch {
        let subcommand = match command {
            NotmuchCommand::Search => "search",
            _ => "search",
        };
        let output_format = format!(
            "--output={}",
            match output {
                NotmuchOutputType::Summary => "summary",
                _ => "summary",
            }
        );

        Notmuch {
            basecommand: "notmuch-remote".to_owned(),
            command,
            output,
            args: vec![
                subcommand.to_owned(),
                output_format,
                "--format=json".to_owned(),
            ],
            caller_args: None,
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        if self.caller_args.is_none() {
            self.caller_args = Some(vec![arg.to_owned()])
        } else {
            if let Some(ref mut args) = self.caller_args.as_mut() {
                args.push(arg.to_owned());
            }
        }
        self
    }

    pub fn exec(&mut self) -> Result<NotmuchOutput, PopenError> {
        if let Some(ref mut args) = self.caller_args.as_mut() {
            self.args.append(args)
        }
        let cmd = Exec::cmd(&self.basecommand)
            .args(&self.args);

        match cmd.stdout(Redirection::Pipe).capture() {
            Ok(cmd_out) => {
                let threads: Vec<SummaryData> = serde_json::from_str(&cmd_out.stdout_str()).unwrap();
                Ok(NotmuchOutput::Summary(threads))
            },
            Err(cmd_out) => Err(cmd_out)
        }
    }
}
