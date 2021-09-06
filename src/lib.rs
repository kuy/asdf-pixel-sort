use image::RgbImage;

mod color;
mod finder;
mod options;
mod sorter;

pub use color::PColor;
pub use options::{Mode, Options};

/// Sorts pixels in the given image with default options
pub fn sort(buf: &mut RgbImage) {
    sort_with_options(buf, &Options::default());
}

/// Sorts pixels in the given image with options
pub fn sort_with_options(buf: &mut RgbImage, options: &Options) {
    for col in 0..buf.width() {
        sorter::sort_column(buf, col, options);
    }

    for row in 0..buf.height() {
        sorter::sort_row(buf, row, options);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_with_options() {
        let mut actual = image::open("tests/p1.bmp").unwrap().to_rgb8();
        let options = Options {
            mode: Mode::black(),
        };
        sort_with_options(&mut actual, &options);
        let expected = image::open("tests/p1.expected.bmp").unwrap().to_rgb8();
        assert!(expected.as_raw() == actual.as_raw());
    }
}
