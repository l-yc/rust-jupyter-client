use hmac::{Hmac, Mac};
use sha2::Sha256;

pub(crate) type HmacSha256 = Hmac<Sha256>;

pub(crate) trait SignComputable {
    fn signature(&self, auth: HmacSha256) -> String;
}

impl SignComputable for Vec<Vec<u8>> {
    fn signature(&self, mut auth: HmacSha256) -> String {
        for msg in self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}

impl<'a> SignComputable for Vec<&'a [u8]> {
    fn signature(&self, mut auth: HmacSha256) -> String {
        for msg in self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}
impl<'a> SignComputable for &'a [&'a [u8]] {
    fn signature(&self, mut auth: HmacSha256) -> String {
        for msg in *self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}

impl<'a> SignComputable for &'a [Vec<u8>] {
    fn signature(&self, mut auth: HmacSha256) -> String {
        for msg in *self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}

pub(crate) fn sign<S>(msg_list: S, auth: HmacSha256) -> String
where
    S: SignComputable,
{
    msg_list.signature(auth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signing() {
        let auth = HmacSha256::new_varkey(b"foobar").unwrap();
        let data = vec![&b"a"[..], b"b"];
        let _signature = sign(data, auth);
    }
}
