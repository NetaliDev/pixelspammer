use std::path::Path;

use std::thread;
use std::thread::JoinHandle;

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::utils;

use image::GenericImageView;

pub fn draw_image(image_path: &str, host: &str, slices: u8, offset_x: u32, offset_y: u32, shuffle: bool, skip_alpha: u8) {
    let img = image::open(Path::new(image_path)).expect("Failed to open image!");

    let x_max = img.dimensions().0 - 1;
    let y_max = img.dimensions().1 - 1;

    let mut area: Vec<String> = Vec::new();

    for x in 0..=x_max {
        for y in 0..=y_max {
            let px = img.get_pixel(x, y);
            // skip alpha
            if px[3] <= skip_alpha {
                continue;
            }
            let target_x = x + offset_x;
            let target_y = y + offset_y;
            let msg = format!("PX {} {} {:0>2X}{:0>2X}{:0>2X}{:0>2X}\n", target_x, target_y, px[0], px[1], px[2], px[3]);

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
            utils::draw_area(host_string, slice_area);
        });
        
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}