use std::env;
use cutter::get_cut;
use image::io::Reader as ImageReader;

pub mod cutter;

fn main() {
    let args: Vec<_> = env::args().collect();

    if matches!(args[0].to_lowercase().as_str(), "help" | "--help" | "-h" | "-?") {
        help();
        return;
    }

    if args.len() != 4 {
        println!("Invalid number of arguments");
    }

    let command = args[1].as_str();

    if command.to_lowercase() != "file" {
        println!("Invalid command {}", command);
        return;
    }

    let src = args[2].as_str();
    let dest = args[3].as_str();
    let t = 30;

    match cut_and_save(src, dest, t) {
        None => {
            println!("Cannot complete oparation");
            return;
        }
        _ => { }
    };
}

fn cut_and_save(src: &str, dest: &str, t: u32) -> Option<()> {
    let img = ImageReader::open(src).ok()?
        .decode().ok()?;

    get_cut(img.as_rgba8()?, t)?
        .to_image()
        .save(dest).ok()?;

    Some(())
}

fn help() {

}