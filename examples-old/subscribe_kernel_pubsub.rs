extern crate env_logger;
extern crate jupyter_client;

fn main() {
    env_logger::init();

    let filename =
        "/Users/simon/Library/Jupyter/runtime/kernel-7237505c-10cb-4082-a40f-96f16a230da5.json";
    let mut connection = jupyter_client::JupyterConnection::with_connection_file(filename)
        .expect("creating jupyter connection");

    let receiver = connection.subscribe_to_iopub().unwrap();
    loop {
        let msg = receiver.recv().unwrap();
        println!("{:?}", msg);
    }
}
