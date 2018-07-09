#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate subprocess;

mod errors;
pub mod search;
pub mod show;

#[derive(Debug)]
pub enum NotmuchOutput {
    SearchOutput(search::data::Output),
    ShowOutput(show::data::Threads),
}
pub type NotmuchResult = std::result::Result<NotmuchOutput, errors::NotmuchError>;

pub trait NotmuchCommand {
    fn exec(&self) -> NotmuchResult;
}

pub const NOTMUCH_BASE_COMMAND_STR: &str = "notmuch-remote";
