use errors::Result;
use hmac::Mac;
use responses::Response;
use signatures::sign;
use slice_deque::SliceDeque;

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

    pub(crate) fn into_packets(self) -> Result<SliceDeque<Part>> {
        let mut result = SliceDeque::new();

        // Start by adding the items that need a signature
        result.push_back(self.header);
        result.push_back(self.parent_header);
        result.push_back(self.metadata);
        result.push_back(self.content);

        let signature = sign(result.as_slice(), self.auth.clone());
        result.push_front(signature.into_bytes());
        result.push_front(DELIMITER.to_vec());
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

        fn input(&mut self, data: &[u8]) {
            println!("Adding data {:?}", data);
        }
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

    #[test]
    fn test_kernel_info_into_packets() {
        let cmd = Command::KernelInfo;
        let auth = FakeAuth::create();
        let wire = cmd.into_wire(auth.clone()).expect("creating wire message");
        let packets = wire.into_packets().expect("creating packets");
        assert_eq!(packets[0], DELIMITER);
        assert_eq!(packets[1], expected_signature().as_bytes());
        // TODO: check the rest
    }
}
