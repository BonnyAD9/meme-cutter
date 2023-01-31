use image::{SubImage, RgbaImage, imageops, Rgba};

// making column iterator for image

struct ImgColIterator<'a> {
    img: &'a RgbaImage,
    pos: u32,
    x: u32
}

impl<'a> Iterator for ImgColIterator<'a> {
    type Item = &'a Rgba<u8>;

    fn next(&mut self) -> Option<&'a Rgba<u8>> {
        if self.pos == self.img.height() {
            Option::None
        } else {
            self.pos += 1;
            Option::Some(&self.img[(self.x, self.pos - 1)])
        }
    }
}

impl<'a> Clone for ImgColIterator<'a> {
    fn clone(&self) -> Self {
        ImgColIterator { img: self.img, pos: self.pos, x: self.x }
    }
}

impl<'a> ImgColIterator<'a> {
    fn new(img: &'a RgbaImage, x: u32) -> Self {
        ImgColIterator { img: img, pos: 0, x: x }
    }
}

struct ColImgIterator<'a> {
    img: &'a RgbaImage,
    pos: u32
}

impl<'a> Iterator for ColImgIterator<'a> {
    type Item = ImgColIterator<'a>;

    fn next(&mut self) -> Option<ImgColIterator<'a>> {
        if self.pos == u32::MAX {
            self.pos = 0
        }
        if self.pos == self.img.width() {
            Option::None
        } else {
            self.pos += 1;
            Some(ImgColIterator::new(self.img, self.pos - 1))
        }
    }
}
impl<'a> ColImgIterator<'a> {
    fn new(img: &'a RgbaImage) -> ColImgIterator {
        ColImgIterator { img: img, pos: u32::MAX }
    }
}

impl<'a> DoubleEndedIterator for ColImgIterator<'a> {
    fn next_back(&mut self) -> Option<ImgColIterator<'a>> {
        if self.pos == u32::MAX {
            self.pos = self.img.width();
        }
        if self.pos == 0 {
            Option::None
        } else {
            self.pos -= 1;
            Some(ImgColIterator::new(self.img, self.pos))
        }
    }
}

fn img_column(img: &RgbaImage) -> ColImgIterator {
    ColImgIterator::new(img)
}

// cutting logic

pub fn get_cut<'i>(img: &'i RgbaImage, t: u32) -> SubImage<&'i RgbaImage> {
    if t > 1020 {
        panic!("t cannot be larger than 1020");
    }

    let top = img.rows().position(|l| has_content(l, t)).unwrap() as u32;
    let bot = img.height() - img.rows().rev().position(|l| has_content(l, t)).unwrap() as u32;
    let left = img_column(img).position(|l| has_content(l, t)).unwrap() as u32;
    let right = img.width() - img_column(img).rev().position(|l| has_content(l, t)).unwrap() as u32;

    imageops::crop_imm(img, left, top, right - left, bot - top)
}

fn has_content<'a, I: Iterator<Item = &'a Rgba<u8>> + Clone>(px: I, t: u32) -> bool {
    let t = t as f64;
    let mut sums = [0; 4];

    let mut len = 0.;

    for p in px.clone() {
        for (s, o) in sums.iter_mut().zip(p.0) {
            *s += o as u32;
        }
        len += 1.;
    };

    let avgs = sums.map(|p| (p as f64) / len);

    for p in px.clone() {
        if avgs.iter().zip(p.0).fold(0.0, |s, (a, c)| s + (a - (c as f64)).abs()) > t {
            return true;
        }
    };

    false
}