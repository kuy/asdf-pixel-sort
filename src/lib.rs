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
