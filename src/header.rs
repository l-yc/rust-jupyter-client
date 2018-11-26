use errors::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Header {
    pub(crate) date: String,
    pub(crate) msg_id: String,
    pub(crate) username: String,
    pub(crate) session: String,
    pub(crate) msg_type: String,
    pub(crate) version: String,
}

impl Header {
    pub(crate) fn new<S>(msg_type: S) -> Header
    where
        S: Into<String>,
    {
        Header {
            // TODO: now
            date: "".to_string(),
            msg_id: msg_id(),
            username: "kernel".to_string(),
            session: "".to_string(),
            msg_type: msg_type.into(),
            version: "5.0".to_string(),
        }
    }

    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>> {
        let s = serde_json::to_string(self)?;
        Ok(s.into_bytes())
    }
}

fn msg_id() -> String {
    let u = uuid::Uuid::new_v4();
    format!("{}", u.to_hyphenated())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_type() {
        let header = Header::new("test");
        assert_eq!(header.msg_type, "test");
    }

    #[test]
    fn test_uuid() {
        // Ensure the msg_id is a proper uuid
        let header = Header::new("test");
        assert_eq!(header.msg_id.len(), 36);
        assert!(header.msg_id.contains("-"));
    }
}
