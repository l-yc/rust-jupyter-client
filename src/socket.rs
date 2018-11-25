use errors::Result;
use hmac::Mac;
use wire::WireMessage;
use zmq;

pub(crate) struct Socket(pub zmq::Socket);

impl Socket {
    pub(crate) fn send_wire<M: Mac>(&self, _wire: WireMessage<M>) -> Result<()> {
        unimplemented!()
    }

    pub(crate) fn recv_wire<M: Mac>(&self, auth: M) -> Result<WireMessage<M>> {
        unimplemented!()
    }
}
