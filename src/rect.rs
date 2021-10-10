use std::io::prelude::Write;
use std::net::TcpStream;

use std::thread;
use std::thread::JoinHandle;

use rand::thread_rng;
use rand::seq::SliceRandom;


fn draw_rect_slice(host: String, area: Vec<String>) {
    let mut stream = TcpStream::connect(host).expect("Failed to connect!");
    loop {
        for pos in area.iter() {
            stream.write(pos.as_bytes()).expect("Failed to send message!");
        }
    }
}

pub fn draw_rect(host: &str, color: &str, slices: u32, height: u32, width: u32, offset_x: u32, offset_y: u32, shuffle: bool) {
    let slice_width = width / slices;
    let slice_height = height / slices;

    let mut slice_width_rest = width % slices;
    let mut slice_height_rest = height % slices;

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    for i in 1..=slices {
        for j in 1..=slices {
            let from_x = (i - 1) * slice_width;
            let from_y = (j - 1) * slice_height;

            let to_x: u32;
            let to_y: u32;
            
            // add division rest to slices evenly
            if slice_width_rest > 0 {
                to_x = i * slice_width + 1;
                slice_width_rest -= 1;
            } else {
                to_x = i * slice_width;
            };

            if slice_height_rest > 0 {
                to_y = j * slice_height + 1;
                slice_height_rest -= 1;
            } else {
                to_y = j * slice_height;
            };

            let mut slice_area: Vec<String> = Vec::new();

            for x in from_x..=to_x {
                for y in from_y..=to_y {
                    let target_x = x + offset_x;
                    let target_y = y + offset_y;
                    let msg = format!("PX {} {} {}\n", target_x, target_y, color);

                    slice_area.push(msg);
                }
            }

            if shuffle {
                slice_area.shuffle(&mut thread_rng());
            }

            let host_string = String::from(host);

            let t = thread::spawn(move || {
                draw_rect_slice(host_string, slice_area);
            });
            
            threads.push(t);
        }
    }

    for t in threads {
        t.join().unwrap();
    }
}