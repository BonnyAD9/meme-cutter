use image::{imageops, DynamicImage, GenericImageView, Rgba, SubImage};
use Direction::{Horizontal, Vertical};

// making iterator for the image
enum Direction {
    Horizontal,
    Vertical,
}

struct ImgLineIterator<'a> {
    img: &'a DynamicImage,
    y: u32,
    x: u32,
    xadd: u32,
    yadd: u32,
}

impl<'a> Iterator for ImgLineIterator<'a> {
    type Item = Rgba<u8>;

    fn next(&mut self) -> Option<Rgba<u8>> {
        if self.x == u32::MAX {
            self.x = 0;
        }
        if self.y == u32::MAX {
            self.y = 0;
        }
        if self.y + self.yadd > self.img.height()
            || self.x + self.xadd > self.img.width()
        {
            Option::None
        } else {
            let res = Option::Some(self.img.get_pixel(self.x, self.y));
            self.x += self.xadd;
            self.y += self.yadd;
            res
        }
    }
}

impl<'a> DoubleEndedIterator for ImgLineIterator<'a> {
    fn next_back(&mut self) -> Option<Rgba<u8>> {
        if self.x == u32::MAX {
            self.x = self.img.width();
        }
        if self.y == u32::MAX {
            self.y = self.img.height();
        }
        if self.y - self.yadd == 0 || self.x - self.xadd == 0 {
            Option::None
        } else {
            self.x -= self.xadd;
            self.y -= self.yadd;
            Option::Some(self.img.get_pixel(self.x, self.y))
        }
    }
}

impl<'a> Clone for ImgLineIterator<'a> {
    fn clone(&self) -> Self {
        ImgLineIterator {
            img: self.img,
            y: self.y,
            x: self.x,
            xadd: self.xadd,
            yadd: self.yadd,
        }
    }
}

impl<'a> ImgLineIterator<'a> {
    fn new(
        img: &'a DynamicImage,
        x: u32,
        y: u32,
        direction: Direction,
    ) -> Self {
        match direction {
            Horizontal => ImgLineIterator {
                img,
                x: u32::MAX,
                y,
                xadd: 1,
                yadd: 0,
            },
            Vertical => ImgLineIterator {
                img,
                x,
                y: u32::MAX,
                xadd: 0,
                yadd: 1,
            },
        }
    }
}

fn img_iter(
    img: &DynamicImage,
    direction: Direction,
    p: u32,
) -> ImgLineIterator {
    ImgLineIterator::new(img, p, p, direction)
}

// cutting logic

pub fn get_cut<'i>(
    img: &'i DynamicImage,
    t: u32,
) -> Option<SubImage<&'i DynamicImage>> {
    if t > 1020 {
        panic!("t cannot be larger than 1020");
    }

    let top = (0..img.height())
        .map(|n| img_iter(img, Horizontal, n))
        .position(|l| has_content(l, t))? as u32;

    let bot = img.height()
        - (0..img.height())
            .rev()
            .map(|n| img_iter(img, Horizontal, n))
            .position(|l| has_content(l, t))? as u32;

    let left = (0..img.width())
        .map(|n| img_iter(img, Vertical, n))
        .position(|l| has_content(l, t))? as u32;

    let right = img.width()
        - (0..img.width())
            .rev()
            .map(|n| img_iter(img, Vertical, n))
            .position(|l| has_content(l, t))? as u32;

    if left >= right || top >= bot {
        None
    } else {
        Some(imageops::crop_imm(img, left, top, right - left, bot - top))
    }
}

fn has_content<I: Iterator<Item = Rgba<u8>> + Clone>(px: I, t: u32) -> bool {
    let t = t as f64;
    let mut sums = [0; 4];

    let mut len = 0.;

    for p in px.clone() {
        for (s, o) in sums.iter_mut().zip(p.0) {
            *s += o as u32;
        }
        len += 1.;
    }

    let avgs = sums.map(|p| (p as f64) / len);

    for p in px.clone() {
        if avgs
            .iter()
            .zip(p.0)
            .fold(0.0, |s, (a, c)| s + (a - (c as f64)).abs())
            > t
        {
            return true;
        }
    }

    false
}
