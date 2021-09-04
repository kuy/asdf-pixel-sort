use crate::color::to_gray;
use image::RgbImage;

pub(crate) fn get_first_not_black_x(
    buf: &RgbImage,
    x_start: u32,
    y: u32,
    black: u32,
) -> Option<u32> {
    let mut x = x_start;

    while x < buf.width() {
        let pixel = buf.get_pixel(x, y);
        if black <= to_gray(pixel) {
            break; // found non-black pixel
        }

        x += 1;

        if buf.width() <= x {
            return None;
        }
    }

    Some(x)
}

pub(crate) fn get_next_black_x(buf: &RgbImage, x_start: u32, y: u32, black: u32) -> u32 {
    let width = buf.width();
    let mut x = x_start + 1;

    while x < width {
        let pixel = buf.get_pixel(x, y);
        if to_gray(pixel) <= black {
            break; // found black pixel
        }

        x += 1;

        if width <= x {
            return width - 1;
        }
    }

    x - 1
}

pub(crate) fn get_first_not_black_y(
    buf: &RgbImage,
    x: u32,
    y_start: u32,
    black: u32,
) -> Option<u32> {
    let mut y = y_start;

    if y < buf.height() {
        loop {
            let pixel = buf.get_pixel(x, y);
            if black <= to_gray(pixel) {
                break; // found non-black pixel
            }

            y += 1;

            if buf.height() <= y {
                return None;
            }
        }
    }

    Some(y)
}

pub(crate) fn get_next_black_y(buf: &RgbImage, x: u32, y_start: u32, black: u32) -> u32 {
    let height = buf.height();
    let mut y = y_start + 1;

    if y < height {
        loop {
            let pixel = buf.get_pixel(x, y);
            if to_gray(pixel) <= black {
                break; // found black pixel
            }

            y += 1;

            if height <= y {
                return height - 1;
            }
        }
    }

    y - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    #[test]
    fn test_get_first_not_black_x() {
        let data = [
            0, 0, 0, //
            8, 8, 8, //
            16, 16, 16, //
            64, 64, 64, //
            8, 8, 8, //
            4, 4, 4, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(6, 1, Vec::from(&data[..])).unwrap();

        let actual = get_first_not_black_x(&buf, 0, 0, 16);
        assert_eq!(actual, Some(2));

        let actual = get_first_not_black_x(&buf, 3, 0, 16);
        assert_eq!(actual, Some(3), "same with initial position");

        let actual = get_first_not_black_x(&buf, 4, 0, 16);
        assert_eq!(actual, None, "not found");

        // NOTE: This spec is wiered, but same with original
        let actual = get_first_not_black_x(&buf, 6, 0, 16);
        assert_eq!(actual, Some(6), "out of bounds");
    }

    #[test]
    fn test_get_next_black_x() {
        let data = [
            255, 255, 255, //
            196, 196, 196, //
            16, 16, 16, //
            64, 64, 64, //
            8, 8, 8, //
            196, 196, 196, //
            196, 196, 196, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(7, 1, Vec::from(&data[..])).unwrap();

        let actual = get_next_black_x(&buf, 0, 0, 16);
        assert_eq!(actual, 1);

        let actual = get_next_black_x(&buf, 2, 0, 16);
        assert_eq!(actual, 3, "same with initial position");

        let actual = get_next_black_x(&buf, 5, 0, 16);
        assert_eq!(actual, 6, "not found");

        let actual = get_next_black_x(&buf, 6, 0, 16);
        assert_eq!(actual, 6, "out of bounds");
    }

    #[test]
    fn test_get_first_not_black_y() {
        let data = [
            0, 0, 0, //
            8, 8, 8, //
            16, 16, 16, //
            64, 64, 64, //
            8, 8, 8, //
            4, 4, 4, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(1, 6, Vec::from(&data[..])).unwrap();

        let actual = get_first_not_black_y(&buf, 0, 0, 16);
        assert_eq!(actual, Some(2));

        let actual = get_first_not_black_y(&buf, 0, 3, 16);
        assert_eq!(actual, Some(3), "same with initial position");

        let actual = get_first_not_black_y(&buf, 0, 4, 16);
        assert_eq!(actual, None, "not found");

        // NOTE: This spec is wiered, but same with original
        let actual = get_first_not_black_y(&buf, 0, 6, 16);
        assert_eq!(actual, Some(6), "out of bounds");
    }

    #[test]
    fn test_get_next_black_y() {
        let data = [
            255, 255, 255, //
            196, 196, 196, //
            16, 16, 16, //
            64, 64, 64, //
            8, 8, 8, //
            196, 196, 196, //
            196, 196, 196, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(1, 7, Vec::from(&data[..])).unwrap();

        let actual = get_next_black_y(&buf, 0, 0, 16);
        assert_eq!(actual, 1);

        let actual = get_next_black_y(&buf, 0, 2, 16);
        assert_eq!(actual, 3, "same with initial position");

        let actual = get_next_black_y(&buf, 0, 5, 16);
        assert_eq!(actual, 6, "not found");

        let actual = get_next_black_y(&buf, 0, 6, 16);
        assert_eq!(actual, 6, "out of bounds");
    }
}
