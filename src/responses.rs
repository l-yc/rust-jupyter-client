use header::Header;
use metadata::Metadata;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct HelpLink {
    pub text: String,
    pub url: String,
}

#[derive(Debug)]
pub enum Response {
    KernelInfo {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: KernelInfoContent,
    },
    Execute {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: ExecuteReplyContent,
    },
    Status {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: StatusContent,
    },
    ExecuteInput {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: ExecuteInputContent,
    },
    Stream {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: StreamContent,
    },
    Error {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: ErrorContent,
    },
    Inspect {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: InspectContent,
    },
    Complete {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: CompleteContent,
    },
}

#[derive(Deserialize, Debug)]
pub struct KernelInfoContent {
    pub banner: String,
    pub implementation: String,
    pub implementation_version: String,
    pub protocol_version: String,
    pub status: Status,
    pub help_links: Vec<HelpLink>,
}

#[derive(Deserialize, Debug)]
pub struct ExecuteReplyContent {
    pub status: Status,
    pub execution_count: i64,
    // status == "ok" fields
    pub payload: Option<Vec<HashMap<String, Value>>>,
    pub user_expressions: Option<HashMap<String, Value>>,
    // status == "error" fields
    pub ename: Option<String>,
    pub evalue: Option<String>,
    pub traceback: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct StatusContent {
    pub execution_state: ExecutionState,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionState {
    Busy,
    Idle,
    Starting,
}

#[derive(Deserialize, Debug)]
pub struct ExecuteInputContent {
    pub code: String,
    pub execution_count: i64,
}

#[derive(Deserialize, Debug)]
pub struct InspectContent {
    pub status: Status,
    pub found: bool,
    pub data: HashMap<String, Value>,
    pub metadata: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct StreamContent {
    pub name: StreamType,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorContent {
    pub ename: String,
    pub evalue: String,
    pub traceback: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CompleteContent {
    matches: Vec<String>,
    cursor_start: u64,
    cursor_end: u64,
    metadata: HashMap<String, Value>,
    status: Status,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StreamType {
    Stdout,
    Stderr,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Error,
    Abort,
}
