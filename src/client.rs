use commands::Command;
use errors::Result;
use responses::Response;

use socket::Socket;

pub struct Client {
    shell_socket: Socket,
}

impl Client {
    pub fn send_shell_command(&self, command: Command) -> Result<Response> {
        let wire = command.into_wire()?;
        self.shell_socket.send_wire(wire)?;

        let resp_wire = self.shell_socket.recv_wire()?;
        Ok(resp_wire.into_response()?)
    }
}
