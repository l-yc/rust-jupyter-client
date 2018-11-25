use errors::Result;
use header::Header;
use signatures::HmacSha256;
use wire::WireMessage;

pub enum Command {
    KernelInfo,
}

impl Command {
    pub(crate) fn into_wire(self, _auth: HmacSha256) -> Result<WireMessage> {
        match self {
            Command::KernelInfo => {
                let header = Header::new("kernel_info");
                let header_bytes = header.to_bytes()?;
                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content: b"{}".to_vec(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
