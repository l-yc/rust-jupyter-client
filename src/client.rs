use commands::Command;
use connection_config::ConnectionConfig;
use errors::Result;
use failure::format_err;
use hmac::Mac;
use log::{debug, trace};
use responses::Response;
use signatures::HmacSha256;
use std::io::Read;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use socket::Socket;

pub struct Client {
    shell_socket: Socket,
    iopub_socket: Arc<Mutex<Socket>>,
    heartbeat_socket: Arc<Mutex<Socket>>,
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
        trace!("iopub port: {}", config.iopub_port);
        let iopub_socket = ctx.socket(zmq::SUB)?;
        iopub_socket.connect(&format!("tcp://localhost:{port}", port = config.iopub_port))?;
        iopub_socket.set_subscribe("".as_bytes())?;

        trace!("heartbeat port: {}", config.hb_port);
        let heartbeat_socket = ctx.socket(zmq::REQ)?;
        heartbeat_socket.connect(&format!("tcp://localhost:{port}", port = config.hb_port))?;

        Ok(Client {
            shell_socket: Socket(shell_socket),
            iopub_socket: Arc::new(Mutex::new(Socket(iopub_socket))),
            heartbeat_socket: Arc::new(Mutex::new(Socket(heartbeat_socket))),
            auth: auth,
        })
    }

    pub fn send_shell_command(&self, command: Command) -> Result<Response> {
        debug!("Sending shell command: {:?}", command);
        let wire = command.into_wire(self.auth.clone())?;
        self.shell_socket.send_wire(wire)?;

        let resp_wire = self.shell_socket.recv_wire(self.auth.clone())?;
        resp_wire.into_response()
    }

    pub fn iopub_subscribe(&self) -> Result<Receiver<Response>> {
        let (tx, rx) = mpsc::channel();
        let socket = self.iopub_socket.clone();
        let auth = self.auth.clone();

        thread::spawn(move || loop {
            let socket = socket.lock().unwrap();
            let wire = socket.recv_wire(auth.clone()).unwrap();
            let msg = wire.into_response().unwrap();
            tx.send(msg).unwrap();
        });

        Ok(rx)
    }

    pub fn heartbeat_every(&self, seconds: Duration) -> Result<Receiver<()>> {
        let (tx, rx) = mpsc::channel();
        let socket = self.heartbeat_socket.clone();

        thread::spawn(move || loop {
            let socket = socket.lock().unwrap();
            let _msg = socket.heartbeat().unwrap();
            tx.send(()).unwrap();
            thread::sleep(seconds);
        });
        Ok(rx)
    }

    pub fn heartbeat(&self) -> Result<Receiver<()>> {
        self.heartbeat_every(Duration::from_secs(1))
    }
}
