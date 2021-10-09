use clap::{Arg, App}; 

mod image;

const VERSION: &str = "1.1.0";
const AUTHOR: &str = "Netali <me@netali.de>";

fn main() {
    let matches = App::new("pixelspammer")
        .version(VERSION)
        .author(AUTHOR)
        .about("A simple, multithreaded pixelflut client")
        .arg(Arg::new("host")
            .short('h')
            .long("host")
            .value_name("IP:PORT")
            .about("Pixelflut-Server to connect to (IP:Port)")
            .takes_value(true)
            .required(true)
        )
        .subcommand(
            App::new("image")
                .version(VERSION)
                .author(AUTHOR)
                .about("Image spammer module")
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
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();

    match matches.subcommand_name() {
        Some("image") => {
            let matches = matches.subcommand_matches("image").unwrap();
            let image_path = matches.value_of("image").unwrap();
            let slices: u32 = matches.value_of_t_or_exit("slices");
            let offset_x: u32 = matches.value_of_t_or_exit("offset-x");
            let offset_y: u32 = matches.value_of_t_or_exit("offset-y");
    
            image::slice_image(image_path, host, slices, offset_x, offset_y);
        }
        None => {
            println!("No subcommand specified!");
        }
        _ => {
            println!("Unknown subcommand specified!");
        }
    }
}
