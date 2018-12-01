use errors::Result;
use hmac::Mac;
use wire::WireMessage;
use zmq;

pub(crate) enum SocketType {
    Shell,
    IoPub,
    Heartbeat,
}

pub(crate) struct Socket(pub zmq::Socket);

impl Socket {
    pub(crate) fn send_wire<M: Mac>(&self, wire: WireMessage<M>) -> Result<()> {
        let packets = wire.into_packets()?;
        let slices: Vec<_> = packets.iter().map(|v| v.as_slice()).collect();
        self.0.send_multipart(slices.as_slice(), 0)?;
        Ok(())
    }

    pub(crate) fn recv_wire<M: Mac>(&self, auth: M) -> Result<WireMessage<M>> {
        let raw_response = self.0.recv_multipart(0)?;
        WireMessage::from_raw_response(raw_response, auth.clone())
    }

    pub(crate) fn heartbeat(&self) -> Result<()> {
        self.0.send(b"", 0)?;
        let _msg = self.0.recv_msg(0)?;
        Ok(())
    }
}

// #[derive(Clone)]
// pub(crate) struct IoPubSocket(pub Arc<Mutex<Socket>>);

// impl IoPubSocket {
//     pub(crate) fn recv_wire<M: Mac>(&self, auth: M) -> Result<WireMessage<M>> {
//         self.0.lock()?.recv_wire(auth)
//     }
// }
