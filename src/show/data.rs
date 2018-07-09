use serde_json;

pub type Threaders = Vec<Thread>;
pub type Threads = Vec<Threaders>;
pub type Thread = Vec<Message>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: serde_json::Value,
    #[serde(rename = "match")]
    is_match: serde_json::Value,
    excluded: serde_json::Value,
    filename: serde_json::Value,
    timestamp: serde_json::Value,
    date_relative: serde_json::Value,
    tags: serde_json::Value,
    headers: MessageHeaders,
    body: Vec<MessageBody>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageHeaders {
    #[serde(rename = "Subject")]
    subject: serde_json::Value,
    #[serde(rename = "From")]
    from: serde_json::Value,
    #[serde(rename = "To")]
    to: serde_json::Value,
    #[serde(rename = "Date")]
    date: serde_json::Value, // TODO: parse as some sort of datetime object
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MessageBody {
    id: i32,
    #[serde(rename = "content-type")]
    content_type: serde_json::Value,
    content: Vec<ContentObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentObject {
    id: i32,
    #[serde(rename = "content-type")]
    content_type: serde_json::Value,
    content: serde_json::Value,
}
