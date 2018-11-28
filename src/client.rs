use commands::Command;
use connection_config::ConnectionConfig;
use errors::Result;
use failure::format_err;
use hmac::Mac;
use log::trace;
use responses::Response;
use signatures::HmacSha256;
use std::io::Read;

use socket::Socket;

pub struct Client {
    shell_socket: Socket,
    auth: HmacSha256,
}

impl Client {
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: Read,
    {
        let config: ConnectionConfig = ConnectionConfig::from_reader(reader)?;
        let auth = HmacSha256::new_varkey(config.key.as_bytes())
            .map_err(|e| format_err!("Error constructing HMAC: {:?}", e))?;

        let ctx = zmq::Context::new();

        // Set up the sockets
        trace!("shell port: {}", config.shell_port);
        let shell_socket = ctx.socket(zmq::REQ)?;
        shell_socket.connect(&format!("tcp://localhost:{port}", port = config.shell_port))?;

        // Set up iopub socket
        /*
        trace!("iopub port: {}", config.iopub_port);
        let iopub_socket = ctx.socket(zmq::SUB)?;
        iopub_socket.connect(&format!("tcp://localhost:{port}", port = config.iopub_port))?;
        iopub_socket.set_subscribe("".as_bytes())?;
        */

        Ok(Client {
            shell_socket: Socket(shell_socket),
            auth: auth,
        })
    }

    pub fn send_shell_command(&self, command: Command) -> Result<Response> {
        let wire = command.into_wire(self.auth.clone())?;
        self.shell_socket.send_wire(wire)?;

        let resp_wire = self.shell_socket.recv_wire(self.auth.clone())?;
        Ok(resp_wire.into_response()?)
    }
}
