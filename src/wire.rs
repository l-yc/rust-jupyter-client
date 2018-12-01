use errors::Result;
use failure::{bail, format_err};
use header::Header;
use hmac::Mac;
use metadata::Metadata;
use responses::*;
use serde_json::Value;
use signatures::sign;

type Part = Vec<u8>;

static DELIMITER: &[u8] = b"<IDS|MSG>";

pub(crate) struct WireMessage<M: Mac> {
    pub(crate) header: Part,
    pub(crate) parent_header: Part,
    pub(crate) metadata: Part,
    pub(crate) content: Part,
    pub(crate) auth: M,
}

impl<M: Mac> WireMessage<M> {
    pub(crate) fn from_raw_response(raw: Vec<Vec<u8>>, auth: M) -> Result<Self> {
        let delim_idx = raw
            .iter()
            .position(|r| String::from_utf8(r.to_vec()).unwrap() == "<IDS|MSG>")
            .ok_or_else(|| format_err!("cannot find delimiter in response"))?;

        // Check the signature
        let signature = String::from_utf8_lossy(&raw[delim_idx + 1]);
        let msg_frames = &raw[delim_idx + 2..];
        let check_sig = sign(msg_frames, auth.clone());

        if check_sig != signature {
            bail!("signatures do not match");
        }

        Ok(WireMessage {
            header: msg_frames[0].clone(),
            parent_header: msg_frames[1].clone(),
            metadata: msg_frames[2].clone(),
            content: msg_frames[3].clone(),
            auth: auth.clone(),
        })
    }

    pub(crate) fn into_response(self) -> Result<Response> {
        let header_str = std::str::from_utf8(&self.header)?;
        let header: Header = serde_json::from_str(header_str)?;

        let parent_header_str = std::str::from_utf8(&self.parent_header)?;
        let parent_header: Header = serde_json::from_str(parent_header_str)?;

        let metadata_str = std::str::from_utf8(&self.metadata)?;
        let metadata: Metadata = serde_json::from_str(metadata_str)?;

        let content_str = std::str::from_utf8(&self.content)?;

        match header.msg_type.as_str() {
            "kernel_info_reply" => Ok(Response::Shell(ShellResponse::KernelInfo {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "execute_reply" => Ok(Response::Shell(ShellResponse::Execute {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "inspect_reply" => Ok(Response::Shell(ShellResponse::Inspect {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "complete_reply" => Ok(Response::Shell(ShellResponse::Complete {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "history_reply" => Ok(Response::Shell(ShellResponse::History {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "is_complete_reply" => {
                let content_json: Value = serde_json::from_str(content_str)?;
                let content = match content_json["status"] {
                    Value::String(ref s) if s == "complete" => IsCompleteStatus::Complete,
                    Value::String(ref s) if s == "invalid" => IsCompleteStatus::Invalid,
                    Value::String(ref s) if s == "unknown" => IsCompleteStatus::Unknown,
                    Value::String(ref s) if s == "incomplete" => {
                        let indent_node = &content_json["indent"];
                        let indent = String::from(
                            indent_node
                                .as_str()
                                .ok_or(format_err!("response indent value empty"))?,
                        );
                        IsCompleteStatus::Incomplete(indent)
                    }
                    _ => unreachable!(),
                };

                Ok(Response::Shell(ShellResponse::IsComplete {
                    header,
                    parent_header,
                    metadata,
                    content: content,
                }))
            }
            "status" => Ok(Response::IoPub(IoPubResponse::Status {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "execute_input" => Ok(Response::IoPub(IoPubResponse::ExecuteInput {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "stream" => Ok(Response::IoPub(IoPubResponse::Stream {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "error" => Ok(Response::IoPub(IoPubResponse::Error {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            _ => unreachable!("{}", header.msg_type),
        }
    }

    pub(crate) fn into_packets(self) -> Result<Vec<Part>> {
        let mut buf = Vec::with_capacity(4);

        // Start by adding the items that need a signature
        buf.push(self.header);
        buf.push(self.parent_header);
        buf.push(self.metadata);
        buf.push(self.content);

        let signature = sign(buf.as_slice(), self.auth.clone());

        let mut result = Vec::with_capacity(6);
        result.push(DELIMITER.to_vec());
        result.push(signature.into_bytes());
        result.extend_from_slice(&buf);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::Command;
    use crypto_mac::MacResult;
    use digest::generic_array::typenum::U64;
    use generic_array::GenericArray;
    use hmac::Mac;

    #[derive(Debug, Clone)]
    struct FakeAuth;

    static KEY: &[u8] = b"foobar0000000000000000000000000000000000000000000000000000000000";

    impl Mac for FakeAuth {
        type OutputSize = U64;
        type KeySize = U64;

        fn new(_keys: &GenericArray<u8, Self::KeySize>) -> Self {
            FakeAuth {}
        }

        fn input(&mut self, _data: &[u8]) {}
        fn reset(&mut self) {}
        fn result(self) -> MacResult<Self::OutputSize> {
            MacResult::new(GenericArray::clone_from_slice(KEY))
        }
    }

    impl FakeAuth {
        fn create() -> FakeAuth {
            FakeAuth::new_varkey(KEY).expect("creating fake auth object")
        }
    }

    fn expected_signature() -> String {
        let auth = FakeAuth::create();
        let res = auth.result();
        let code = res.code();
        let encoded = hex::encode(code);
        encoded
    }

    macro_rules! compare_bytestrings {
        ($a:expr, $b:expr) => {
            let a = String::from_utf8_lossy($a).into_owned();
            let b = String::from_utf8_lossy($b).into_owned();
            assert_eq!($a, $b, "result {:?} != expected {:?}", a, b);
        };
    }

    #[test]
    fn test_kernel_info_into_packets() {
        use crate::header::Header;
        use serde_json::{json, Value};

        let cmd = Command::KernelInfo;
        let auth = FakeAuth::create();
        let wire = cmd.into_wire(auth.clone()).expect("creating wire message");
        let packets = wire.into_packets().expect("creating packets");

        let mut packets = packets.into_iter();
        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &DELIMITER);

        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &expected_signature().as_bytes());

        let packet = packets.next().unwrap();
        let header_str = std::str::from_utf8(&packet).unwrap();
        let header: Header = serde_json::from_str(header_str).unwrap();

        assert_eq!(header.msg_type, "kernel_info_request");

        // The rest of the packet should be empty maps
        let packet = packets.next().unwrap();
        let parent_header_str = std::str::from_utf8(&packet).unwrap();
        let parent_header: Value = serde_json::from_str(parent_header_str).unwrap();
        assert_eq!(parent_header, json!({}));

        let packet = packets.next().unwrap();
        let metadata_str = std::str::from_utf8(&packet).unwrap();
        let metadata: Value = serde_json::from_str(metadata_str).unwrap();
        assert_eq!(metadata, json!({}));

        let packet = packets.next().unwrap();
        let content_str = std::str::from_utf8(&packet).unwrap();
        let content: Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(content, json!({}));
    }

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
    fn test_execute_request_into_packets() {
        use crate::header::Header;
        use serde_json::{json, Value};
        use std::collections::HashMap;

        let cmd = Command::Execute {
            code: "a = 10".to_string(),
            silent: false,
            store_history: true,
            user_expressions: HashMap::new(),
            allow_stdin: true,
            stop_on_error: false,
        };
        let auth = FakeAuth::create();
        let wire = cmd.into_wire(auth.clone()).expect("creating wire message");
        let packets = wire.into_packets().expect("creating packets");

        let mut packets = packets.into_iter();
        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &DELIMITER);

        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &expected_signature().as_bytes());

        let packet = packets.next().unwrap();
        let header_str = std::str::from_utf8(&packet).unwrap();
        let header: Header = serde_json::from_str(header_str).unwrap();

        assert_eq!(header.msg_type, "execute_request");

        // The rest of the packet should be empty maps
        let packet = packets.next().unwrap();
        let parent_header_str = std::str::from_utf8(&packet).unwrap();
        let parent_header: Value = serde_json::from_str(parent_header_str).unwrap();
        assert_eq!(parent_header, json!({}));

        let packet = packets.next().unwrap();
        let metadata_str = std::str::from_utf8(&packet).unwrap();
        let metadata: Value = serde_json::from_str(metadata_str).unwrap();
        assert_eq!(metadata, json!({}));

        let packet = packets.next().unwrap();
        let content_str = std::str::from_utf8(&packet).unwrap();
        let content: Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(
            content,
            json!({
                "code": "a = 10",
                "silent": false,
                "store_history": true,
                "user_expressions": {},
                "allow_stdin": true,
                "stop_on_error": false,
            })
        );
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
    fn test_is_complete_into_packets() {
        use crate::header::Header;
        use serde_json::{json, Value};

        let cmd = Command::IsComplete {
            code: "a = 10".to_string(),
        };
        let auth = FakeAuth::create();
        let wire = cmd.into_wire(auth.clone()).expect("creating wire message");
        let packets = wire.into_packets().expect("creating packets");

        let mut packets = packets.into_iter();
        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &DELIMITER);

        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &expected_signature().as_bytes());

        let packet = packets.next().unwrap();
        let header_str = std::str::from_utf8(&packet).unwrap();
        let header: Header = serde_json::from_str(header_str).unwrap();

        assert_eq!(header.msg_type, "is_complete_request");

        // The rest of the packet should be empty maps
        let packet = packets.next().unwrap();
        let parent_header_str = std::str::from_utf8(&packet).unwrap();
        let parent_header: Value = serde_json::from_str(parent_header_str).unwrap();
        assert_eq!(parent_header, json!({}));

        let packet = packets.next().unwrap();
        let metadata_str = std::str::from_utf8(&packet).unwrap();
        let metadata: Value = serde_json::from_str(metadata_str).unwrap();
        assert_eq!(metadata, json!({}));

        let packet = packets.next().unwrap();
        let content_str = std::str::from_utf8(&packet).unwrap();
        let content: Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(
            content,
            json!({
            "code": "a = 10",
        })
        );
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
}
