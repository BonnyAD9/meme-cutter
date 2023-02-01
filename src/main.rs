use cutter::get_cut;
use image::io::Reader as ImageReader;
use std::env;

pub mod cutter;

fn main() {
    let args: Vec<_> = env::args().collect();

    if matches!(
        args[1].to_lowercase().as_str(),
        "help" | "--help" | "-h" | "-?"
    ) {
        help();
        return;
    }

    if args.len() != 4 {
        println!("Invalid number of arguments");
        return;
    }

    let command = args[1].to_lowercase();
    let command = command.as_str();
    let src = args[2].as_str();
    let dest = args[3].as_str();
    let mut t = 30;

    let mut it = args[4..].iter();
    while let Some(a) = it.next() {
        match a.to_lowercase().as_str() {
            "-t" | "--tolerance" => {
                let t_str: &str = if let Some(t_s) = it.next() {
                    t_s.as_str()
                } else {
                    println!("Missing tolerance value (argument '{}')", a);
                    return;
                };

                t = t_str.parse().unwrap_or(u32::MAX);
                if t == u32::MAX {
                    println!(
                        "Invalid tolerance value '{}' (argument '{}'), \
                        expected number",
                        t_str, a
                    );
                    return;
                }
                if t > 1020 {
                    println!(
                        "Invalid tolerance value '{}' (argument '{}'), \
                        value must be smaller than 1020",
                        t_str, a
                    );
                    return;
                }
            }
            _ => {
                println!("Invalid argument '{}'", a);
                return;
            }
        }
    }

    let res = match command {
        "file" => cut_and_save(src, dest, t),
        "directory" => cut_and_save_dir(src, dest, t),
        _ => {
            println!("Invalid action '{}'", command);
            return;
        }
    };

    match res {
        None => {
            println!("Cannot complete oparation");
            return;
        }
        _ => {}
    };
}

fn cut_and_save(src: &str, dest: &str, t: u32) -> Option<()> {
    let img = ImageReader::open(src).ok()?.decode().ok()?;

    get_cut(img.as_rgba8()?, t)?.to_image().save(dest).ok()?;

    Some(())
}

fn cut_and_save_dir(src: &str, dest: &str, t: u32) -> Option<()> {
    None
}

fn help() {
    println!(
        "meme-cutter v0 (in development) by {}{}{}

Usage:
  meme-cutter [action] [input] [output] [flags]

Actions:
  help
    displays this help

  file
    cut single image file

Flags:
  -t  --tolerance
    tolerance, value from 0 to 2010 (inclusive), how much different the
    color must be to be left in the image
", // BonnyAD9 gradient in 3 strings
        "\x1b[38;2;250;50;170mB\x1b[38;2;240;50;180mo\x1b[38;2;230;50;190mn",
        "\x1b[38;2;220;50;200mn\x1b[38;2;210;50;210my\x1b[38;2;200;50;220mA",
        "\x1b[38;2;190;50;230mD\x1b[38;2;180;50;240m9\x1b[0m"
    );
}
