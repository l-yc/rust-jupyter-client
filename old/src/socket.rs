use std::ops::Deref;
use zmq;

pub(crate) struct Socket(pub zmq::Socket);

impl Deref for Socket {
    type Target = zmq::Socket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
