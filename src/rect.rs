use std::io::prelude::Write;
use std::net::TcpStream;

use std::thread;
use std::thread::JoinHandle;

use rand::thread_rng;
use rand::seq::SliceRandom;


fn draw_rect_slice(host: String, area: Vec<String>) {
    let mut stream = TcpStream::connect(host).expect("Failed to connect!");
    
    let area_string: String = area.into_iter().collect();

    loop {
        stream.write(area_string.as_bytes()).expect("Failed to send message!");
    }
}

pub fn draw_rect(host: &str, color: &str, slices: u8, height: u32, width: u32, offset_x: u32, offset_y: u32, shuffle: bool) {
    let mut area: Vec<String> = Vec::new();

    for x in 0..=width {
        for y in 0..=height {
            let target_x = x + offset_x;
            let target_y = y + offset_y;
            let msg = format!("PX {} {} {}\n", target_x, target_y, color);

            area.push(msg);
        }
    }

    if shuffle {
        area.shuffle(&mut thread_rng());
    }

    let slices = slices as usize;
    
    let len = area.len();
    let mut rest = len % slices;

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    for _ in 1..=slices {
        let host_string = String::from(host);
        let mut slice_count = len / slices;
        if rest > 0 {
            slice_count += 1;
            rest -= 1;
        }

        let mut slice_area: Vec<String> = Vec::new();

        for _ in 1..=slice_count {
            slice_area.push(area.pop().unwrap());
        }

        let t = thread::spawn(move || {
            draw_rect_slice(host_string, slice_area);
        });
        
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}