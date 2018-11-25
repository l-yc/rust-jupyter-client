use commands::Command;
use errors::Result;
use responses::Response;
use signatures::HmacSha256;

use socket::Socket;

pub struct Client {
    shell_socket: Socket,
    auth: HmacSha256,
}

impl Client {
    pub fn send_shell_command(&self, command: Command) -> Result<Response> {
        let wire = command.into_wire(self.auth.clone())?;
        self.shell_socket.send_wire(wire)?;

        let resp_wire = self.shell_socket.recv_wire()?;
        Ok(resp_wire.into_response(self.auth.clone())?)
    }
}
