use errors::Result;
use header::Header;
use hmac::Mac;
use wire::WireMessage;
use std::collections::HashMap;
use serde_derive::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Command {
    KernelInfo,
    ExecuteRequest {
        code: String,
        silent: bool,
        store_history: bool,
        user_expressions: HashMap<String, String>,
        allow_stdin: bool,
        stop_on_error: bool,
    },
}

impl Command {
    pub(crate) fn into_wire<M: Mac>(self, auth: M) -> Result<WireMessage<M>> {
        match self {
            Command::KernelInfo => {
                let header = Header::new("kernel_info_request");
                let header_bytes = header.to_bytes()?;
                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content: b"{}".to_vec(),
                    auth,
                })
            },
            r @ Command::ExecuteRequest { .. } => {
                let header = Header::new("execute_request");
                let header_bytes = header.to_bytes()?;
                let content_str = serde_json::to_string(&r)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signatures::HmacSha256;
    use hmac::Mac;

    #[test]
    fn test_kernel_info_message() {
        let cmd = Command::KernelInfo;
        let auth = HmacSha256::new_varkey(b"foobar").unwrap();
        let wire = cmd.into_wire(auth).unwrap();
        assert_eq!(wire.content, b"{}");
        assert_eq!(wire.metadata, b"{}");
    }
}
