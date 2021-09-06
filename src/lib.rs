pub use color::PColor;
use image::RgbImage;

mod color;
mod finder;
mod sorter;

/// Sorts pixels in the given image
pub fn sort(buf: &mut RgbImage, black: &PColor) {
    for col in 0..buf.width() {
        sorter::sort_column(buf, col, black);
    }

    for row in 0..buf.height() {
        sorter::sort_row(buf, row, black);
    }
}
