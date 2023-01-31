use cutter::get_cut;
use image::io::Reader as ImageReader;

pub mod cutter;

fn main() {
    let img = ImageReader::open("test.png").unwrap().decode().unwrap();
    get_cut(img.as_rgba8().unwrap(), 30)
        .to_image().save("result.png").expect("");
}
