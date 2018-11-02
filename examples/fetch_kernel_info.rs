extern crate env_logger;
extern crate jupyter_client;

fn main() {
    env_logger::init();

    let filename =
        "/Users/simon/Library/Jupyter/runtime/kernel-7237505c-10cb-4082-a40f-96f16a230da5.json";
    let mut connection = jupyter_client::JupyterConnection::with_connection_file(filename)
        .expect("creating jupyter connection");
    let info = connection.get_kernel_info().expect("fetching kernel info");
    println!("Result: {:?}", info);
}
