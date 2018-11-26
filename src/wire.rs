use errors::Result;
use hmac::Mac;
use responses::Response;
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
    pub(crate) fn into_response(self) -> Result<Response> {
        unimplemented!()
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

        fn new(keys: &GenericArray<u8, Self::KeySize>) -> Self {
            FakeAuth {}
        }

        fn input(&mut self, data: &[u8]) {}
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
        compare_bytestrings!(&packets[0], &DELIMITER);
        compare_bytestrings!(&packets[1], &expected_signature().as_bytes());

        let header_str = std::str::from_utf8(&packets[2]).unwrap();
        let header: Header = serde_json::from_str(header_str).unwrap();

        assert_eq!(header.msg_type, "kernel_info");

        // The rest of the packet should be empty maps
        let parent_header_str = std::str::from_utf8(&packets[3]).unwrap();
        let parent_header: Value = serde_json::from_str(parent_header_str).unwrap();
        assert_eq!(parent_header, json!({}));

        let metadata_str = std::str::from_utf8(&packets[3]).unwrap();
        let metadata: Value = serde_json::from_str(metadata_str).unwrap();
        assert_eq!(metadata, json!({}));

        let content_str = std::str::from_utf8(&packets[3]).unwrap();
        let content: Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(content, json!({}));
    }
}
