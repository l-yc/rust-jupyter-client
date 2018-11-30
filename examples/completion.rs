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
}

fn main() {
    env_logger::init();

    let args = Opt::from_args();

    let filename = args.filename;
    let file = File::open(filename).expect("opening jupyter config file");

    let client = Client::from_reader(&file).expect("creating jupyter connection");

    // Set up some previous code
    let code = r#"class Foo(object):
    def bar(self):
        return 10

    def baz(self):
        return 20
"#.to_string();
    let prep_cmd = Command::Execute {
        code: code,
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };

    client
        .send_shell_command(prep_cmd)
        .expect("sending command");

    let prep_cmd = Command::Execute {
        code: "a = Foo()".to_string(),
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };

    client
        .send_shell_command(prep_cmd)
        .expect("sending command");

    let command = Command::Completion {
        code: "a.".to_string(),
        cursor_pos: 2,
    };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);
}
