use std::io::{self, Write, Read};
use std::net::TcpStream;

use prost::Message;
use crate::config::GPM_TCP_ADDR;
use crate::import_sgcp;

import_sgcp!();

pub fn make_request(component: sgcp::Component, task: i32) -> io::Result<()> {
    let mut stream = TcpStream::connect(GPM_TCP_ADDR)?;
    println!("Successfully established connection to GPM server");
    
    let mut msg = sgcp::Request::default();
    msg.component = component as i32;
    msg.task_code = match component {
        Component::UndefinedComponent => "UNDEFINED".to_string(),
        Component::Bms => bms::Task::try_from(task).unwrap().as_str_name().to_string(),
        Component::Emg => emg::Task::try_from(task).unwrap().as_str_name().to_string(),
        Component::Maestro => maestro::Task::try_from(task).unwrap().as_str_name().to_string(),
        Component::Telemetry => "".to_string(),
    };

    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();

    println!("Length of encoded message={:?}", msg.encoded_len());

    stream.write(&msg.encoded_len().to_be_bytes())?;
    stream.write(&buf)?;
    stream.flush()?;
    println!("Sent message to component={:?} task_code={:?} with data={:?}", component, msg.task_code, msg.task_data);
    
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {}", response);
    
    Ok(())
}
