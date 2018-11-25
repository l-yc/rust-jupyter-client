use errors::Result;
use wire::WireMessage;
use zmq;

pub(crate) struct Socket(pub zmq::Socket);

impl Socket {
    pub(crate) fn send_wire(&self, wire: WireMessage) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn recv_wire(&self) -> Result<WireMessage> {
        unimplemented!()
    }
}
