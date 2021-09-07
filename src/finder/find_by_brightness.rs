use image::RgbImage;

use crate::PColor;

pub(crate) fn get_first_bright_x(
    buf: &RgbImage,
    x_start: u32,
    y: u32,
    brightness: u8,
) -> Option<u32> {
    let width = buf.width();
    let mut x = x_start;

    while x < width {
        let pixel = buf.get_pixel(x, y);
        if brightness <= PColor::from(*pixel).brightness() {
            break; // found bright pixel
        }

        x += 1;

        if width <= x {
            return None;
        }
    }

    Some(x)
}

pub(crate) fn get_next_dark_x(buf: &RgbImage, x_start: u32, y: u32, brightness: u8) -> u32 {
    let width = buf.width();
    let mut x = x_start + 1;

    while x < width {
        let pixel = buf.get_pixel(x, y);
        if PColor::from(*pixel).brightness() <= brightness {
            break; // found dark pixel
        }

        x += 1;

        if width <= x {
            return width - 1;
        }
    }

    x - 1
}

pub(crate) fn get_first_bright_y(
    buf: &RgbImage,
    x: u32,
    y_start: u32,
    brightness: u8,
) -> Option<u32> {
    let height = buf.height();
    let mut y = y_start;

    if y < height {
        loop {
            let pixel = buf.get_pixel(x, y);
            if brightness <= PColor::from(*pixel).brightness() {
                break; // found bright pixel
            }

            y += 1;

            if height <= y {
                return None;
            }
        }
    }

    Some(y)
}

pub(crate) fn get_next_dark_y(buf: &RgbImage, x: u32, y_start: u32, brightness: u8) -> u32 {
    let height = buf.height();
    let mut y = y_start + 1;

    if y < height {
        loop {
            let pixel = buf.get_pixel(x, y);
            if PColor::from(*pixel).brightness() <= brightness {
                break; // found dark pixel
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

    static BRIGHTNESS: u8 = 60;

    #[test]
    fn test_get_first_bright_x() {
        let data = [
            0, 0, 0, //
            16, 59, 6, //
            0, 60, 40, //
            24, 24, 24, //
            8, 8, 255, //
            4, 4, 4, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(6, 1, Vec::from(&data[..])).unwrap();

        let actual = get_first_bright_x(&buf, 0, 0, BRIGHTNESS);
        assert_eq!(actual, Some(2));

        let actual = get_first_bright_x(&buf, 2, 0, BRIGHTNESS);
        assert_eq!(actual, Some(2), "same with start position");

        let actual = get_first_bright_x(&buf, 3, 0, BRIGHTNESS);
        assert_eq!(actual, Some(4), "non-zero start");

        let actual = get_first_bright_x(&buf, 5, 0, BRIGHTNESS);
        assert_eq!(actual, None, "not found");

        // NOTE: This spec is wiered, but same with original
        let actual = get_first_bright_x(&buf, 6, 0, BRIGHTNESS);
        assert_eq!(actual, Some(6), "out of bounds");
    }

    #[test]
    fn test_get_next_dark_x() {
        let data = [
            255, 255, 255, //
            32, 196, 0, //
            0, 60, 0, //
            0, 0, 0, //
            8, 255, 8, //
            196, 196, 196, //
            196, 196, 196, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(7, 1, Vec::from(&data[..])).unwrap();

        let actual = get_next_dark_x(&buf, 0, 0, BRIGHTNESS);
        assert_eq!(actual, 1);

        let actual = get_next_dark_x(&buf, 2, 0, BRIGHTNESS);
        assert_eq!(actual, 2, "skip start position even if it's dark");

        let actual = get_next_dark_x(&buf, 3, 0, BRIGHTNESS);
        assert_eq!(actual, 6, "not found");

        let actual = get_next_dark_x(&buf, 6, 0, BRIGHTNESS);
        assert_eq!(actual, 6, "out of bounds");
    }

    #[test]
    fn test_get_first_bright_y() {
        let data = [
            0, 0, 4, //
            0, 8, 12, //
            0, 60, 0, //
            16, 16, 16, //
            8, 8, 8, //
            4, 4, 4, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(1, 6, Vec::from(&data[..])).unwrap();

        let actual = get_first_bright_y(&buf, 0, 0, BRIGHTNESS);
        assert_eq!(actual, Some(2));

        let actual = get_first_bright_y(&buf, 0, 2, BRIGHTNESS);
        assert_eq!(actual, Some(2), "same with initial position");

        let actual = get_first_bright_y(&buf, 0, 3, BRIGHTNESS);
        assert_eq!(actual, None, "not found");

        // NOTE: This spec is wiered, but same with original
        let actual = get_first_bright_y(&buf, 0, 6, BRIGHTNESS);
        assert_eq!(actual, Some(6), "out of bounds");
    }

    #[test]
    fn test_get_next_dark_y() {
        let data = [
            255, 0, 255, //
            0, 0, 196, //
            0, 60, 0, //
            60, 0, 0, //
            8, 8, 8, //
            196, 196, 196, //
            196, 196, 196, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(1, 7, Vec::from(&data[..])).unwrap();

        let actual = get_next_dark_y(&buf, 0, 0, BRIGHTNESS);
        assert_eq!(actual, 1);

        let actual = get_next_dark_y(&buf, 0, 2, BRIGHTNESS);
        assert_eq!(actual, 2, "same with initial position");

        let actual = get_next_dark_y(&buf, 0, 5, BRIGHTNESS);
        assert_eq!(actual, 6, "not found");

        let actual = get_next_dark_y(&buf, 0, 6, BRIGHTNESS);
        assert_eq!(actual, 6, "out of bounds");
    }
}
