extern crate env_logger;
extern crate jupyter_client;

use jupyter_client::commands::Command;
use jupyter_client::Client;
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();

    let client = Client::existing().expect("creating jupyter connection");

    let receiver = client.iopub_subscribe().unwrap();
    thread::spawn(move || {
        for msg in receiver {
            println!("{:?}", msg);
        }
    });

    let command = Command::Shutdown { restart: false };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);

    thread::sleep(Duration::from_secs(1));
}
