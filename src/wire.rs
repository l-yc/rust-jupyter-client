use errors::Result;
use responses::Response;
use signatures::HmacSha256;

pub(crate) struct WireMessage;

impl WireMessage {
    pub(crate) fn into_response(self, _auth: HmacSha256) -> Result<Response> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}
