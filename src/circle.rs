use std::io::prelude::Write;
use std::net::TcpStream;

use std::thread;
use std::thread::JoinHandle;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn draw_circle_slice(host: String, area: Vec<String>) {
    let mut stream = TcpStream::connect(host).expect("Failed to connect!");
    
    let area_string: String = area.into_iter().collect();

    loop {
        stream.write(area_string.as_bytes()).expect("Failed to send message!");
    }
}

pub fn draw_circle(host: &str, color: &str, slices: u8, radius: u16, center_x: u16, center_y: u16, shuffle: bool) {
    let center_x = center_x as i32;
    let center_y = center_y as i32;

    let r2 = radius * radius;

    let mut area: Vec<String> = Vec::new();

    for x in 0..=radius {
        for y in 0..=radius {
            let x = x as i32;
            let y = y as i32;
            let r2 = r2 as i32;

            let x2 = x * x;
            let y2 = y * y;

            if (x2 + y2) > r2 {
                continue;
            }

            let target_x_pos = x + center_x;
            let target_x_neg = -1 * x + center_x;
    
            let target_y_pos = y + center_y;
            let target_y_neg = -1 * y + center_y;
    
            let msg = format!("PX {} {} {}\n", target_x_pos, target_y_pos, color);
            area.push(msg);
    
            if target_x_neg >= 0 {
                let msg = format!("PX {} {} {}\n", target_x_neg, target_y_pos, color);
                area.push(msg);
            }
    
            if target_y_neg >= 0 {
                let msg = format!("PX {} {} {}\n", target_x_pos, target_y_neg, color);
                area.push(msg);
            }
    
            if target_x_neg >= 0 && target_y_neg >= 0 {
                let msg = format!("PX {} {} {}\n", target_x_neg, target_y_neg, color);
                area.push(msg);
            }
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
            draw_circle_slice(host_string, slice_area);
        });
        
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}