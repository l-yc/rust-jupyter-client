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
    Shell(ShellResponse),
    IoPub(IoPubResponse),
}

#[derive(Debug)]
pub enum ShellResponse {
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
    History {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: HistoryContent,
    },
    IsComplete {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: IsCompleteStatus,
    },
    Shutdown {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: ShutdownContent,
    },
    CommInfo {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: CommInfoContent,
    },
}

#[derive(Debug)]
pub enum IoPubResponse {
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
    pub matches: Vec<String>,
    pub cursor_start: u64,
    pub cursor_end: u64,
    pub metadata: HashMap<String, Value>,
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct HistoryContent {
    pub history: Vec<Value>,
}

#[derive(Deserialize, Debug)]
pub struct ShutdownContent {
    pub restart: bool,
}

#[derive(Deserialize, Debug)]
pub struct CommInfoContent {
    pub comms: HashMap<String, HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IsCompleteStatus {
    Complete,
    Incomplete(String), // argument is the indent value
    Invalid,
    Unknown,
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
