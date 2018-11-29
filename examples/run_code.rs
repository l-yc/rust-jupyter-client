extern crate env_logger;
extern crate jupyter_client;
extern crate structopt;

use jupyter_client::{Client, Command};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
    #[structopt(name = "command", short = "c")]
    command: String,
}

fn main() {
    env_logger::init();

    let args = Opt::from_args();

    let filename = args.filename;
    let file = File::open(filename).expect("opening jupyter config file");

    let client = Client::from_reader(&file).expect("creating jupyter connection");

    let command = Command::ExecuteRequest {
        code: args.command,
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);
}
