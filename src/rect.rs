use std::io::prelude::Write;
use std::net::TcpStream;

pub fn draw_rect(host: &str, color: &str, height: u32, width: u32, offset_x: u32, offset_y: u32) {
    let mut stream = TcpStream::connect(host).expect("Failed to connect!");

    loop {
        for x in 0..=width {
            for y in 0..=height {
                let target_x = x + offset_x;
                let target_y = y + offset_y;
                let msg = format!("PX {} {} {}\n", target_x, target_y, color);

                stream.write(msg.as_bytes()).expect("Failed to send message!");
            }
        }
    }
}