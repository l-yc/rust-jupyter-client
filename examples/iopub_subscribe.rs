extern crate env_logger;
extern crate jupyter_client;
extern crate structopt;

use jupyter_client::Client;
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

    let receiver = client.iopub_subscribe().unwrap();
    for msg in receiver {
        println!("{:?}", msg);
    }
}