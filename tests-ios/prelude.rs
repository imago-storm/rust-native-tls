extern crate native_tls;

pub use native_tls::*;

use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;

macro_rules! p {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        }
    }
}
