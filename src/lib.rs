extern crate hmac;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate sha2;
extern crate uuid;
extern crate zmq;
#[macro_use]
extern crate serde_derive;
extern crate hex;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::error::Error;
use std::fs::File;
use std::path::Path;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type HmacSha256 = Hmac<Sha256>;

static DELIMITER: &[u8] = b"<IDS|MSG>";

pub enum Request {
    KernelInfoRequest,
}

pub enum Response {
    KernelInfoResponse,
}

#[derive(Deserialize, Debug)]
pub struct ConnectionConfig {
    pub shell_port: u32,
    pub iopub_port: u32,
    pub stdin_port: u32,
    pub control_port: u32,
    pub hb_port: u32,
    pub ip: String,
    pub key: String,
    pub transport: String,
    pub signature_scheme: String,
    pub kernel_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    date: String,
    msg_id: String,
    username: String,
    session: String,
    msg_type: String,
    version: String,
}

impl Header {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let s = serde_json::to_string(self)?;
        Ok(s.into_bytes())
    }
}

fn msg_id() -> String {
    let u = uuid::Uuid::new_v4();
    format!("{}", u.to_hyphenated())
}

fn new_header<S>(msg_type: S) -> Header
where
    S: Into<String>,
{
    Header {
        date: "".to_string(),
        msg_id: msg_id(),
        username: "kernel".to_string(),
        session: "".to_string(),
        msg_type: msg_type.into(),
        version: "5.0".to_string(),
    }
}

pub struct JupyterConnection {
    socket: zmq::Socket,
    context: zmq::Context,
    key: String,
    auth: HmacSha256,
}

impl JupyterConnection {
    pub fn with_connection_file<P>(path: P) -> Result<JupyterConnection>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let config: ConnectionConfig = serde_json::from_reader(file)?;
        let auth = HmacSha256::new_varkey(config.key.as_bytes())
            .map_err(|e| format!("Error creating auth source {:?}", e))?;

        let ctx = zmq::Context::new();
        let mut socket = ctx.socket(zmq::REQ)?;
        socket.connect(&format!("tcp://localhost:{port}", port = config.shell_port))?;

        Ok(JupyterConnection {
            socket: socket,
            context: ctx,
            key: config.key,
            auth: auth,
        })
    }

    pub fn get_kernel_info(&mut self) -> Result<()> {
        self.send(Request::KernelInfoRequest)?;
        Ok(())
    }

    fn send(&mut self, request: Request) -> Result<Response> {
        match request {
            Request::KernelInfoRequest => {
                let header = new_header("kernel_info_request");
                let header_bytes = header.to_bytes()?;
                let raw_msg_list = vec![header_bytes.as_slice(), b"{}", b"{}", b"{}"];
                let signature = self.sign(raw_msg_list.as_slice());
                let sig_bytes = signature.into_bytes();

                let mut msg_list = Vec::with_capacity(6);
                msg_list.push(DELIMITER);
                msg_list.push(&sig_bytes);
                msg_list.push(raw_msg_list[0]);
                msg_list.push(raw_msg_list[1]);
                msg_list.push(raw_msg_list[2]);
                msg_list.push(raw_msg_list[3]);

                debug_message(&msg_list);
                self.socket.send_multipart(msg_list.as_slice(), 0)?;
                let response = self.socket.recv_multipart(0)?;

                for chunk in &response {
                    let text = String::from_utf8_lossy(chunk.as_slice());
                    trace!("{}", text);
                }
            }
        }
        Ok(Response::KernelInfoResponse)
    }

    fn sign(&mut self, msg_list: &[&[u8]]) -> String {
        let mut auth = self.auth.clone();
        for msg in msg_list {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}

#[inline(always)]
fn debug_message(msg: &[&[u8]]) {
    let strings: Vec<_> = msg.iter().map(|b| String::from_utf8_lossy(b)).collect();
    trace!("sending: {:?}", strings);
}
