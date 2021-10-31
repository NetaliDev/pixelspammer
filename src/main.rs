use clap::{Arg, App}; 

mod image;
mod rect;
mod circle;
mod utils;

const VERSION: &str = "1.4.0";
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
        .arg(Arg::new("slices")
            .short('s')
            .long("slices")
            .value_name("SLICES")
            .about("Number of threads used to draw")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new("no-shuffle")
            .long("no-shuffle")
            .about("Disable the shuffling of the pixel draw order")
            .required(false)
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
                .arg(Arg::new("skip-alpha")
                    .long("skip-alpha")
                    .value_name("ALPHA-VALUE")
                    .about("The alpha value at of which the pixel will be skipped if it is below or equal")
                    .takes_value(true)
                    .required(false)
                    .default_value("10")
                )
        )
        .subcommand(
            App::new("rect")
                .version(VERSION)
                .author(AUTHOR)
                .about("Rectangle spammer module")
                .arg(Arg::new("color")
                    .short('c')
                    .long("color")
                    .value_name("RGB-HEX-COLOR")
                    .about("Hex-color to fill the rectangle (with optional alpha value)")
                    .takes_value(true)
                    .required(true)
                )
                .arg(Arg::new("height")
                    .short('h')
                    .long("height")
                    .value_name("HEIGHT")
                    .about("Height of the rectangle")
                    .takes_value(true)
                    .required(true)
                )
                .arg(Arg::new("width")
                    .short('w')
                    .long("width")
                    .value_name("WIDTH")
                    .about("width of the rectangle")
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
        .subcommand(
            App::new("circle")
                .version(VERSION)
                .author(AUTHOR)
                .about("Circle spammer module")
                .arg(Arg::new("color")
                    .short('c')
                    .long("color")
                    .value_name("RGB-HEX-COLOR")
                    .about("Hex-color of the circle (with optional alpha value)")
                    .takes_value(true)
                    .required(true)
                )
                .arg(Arg::new("radius")
                    .short('r')
                    .long("radius")
                    .value_name("RADIUS")
                    .about("Radius of the circle")
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
    let slices: u8 = matches.value_of_t_or_exit("slices");
    let shuffle = !matches.is_present("no-shuffle");

    match matches.subcommand_name() {
        Some("image") => {
            let matches = matches.subcommand_matches("image").unwrap();
            let image_path = matches.value_of("image").unwrap();
            let offset_x: u32 = matches.value_of_t_or_exit("offset-x");
            let offset_y: u32 = matches.value_of_t_or_exit("offset-y");
            let skip_alpha: u8 = matches.value_of_t_or_exit("skip-alpha");
    
            image::draw_image(image_path, host, slices, offset_x, offset_y, shuffle, skip_alpha);
        }
        Some("rect") => {
            let matches = matches.subcommand_matches("rect").unwrap();
            let color = matches.value_of("color").unwrap();
            let height: u32 = matches.value_of_t_or_exit("height");
            let width: u32 = matches.value_of_t_or_exit("width");
            let offset_x: u32 = matches.value_of_t_or_exit("offset-x");
            let offset_y: u32 = matches.value_of_t_or_exit("offset-y");

            rect::draw_rect(host, color, slices, height, width, offset_x, offset_y, shuffle);
        }
        Some("circle") => {
            let matches = matches.subcommand_matches("circle").unwrap();
            let color = matches.value_of("color").unwrap();
            let radius: u16 = matches.value_of_t_or_exit("radius");
            let offset_x: u16 = matches.value_of_t_or_exit("offset-x");
            let offset_y: u16 = matches.value_of_t_or_exit("offset-y");

            circle::draw_circle(host, color, slices, radius, offset_x, offset_y, shuffle);
        }
        None => {
            println!("No subcommand specified!");
        }
        _ => {
            println!("Unknown subcommand specified!");
        }
    }
}
