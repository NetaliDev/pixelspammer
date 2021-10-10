use std::io::prelude::Write;
use std::net::TcpStream;
use std::path::Path;

use std::thread;
use std::thread::JoinHandle;

use rand::thread_rng;
use rand::seq::SliceRandom;

use image::GenericImageView;

fn draw_image_slice(host: String, area: Vec<String>) {
    let mut stream = TcpStream::connect(host).expect("Failed to connect!");
    loop {
        for pos in area.iter() {
            stream.write(pos.as_bytes()).expect("Failed to send message!");
        }
    }
}

pub fn draw_image(image_path: &str, host: &str, slices: u32, offset_x: u32, offset_y: u32, shuffle: bool, skip_alpha: u8) {
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

            let mut slice_area: Vec<String> = Vec::new();

            for x in from_x..=to_x {
                for y in from_y..=to_y {
                    let px = img.get_pixel(x, y);
                    // skip alpha
                    if px[3] <= skip_alpha {
                        continue;
                    }
                    let target_x = x + offset_x;
                    let target_y = y + offset_y;
                    let msg = format!("PX {} {} {:0>2X}{:0>2X}{:0>2X}{:0>2X}\n", target_x, target_y, px[0], px[1], px[2], px[3]);

                    slice_area.push(msg);
                }
            }

            if shuffle {
                slice_area.shuffle(&mut thread_rng());
            }

            let host_string = String::from(host);

            let t = thread::spawn(move || {
                draw_image_slice(host_string, slice_area)
            });
            
            threads.push(t);
        }
    }

    for t in threads {
        t.join().unwrap();
    }
}