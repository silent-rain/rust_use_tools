use std::{
    io::{self, Write},
    net::TcpStream,
};

use tracing::{info, instrument};

#[instrument]
async fn write(stream: &mut TcpStream) -> io::Result<usize> {
    let result = stream.write(b"hello world\n");
    info!("wrote to stream; success={:?}", result.is_ok());
    result
}

fn main() {}