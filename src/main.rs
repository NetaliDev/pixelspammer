use std::io::prelude::Write;
use std::net::TcpStream;
use std::path::Path;

use std::thread;
use std::thread::JoinHandle;

use image::GenericImageView;

use clap::{Arg, App}; 

fn draw_image_slice(image_path: &String, host: &String, offset_x: u32, offset_y: u32, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
    let img = image::open(Path::new(image_path)).expect("Failed to open image!");

    let mut stream = TcpStream::connect(host).expect("Failed to connect!");

    loop {
        for x in from_x..=to_x {
            for y in from_y..=to_y {
                let px = img.get_pixel(x, y);
                // skip alpha < 10
                if px[3] < 10 {
                    continue;
                }
                let target_x = x + offset_x;
                let target_y = y + offset_y;
                let msg = format!("PX {} {} {:0>2X}{:0>2X}{:0>2X}{:0>2X}\n", target_x, target_y, px[0], px[1], px[2], px[3]);
    
                stream.write(msg.as_bytes()).expect("Failed to send message!");
            }
        }
    }

    //TODO: shutdown on ctrl-c
    //stream.shutdown(Shutdown::Both).expect("Failed to shutdown connection!");
}

fn slice_image(image_path: &str, host: &str, slices: u32, offset_x: u32, offset_y: u32) {
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
            let host_string = String::from(host);

            let t = thread::spawn(move || {
                draw_image_slice(&path_string, &host_string, offset_x, offset_y, from_x, from_y, to_x, to_y)
            });
            
            threads.push(t);
        }
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn main() {
    let matches = App::new("pixelspammer")
        .version("1.0.0")
        .author("Netali <me@netali.de>")
        .about("A simple, multithreaded pixelflut client")
        .arg(Arg::new("host")
            .short('h')
            .long("host")
            .value_name("IP:PORT")
            .about("Pixelflut-Server to connect to (IP:Port)")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new("image")
            .short('i')
            .long("image")
            .value_name("FILE")
            .about("Image file to flood")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new("slices")
            .short('s')
            .long("slices")
            .value_name("SLICES")
            .about("Number of parts in that each axis should be sliced")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new("offset-x")
            .short('x')
            .long("xoffset")
            .value_name("OFFSET")
            .about("Offset on the x-axis")
            .takes_value(true)
            .required(false)
            .default_value("0")
        )
        .arg(Arg::new("offset-y")
            .short('y')
            .long("yoffset")
            .value_name("OFFSET")
            .about("Offset on the y-axis")
            .takes_value(true)
            .required(false)
            .default_value("0")
        )
        .get_matches();

    let image_path = matches.value_of("image").unwrap();
    let host = matches.value_of("host").unwrap();
    let slices: u32 = matches.value_of_t_or_exit("slices");
    let offset_x: u32 = matches.value_of_t_or_exit("offset-x");
    let offset_y: u32 = matches.value_of_t_or_exit("offset-y");

    slice_image(image_path, host, slices, offset_x, offset_y);
}
