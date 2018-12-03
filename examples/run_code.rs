extern crate env_logger;
extern crate jupyter_client;
extern crate structopt;

use jupyter_client::commands::Command;
use jupyter_client::Client;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "command", short = "c")]
    command: String,
}

fn main() {
    env_logger::init();

    let args = Opt::from_args();

    let client = Client::existing().expect("creating jupyter connection");

    let command = Command::Execute {
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
