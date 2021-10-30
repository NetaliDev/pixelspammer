use std::io::prelude::Write;
use std::net::TcpStream;


pub fn draw_circle(host: &str, radius: u32, center_x: u32, center_y: u32, color: &str) {
    // r^2 = x^2 + y^2
    let center_x: f32 = center_x as f32;
    let center_y: f32 = center_y as f32;

    let mut area: Vec<String> = Vec::new();

    for r in 0..=radius {
        for x in 0..=r {
            // y^2 = r^2 - x^2
            let x = x as f32;
    
            let r2: f32 = (r * r) as f32;

            let x2: f32 = x * x;
            let y2: f32 = r2 - x2;
    
            let y = y2.sqrt().round();
    
            let target_x_pos = x + center_x;
            let target_x_neg = -1.0 * x + center_x;
    
            let target_y_pos = y + center_y;
            let target_y_neg = -1.0 * y + center_y;
    
            let target_x: u32 = target_x_pos as u32;
            let target_y: u32 = target_y_pos as u32;
            let msg = format!("PX {} {} {}\n", target_x, target_y, color);
            area.push(msg);
    
            if target_x_neg >= 0.0 {
                let target_x: u32 = target_x_neg as u32;
                let target_y: u32 = target_y_pos as u32;
                let msg = format!("PX {} {} {}\n", target_x, target_y, color);
                area.push(msg);
            }
    
            if target_y_neg >= 0.0 {
                let target_x: u32 = target_x_pos as u32;
                let target_y: u32 = target_y_neg as u32;
                let msg = format!("PX {} {} {}\n", target_x, target_y, color);
                area.push(msg);
            }
    
            if target_x_neg >= 0.0 && target_y_neg >= 0.0 {
                let target_x: u32 = target_x_neg as u32;
                let target_y: u32 = target_y_neg as u32;
                let msg = format!("PX {} {} {}\n", target_x, target_y, color);
                area.push(msg);
            }
        }
    }
    

    let mut stream = TcpStream::connect(host).expect("Failed to connect!");
    loop {
        for pos in area.iter() {
            stream.write(pos.as_bytes()).expect("Failed to send message!");
        }
    }
}