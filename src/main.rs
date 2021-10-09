use std::io::prelude::Write;
use std::net::TcpStream;
use std::net::Shutdown;
use std::path::Path;

use image::GenericImageView;

fn draw_image(image_path: &str, offset_x: u32, offset_y: u32) {
    let img = image::open(Path::new(image_path)).expect("Failed to open image!");

    let x_max = img.dimensions().0;
    let y_max = img.dimensions().1;

    let mut stream = TcpStream::connect("127.0.0.1:1234").expect("Failed to connect!");

    for x in 1..x_max {
        for y in 1..y_max {
            let px = img.get_pixel(x, y);
            let target_x = x + offset_x;
            let target_y = y + offset_y;
            let msg = format!("PX {} {} {:X}{:X}{:X}{:X}\n", target_x, target_y, px[0], px[1], px[2], px[3]);

            stream.write(msg.as_bytes()).expect("Failed to send message!");
        }
    }

    stream.shutdown(Shutdown::Both).expect("Failed to shutdown connection!");
}

fn main() {
    draw_image("/home/netali/pixelflut/femlogo.jpg", 50, 50);
}
