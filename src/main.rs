use std::io::{self, Write, Read};
use std::net::TcpStream;

use prost::Message;

pub mod sgcp {
    include!(concat!(env!("OUT_DIR"), "/sgcp.rs"));
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4760")?;
    println!("Connected to the server!");
    
    let mut msg = sgcp::CommandRequest::default();
    msg.component = sgcp::Component::Emg as i32;
    msg.task_code = 1;

    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    stream.write(&buf)?;
    println!("Sent message");
    
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {}", response);
    
    Ok(())
}