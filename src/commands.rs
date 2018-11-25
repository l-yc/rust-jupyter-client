use errors::Result;
use wire::WireMessage;

pub enum Command {}

impl Command {
    pub(crate) fn into_wire(self) -> Result<WireMessage> {
        unimplemented!()
    }
}
