use errors::Result;
use responses::Response;

pub(crate) struct WireMessage;

impl WireMessage {
    pub(crate) fn into_response(self) -> Result<Response> {
        unimplemented!()
    }
}
