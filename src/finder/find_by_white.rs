use image::RgbImage;

use crate::PColor;

pub(crate) fn get_first_not_white_x(
    buf: &RgbImage,
    x_start: u32,
    y: u32,
    white: &PColor,
) -> Option<u32> {
    let width = buf.width();
    let mut x = x_start;

    while x < width {
        let pixel = buf.get_pixel(x, y);
        if *white >= (*pixel).into() {
            break; // found non-white pixel
        }

        x += 1;

        if width <= x {
            return None;
        }
    }

    Some(x)
}

pub(crate) fn get_next_white_x(buf: &RgbImage, x_start: u32, y: u32, white: &PColor) -> u32 {
    let width = buf.width();
    let mut x = x_start + 1;

    while x < width {
        let pixel = buf.get_pixel(x, y);
        if *white <= (*pixel).into() {
            break; // found white pixel
        }

        x += 1;

        if width <= x {
            return width - 1;
        }
    }

    x - 1
}

pub(crate) fn get_first_not_white_y(
    buf: &RgbImage,
    x: u32,
    y_start: u32,
    white: &PColor,
) -> Option<u32> {
    let mut y = y_start;

    if y < buf.height() {
        loop {
            let pixel = buf.get_pixel(x, y);
            if *white >= (*pixel).into() {
                break; // found non-white pixel
            }

            y += 1;

            if buf.height() <= y {
                return None;
            }
        }
    }

    Some(y)
}

pub(crate) fn get_next_white_y(buf: &RgbImage, x: u32, y_start: u32, white: &PColor) -> u32 {
    let height = buf.height();
    let mut y = y_start + 1;

    if y < height {
        loop {
            let pixel = buf.get_pixel(x, y);
            if *white <= (*pixel).into() {
                break; // found white pixel
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
    use once_cell::sync::Lazy;

    static WHITE: Lazy<PColor> = Lazy::new(|| PColor::new(192, 192, 192));

    #[test]
    fn test_get_first_not_white_x() {
        let data = [
            255, 255, 255, //
            193, 193, 193, //
            192, 192, 192, //
            191, 191, 191, //
            248, 248, 248, //
            255, 255, 255, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(6, 1, Vec::from(&data[..])).unwrap();

        let actual = get_first_not_white_x(&buf, 0, 0, &WHITE);
        assert_eq!(Some(2), actual);

        let actual = get_first_not_white_x(&buf, 3, 0, &WHITE);
        assert_eq!(Some(3), actual, "same with start position");

        let actual = get_first_not_white_x(&buf, 4, 0, &WHITE);
        assert_eq!(None, actual, "not found");

        // NOTE: This spec is wiered, but same with original
        let actual = get_first_not_white_x(&buf, 6, 0, &WHITE);
        assert_eq!(Some(6), actual, "out of bounds");
    }

    #[test]
    fn test_get_next_white_x() {
        let data = [
            0, 0, 0, //
            191, 191, 191, //
            192, 192, 192, //
            8, 8, 8, //
            255, 255, 255, //
            4, 4, 4, //
            4, 4, 4, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(7, 1, Vec::from(&data[..])).unwrap();

        let actual = get_next_white_x(&buf, 0, 0, &WHITE);
        assert_eq!(1, actual);

        let actual = get_next_white_x(&buf, 2, 0, &WHITE);
        assert_eq!(3, actual, "skip start position even if it's white");

        let actual = get_next_white_x(&buf, 5, 0, &WHITE);
        assert_eq!(6, actual, "not found");

        let actual = get_next_white_x(&buf, 6, 0, &WHITE);
        assert_eq!(6, actual, "out of bounds");
    }

    #[test]
    fn test_get_first_not_white_y() {
        let data = [
            255, 255, 255, //
            193, 193, 193, //
            192, 192, 192, //
            191, 191, 191, //
            248, 248, 248, //
            255, 255, 255, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(1, 6, Vec::from(&data[..])).unwrap();

        let actual = get_first_not_white_y(&buf, 0, 0, &WHITE);
        assert_eq!(Some(2), actual);

        let actual = get_first_not_white_y(&buf, 0, 3, &WHITE);
        assert_eq!(Some(3), actual, "same with start position");

        let actual = get_first_not_white_y(&buf, 0, 4, &WHITE);
        assert_eq!(None, actual, "not found");

        // NOTE: This spec is wiered, but same with original
        let actual = get_first_not_white_y(&buf, 0, 6, &WHITE);
        assert_eq!(Some(6), actual, "out of bounds");
    }

    #[test]
    fn test_get_next_white_y() {
        let data = [
            0, 0, 0, //
            191, 191, 191, //
            192, 192, 192, //
            8, 8, 8, //
            255, 255, 255, //
            4, 4, 4, //
            4, 4, 4, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(1, 7, Vec::from(&data[..])).unwrap();

        let actual = get_next_white_y(&buf, 0, 0, &WHITE);
        assert_eq!(1, actual);

        let actual = get_next_white_y(&buf, 0, 2, &WHITE);
        assert_eq!(3, actual, "skip start position even if it's white");

        let actual = get_next_white_y(&buf, 0, 5, &WHITE);
        assert_eq!(6, actual, "not found");

        let actual = get_next_white_y(&buf, 0, 6, &WHITE);
        assert_eq!(6, actual, "out of bounds");
    }
}
