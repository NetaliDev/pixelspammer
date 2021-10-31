use std::io::prelude::Write;
use std::net::TcpStream;

pub fn draw_area(host: String, area: Vec<String>) {
    let mut stream = TcpStream::connect(host).expect("Failed to connect!");
    
    let area_string: String = area.into_iter().collect();

    loop {
        stream.write(area_string.as_bytes()).expect("Failed to send message!");
    }
}