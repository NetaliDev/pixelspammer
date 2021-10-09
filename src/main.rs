use std::io::prelude::Write;
use std::net::TcpStream;
use std::path::Path;

use std::thread;
use std::thread::JoinHandle;

use image::GenericImageView;

fn draw_image_slice(image_path: &String, offset_x: u32, offset_y: u32, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
    let img = image::open(Path::new(image_path)).expect("Failed to open image!");

    let mut stream = TcpStream::connect("127.0.0.1:1234").expect("Failed to connect!");

    loop {
        for x in from_x..=to_x {
            for y in from_y..=to_y {
                let px = img.get_pixel(x, y);
                let target_x = x + offset_x;
                let target_y = y + offset_y;
                let msg = format!("PX {} {} {:X}{:X}{:X}{:X}\n", target_x, target_y, px[0], px[1], px[2], px[3]);
    
                stream.write(msg.as_bytes()).expect("Failed to send message!");
            }
        }
    }

    //TODO: shutdown on ctrl-c
    //stream.shutdown(Shutdown::Both).expect("Failed to shutdown connection!");
}

fn slice_image(image_path: &str, slices: u32, offset_x: u32, offset_y: u32) {
    let img = image::open(Path::new(image_path)).expect("Failed to open image!");

    let x_max = img.dimensions().0 - 1;
    let y_max = img.dimensions().1 - 1;

    let slice_x = x_max / slices;
    let slice_y = y_max / slices;

    let mut slice_x_rest = x_max % slices;
    let mut slice_y_rest = y_max % slices;

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    for i in 1..=slices {
        for j in 1..=slices {
            let from_x = (i - 1) * slice_x + 1;
            let from_y = (j - 1) * slice_y + 1;

            let to_x: u32;
            let to_y: u32;
            
            // add division rest to slices evenly
            if slice_x_rest > 0 {
                to_x = i * slice_x + 1;
                slice_x_rest -= 1;
            } else {
                to_x = i * slice_x;
            };

            if slice_y_rest > 0 {
                to_y = j * slice_y + 1;
                slice_y_rest -= 1;
            } else {
                to_y = j * slice_y;
            };

            let path_string = String::from(image_path);

            let t = thread::spawn(move || {
                draw_image_slice(&path_string, offset_x, offset_y, from_x, from_y, to_x, to_y)
            });
            
            threads.push(t);
        }
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn main() {
    //TODO: get command line args
    slice_image("/home/netali/pixelflut/femlogo.jpg", 4, 350, 350);
}
