use errors::Result;
use signatures::HmacSha256;
use wire::WireMessage;

pub enum Command {
    KernelInfo,
}

impl Command {
    pub(crate) fn into_wire(self, _auth: HmacSha256) -> Result<WireMessage> {
        match self {
            Command::KernelInfo => unimplemented!("KernelInfo => WireMessage"),
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
        let _wire = cmd.into_wire(auth).unwrap();
    }
}
