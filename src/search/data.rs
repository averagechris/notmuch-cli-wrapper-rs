use serde_json;

#[derive(Debug)]
pub enum Output {
    Messages(Vec<MessageData>),
    Summary(Vec<SummaryData>),
    Threads(Vec<ThreadData>),
    Untyped(serde_json::Value),
}
#[derive(Debug)]
pub enum SearchOutputType {
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
pub type ThreadData = String;
pub type MessageData = String;
