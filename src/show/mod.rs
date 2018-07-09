use serde_json;
use subprocess;

pub mod data;

use self::data::Threads;
use {NotmuchCommand, NotmuchOutput, NotmuchResult, NOTMUCH_BASE_COMMAND_STR};

#[derive(Debug)]
pub struct Show {
    args: Vec<String>,
    command: String,
    // output_type: SearchOutputType,
}

impl Show {
    pub fn new() -> ShowBuilder {
        ShowBuilder {
            command: NOTMUCH_BASE_COMMAND_STR.to_owned(),
            args: vec!["show".to_owned(), "--format=json".to_owned(), "--include-html=true".to_owned()],
        }
    }
    fn parse(&self, command_output: &str) -> NotmuchResult {
        let result: Threads = serde_json::from_str(command_output)?;
        Ok(NotmuchOutput::ShowOutput(result))
        // let result: serde_json::Value = serde_json::from_str(command_output)?;
        // Ok(NotmuchOutput::ShowOutput(result))
    }
}

impl NotmuchCommand for Show {
    fn exec(&self) -> NotmuchResult {
        let cmd_out = subprocess::Exec::cmd(&self.command)
            .args(&self.args)
            .stdout(subprocess::Redirection::Pipe)
            .capture()?
            .stdout_str();
        self.parse(&cmd_out)
    }
}

pub struct ShowBuilder {
    args: Vec<String>,
    command: String,
}

impl ShowBuilder {
    pub fn build(self) -> Show {
        Show {
            args: self.args,
            command: self.command,
        }
    }
    pub fn arg(mut self, new_arg: &str) -> ShowBuilder {
        self.args.push(new_arg.to_owned());
        self
    }
    pub fn args(mut self, new_args: &Vec<&str>) -> ShowBuilder {
        for arg in new_args.iter() {
            self = self.arg(arg);
        }
        self
    }
    pub fn thread(self, thread_id: &str) -> ShowBuilder {
        self.arg(&format!("thread:{}", thread_id))
    }
    pub fn and(self) -> ShowBuilder {
        self.arg("and")
    }
    pub fn or(self) -> ShowBuilder {
        self.arg("or")
    }
    pub fn from(self, email_or_name: &str) -> ShowBuilder {
        self.arg(&format!("from:{}", email_or_name))
    }
    pub fn to(self, email_or_name: &str) -> ShowBuilder {
        self.arg(&format!("to:{}", email_or_name))
    }
    pub fn with_tag(self, new_tag: &str) -> ShowBuilder {
        self.arg(&format!("tag:{}", new_tag))
    }
    pub fn with_tags(mut self, new_tags: Vec<&str>) -> ShowBuilder {
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
    fn has_args_starting_with(&self, s: &str) -> bool {
        self.args.iter().filter(|arg| arg.starts_with(s)).count() > 0
    }
}
