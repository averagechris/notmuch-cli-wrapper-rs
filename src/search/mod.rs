pub mod data;

use serde_json;
use subprocess;

pub use self::data::SearchOutputType;
use self::data::{MessageData, Output, SummaryData, ThreadData};
use {NotmuchCommand, NotmuchOutput, NotmuchResult, NOTMUCH_BASE_COMMAND_STR};

#[derive(Debug)]
pub struct Search {
    args: Vec<String>,
    command: String,
    output_type: SearchOutputType,
}

impl Search {
    pub fn new() -> SearchBuilder {
        SearchBuilder {
            command: NOTMUCH_BASE_COMMAND_STR.to_owned(),
            args: vec!["search".to_owned(), "--format=json".to_owned()],
            output_type: None,
        }
    }
    fn parse(&self, command_output: &str) -> NotmuchResult {
        Ok(NotmuchOutput::SearchOutput(match self.output_type {
            SearchOutputType::Messages => {
                let messages: Vec<MessageData> = serde_json::from_str(command_output)?;
                Output::Messages(messages)
            }
            SearchOutputType::Summary => {
                let summaries: Vec<SummaryData> = serde_json::from_str(command_output)?;
                Output::Summary(summaries)
            }
            SearchOutputType::Threads => {
                let threads: Vec<ThreadData> = serde_json::from_str(command_output)?;
                Output::Threads(threads)
            }
        }))
    }
}

impl NotmuchCommand for Search {
    fn exec(&self) -> NotmuchResult {
        let cmd_out = subprocess::Exec::cmd(&self.command)
            .args(&self.args)
            .stdout(subprocess::Redirection::Pipe)
            .capture()?
            .stdout_str();
        self.parse(&cmd_out)
    }
}

pub struct SearchBuilder {
    args: Vec<String>,
    command: String,
    output_type: Option<SearchOutputType>,
}

impl SearchBuilder {
    pub fn build(mut self) -> Search {
        let output_type = self.output_type.unwrap_or(SearchOutputType::Summary);
        let output_flag = format!(
            "--output={}",
            match output_type {
                SearchOutputType::Messages => "messages",
                SearchOutputType::Threads => "threads",
                SearchOutputType::Summary => "search",
            }
        );
        // output_flag must be positioned in args before any search terms
        self.args.insert(1, output_flag);

        Search {
            args: self.args,
            command: self.command,
            output_type,
        }
    }
    pub fn arg(mut self, new_arg: &str) -> SearchBuilder {
        self.args.push(new_arg.to_owned());
        self
    }
    pub fn args(mut self, new_args: &Vec<&str>) -> SearchBuilder {
        for arg in new_args.iter() {
            self = self.arg(arg);
        }
        self
    }
    pub fn and(self) -> SearchBuilder {
        self.arg("and")
    }
    pub fn or(self) -> SearchBuilder {
        self.arg("or")
    }
    pub fn from(self, email_or_name: &str) -> SearchBuilder {
        self.arg(&format!("from:{}", email_or_name))
    }
    pub fn to(self, email_or_name: &str) -> SearchBuilder {
        self.arg(&format!("to:{}", email_or_name))
    }
    pub fn with_tag(self, new_tag: &str) -> SearchBuilder {
        self.arg(&format!("tag:{}", new_tag))
    }
    pub fn with_tags(mut self, new_tags: Vec<&str>) -> SearchBuilder {
        if self.has_args_starting_with("tag:") {
            self = self.and();
        }
        for (i, tag) in new_tags.iter().enumerate() {
            if i % 2 != 0 && i != 0 {
                self = self.and();
            }
            self = self.with_tag(tag);
        }
        self
    }
    pub fn with_output_type(mut self, o: SearchOutputType) -> SearchBuilder {
        self.output_type = Some(o);
        self
    }

    fn has_args_starting_with(&self, s: &str) -> bool {
        self.args.iter().filter(|arg| arg.starts_with(s)).count() > 0
    }
}
