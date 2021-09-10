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
    if options.direction.has_column() {
        for col in 0..buf.width() {
            sort_column(buf, col, options);
        }
    }

    if options.direction.has_row() {
        for row in 0..buf.height() {
            sort_row(buf, row, options);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Direction, Mode};
    use helper::*;

    #[test]
    fn test_sort() {
        assert_sort!("p1", "brightness_default");
    }

    #[test]
    fn test_sort_with_options_black() {
        let options = Options {
            mode: Mode::black(),
            ..Default::default()
        };

        assert_sort_with_options!("p1", "black_default", &options);
    }

    #[test]
    fn test_sort_with_options_white() {
        let options = Options {
            mode: Mode::white(),
            ..Default::default()
        };

        assert_sort_with_options!("p1", "white_default", &options);
    }

    #[test]
    fn test_sort_with_options_column() {
        let options = Options {
            direction: Direction::Column,
            ..Default::default()
        };

        assert_sort_with_options!("p1", "column", &options);
    }

    #[test]
    fn test_sort_with_options_row() {
        let options = Options {
            direction: Direction::Row,
            ..Default::default()
        };

        assert_sort_with_options!("p1", "row", &options);
    }
}
