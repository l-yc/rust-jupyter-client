extern crate failure;
extern crate hex;
extern crate hmac;
extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate sha2;
extern crate slice_deque;
extern crate uuid;
extern crate zmq;

#[cfg(test)]
extern crate crypto_mac;
#[cfg(test)]
extern crate digest;
#[cfg(test)]
extern crate generic_array;

mod client;
mod commands;
mod connection_config;
mod errors;
mod header;
mod responses;
mod signatures;
mod socket;
mod wire;

pub use client::Client;
pub use commands::Command;
