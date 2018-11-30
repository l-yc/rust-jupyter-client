use errors::Result;
use header::Header;
use hmac::Mac;
use serde::{Serialize as SerdeSerialize, Serializer};
use serde_derive::Serialize;
use std::collections::HashMap;
use wire::WireMessage;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Command {
    KernelInfo,
    Execute {
        code: String,
        silent: bool,
        store_history: bool,
        user_expressions: HashMap<String, String>,
        allow_stdin: bool,
        stop_on_error: bool,
    },
    Inspect {
        code: String,
        cursor_pos: u64,
        detail_level: DetailLevel,
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
            }
            r @ Command::Execute { .. } => {
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
            r @ Command::Inspect { .. } => {
                let header = Header::new("inspect_request");
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

#[derive(Debug)]
pub enum DetailLevel {
    Zero,
    One,
}

impl SerdeSerialize for DetailLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DetailLevel::Zero => serializer.serialize_i32(0),
            DetailLevel::One => serializer.serialize_i32(1),
        }
    }
}
