use errors::Result;
use responses::Response;
use signatures::HmacSha256;

type Part = Vec<u8>;

static DELIMITER: &[u8] = b"<IDS|MSG>";

pub(crate) struct WireMessage {
    pub(crate) header: Part,
    pub(crate) parent_header: Part,
    pub(crate) metadata: Part,
    pub(crate) content: Part,
    pub(crate) auth: HmacSha256,
}

impl WireMessage {
    pub(crate) fn into_response(self) -> Result<Response> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}
