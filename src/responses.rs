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
}

#[derive(Deserialize, Debug)]
pub struct KernelInfoContent {
    pub banner: String,
    pub implementation: String,
    pub implementation_version: String,
    pub protocol_version: String,
    pub status: String,
    pub help_links: Vec<HelpLink>,
}

#[derive(Deserialize, Debug)]
pub struct ExecuteReplyContent {
    pub status: String,
    pub execution_count: i64,
    pub payload: Option<Vec<HashMap<String, Value>>>,
    pub user_expressions: Option<HashMap<String, Value>>,
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
pub struct StreamContent {
    pub name: StreamType,
    pub text: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StreamType {
    Stdout,
    Stderr,
}
