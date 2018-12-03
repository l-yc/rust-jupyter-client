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
    ExecuteResult {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: ExecuteResultContent,
    },
    ClearOutput {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: ClearOutputContent,
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
    pub status: Status,
    pub matches: Vec<String>,
    pub cursor_start: u64,
    pub cursor_end: u64,
    pub metadata: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct HistoryContent {
    pub status: Status,
    pub history: Vec<Value>,
}

#[derive(Deserialize, Debug)]
pub struct ShutdownContent {
    pub status: Status,
    pub restart: bool,
}

#[derive(Deserialize, Debug)]
pub struct CommInfoContent {
    pub status: Status,
    pub comms: HashMap<String, HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct ExecuteResultContent {
    pub data: HashMap<String, String>,
    pub metadata: Value,
    pub execution_count: i64,
}

#[derive(Deserialize, Debug)]
pub struct ClearOutputContent {
    pub wait: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionState {
    Busy,
    Idle,
    Starting,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use crate::wire::WireMessage;

    #[test]
    fn test_kernel_info_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "kernel_info_reply",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "kernel_info_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "banner": "banner",
                "implementation": "implementation",
                "implementation_version": "implementation_version",
                "protocol_version": "protocol_version",
                "status": "ok",
                "help_links": [{"text": "text", "url": "url"}]
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::KernelInfo {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "kernel_info_reply");

                // Check the content
                assert_eq!(content.banner, "banner");
                assert_eq!(content.implementation, "implementation");
                assert_eq!(content.implementation_version, "implementation_version");
                assert_eq!(content.protocol_version, "protocol_version");
                assert_eq!(content.status, Status::Ok);
                assert_eq!(
                    content.help_links,
                    vec![HelpLink {
                        text: "text".to_string(),
                        url: "url".to_string(),
                    }]
                );
            }
            _ => unreachable!("Incorrect response type, should be KernelInfo"),
        }
    }

    #[test]
    fn test_execute_request_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_reply",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "execution_count": 4
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::Execute {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "execute_reply");

                // Check the content
                assert_eq!(content.status, Status::Ok);
                assert_eq!(content.execution_count, 4);
            }
            _ => unreachable!("Incorrect response type, should be KernelInfo"),
        }
    }

    #[test]
    fn test_status_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "status",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header, not relevant
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "execution_state": "busy"
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::Status {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "status");

                // Check the content
                assert_eq!(content.execution_state, ExecutionState::Busy);
            }
            _ => unreachable!("Incorrect response type, should be Status"),
        }
    }

    #[test]
    fn test_execute_input_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_input",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header, not relevant
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "code": "a = 10",
                "execution_count": 4
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::ExecuteInput {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "execute_input");

                // Check the content
                assert_eq!(content.code, "a = 10");
                assert_eq!(content.execution_count, 4);
            }
            _ => unreachable!("Incorrect response type, should be Status"),
        }
    }

    #[test]
    fn test_stream_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "stream",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header, not relevant
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "name": "stdout",
                "text": "10"
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::Stream {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "stream");

                // Check the content
                assert_eq!(content.name, StreamType::Stdout);
                assert_eq!(content.text, "10");
            }
            _ => unreachable!("Incorrect response type, should be Stream"),
        }
    }

    #[test]
    fn test_is_complete_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_reply",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "complete"
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::IsComplete {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "is_complete_reply");

                // Check the content
                assert_eq!(content, IsCompleteStatus::Complete);
            }
            _ => unreachable!("Incorrect response type, should be IsComplete"),
        }
    }

    #[test]
    fn test_is_complete_message_parsing_with_incomplete_reply() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_reply",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "incomplete",
                "indent": "  "
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::IsComplete {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "is_complete_reply");

                // Check the content
                assert_eq!(content, IsCompleteStatus::Incomplete("  ".to_string()));
            }
            _ => unreachable!("Incorrect response type, should be IsComplete"),
        }
    }

    #[test]
    fn test_shutdown_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "shutdown_reply",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "kernel_info_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "restart": false
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::Shutdown {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "shutdown_reply");

                // Check the content
                assert_eq!(content.restart, false);
            }
            _ => unreachable!("Incorrect response type, should be KernelInfo"),
        }
    }

    #[test]
    fn test_comm_info_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "comm_info_reply",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "comm_info_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "comms": {
                    "u-u-i-d": {
                        "target_name": "foobar"
                    }
                }
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::CommInfo {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "comm_info_reply");

                // Check the content
                assert_eq!(content.comms["u-u-i-d"]["target_name"], "foobar");
            }
            _ => unreachable!("Incorrect response type, should be CommInfo"),
        }
    }

    #[test]
    fn test_execute_result_message_parsing() {
        use serde_json::json;

        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_result",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "data": {
                    "text/plain": "10"
                },
                "metadata": {},
                "execution_count": 46
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::ExecuteResult {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "execute_result");

                // Check the content
                assert_eq!(content.data["text/plain"], "10");
                assert_eq!(content.metadata, json!({}));
                assert_eq!(content.execution_count, 46);
            }
            _ => unreachable!("Incorrect response type, should be ExecuteResult"),
        }
    }

    #[test]
    fn test_clear_output_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "clear_output",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#.to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "wait": false
            }"#.to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::ClearOutput {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "clear_output");

                // Check the content
                assert_eq!(content.wait, false);
            }
            _ => unreachable!("Incorrect response type, should be ClearOutput"),
        }
    }
}
