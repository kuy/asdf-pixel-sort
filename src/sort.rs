use image::RgbImage;

use crate::{
    line_sorter::{sort_column, sort_row},
    Options,
};

/// Sorts pixels in the given image with default options
pub fn sort(buf: &mut RgbImage) {
    sort_with_options(buf, &Options::default());
}

/// Sorts pixels in the given image with options
pub fn sort_with_options(buf: &mut RgbImage, options: &Options) {
    for col in 0..buf.width() {
        sort_column(buf, col, options);
    }

    for row in 0..buf.height() {
        sort_row(buf, row, options);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;

    #[test]
    fn test_sort() {
        let mut actual = image::open("tests/p1.bmp").unwrap().to_rgb8();
        sort(&mut actual);
        let expected = image::open("tests/p1-brightness_default.bmp")
            .unwrap()
            .to_rgb8();
        assert!(expected.as_raw() == actual.as_raw());
    }

    #[test]
    fn test_sort_with_options() {
        let mut actual = image::open("tests/p1.bmp").unwrap().to_rgb8();
        let options = Options {
            mode: Mode::black(),
        };
        sort_with_options(&mut actual, &options);
        let expected = image::open("tests/p1-black_default.bmp").unwrap().to_rgb8();
        assert!(expected.as_raw() == actual.as_raw());
    }
}
