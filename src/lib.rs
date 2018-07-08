#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate subprocess;
use subprocess::{Exec, PopenError, Redirection};

pub struct Notmuch {
    pub basecommand: String,
    pub command: NotmuchCommand,
    pub output: NotmuchOutputType,
    pub args: Vec<String>,
    pub caller_args: Option<Vec<String>>,
}

pub enum NotmuchCommand {
    Search,
}

#[derive(Debug)]
pub enum NotmuchOutput {
    Messages(Vec<MessageData>),
    Summary(Vec<SummaryData>),
    Threads(Vec<ThreadData>),
}
pub enum NotmuchOutputType {
    // Files,
    Messages,
    Summary,
    // Tags,
    Threads,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryData {
    thread: ThreadData,
    timestamp: usize,
    date_relative: String,
    matched: usize,
    total: usize,
    authors: String,
    subject: String,
    tags: Vec<String>,
    query: Vec<Option<MessageData>>,
}
type ThreadData = String;
type MessageData = String;

impl Notmuch {
    pub fn new(command: NotmuchCommand, output: NotmuchOutputType) -> Notmuch {
        let subcommand = match command {
            NotmuchCommand::Search => "search",
            // _ => "search",
        };
        let output_format = format!(
            "--output={}",
            match output {
                NotmuchOutputType::Messages => "messages",
                NotmuchOutputType::Summary => "summary",
                NotmuchOutputType::Threads => "threads",
                // _ => "summary",
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
            if let Some(ref mut existing_args) = self.caller_args.as_mut() {
                existing_args.push(arg.to_owned());
            }
        }
        self
    }

    pub fn exec(&mut self) -> Result<NotmuchOutput, PopenError> {
        if let Some(ref mut caller_args) = self.caller_args.as_mut() {
            // combine the default arguments with arguments provided
            // by the caller
            self.args.append(caller_args)
        }
        let cmd = Exec::cmd(&self.basecommand).args(&self.args);

        match cmd.stdout(Redirection::Pipe).capture() {
            Ok(cmd_out) => match self.output {
                NotmuchOutputType::Messages => {
                    let messages: Vec<MessageData> =
                        serde_json::from_str(&cmd_out.stdout_str()).unwrap();
                    return Ok(NotmuchOutput::Messages(messages));
                }
                NotmuchOutputType::Summary => {
                    let summaries: Vec<SummaryData> =
                        serde_json::from_str(&cmd_out.stdout_str()).unwrap();
                    return Ok(NotmuchOutput::Summary(summaries));
                }
                NotmuchOutputType::Threads => {
                    let threads: Vec<ThreadData> =
                        serde_json::from_str(&cmd_out.stdout_str()).unwrap();
                    return Ok(NotmuchOutput::Threads(threads));
                }
            },
            Err(cmd_out) => return Err(cmd_out),
        }
    }
}
