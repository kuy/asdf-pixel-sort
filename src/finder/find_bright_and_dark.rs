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

#[cfg(test)]
mod tests {
    use super::*;

    use image::ImageBuffer;
    use once_cell::sync::Lazy;

    static BRIGHTNESS: u8 = 60;

    #[test]
    fn test_get_first_bright_x() {
        let data = [
            0, 0, 0, //
            8, 8, 8, //
            16, 16, 16, //
            64, 64, 64, //
            8, 8, 8, //
            4, 4, 4, //
        ];
        let buf: RgbImage = ImageBuffer::from_raw(6, 1, Vec::from(&data[..])).unwrap();

        let actual = get_first_bright_x(&buf, 0, 0, BRIGHTNESS);
        assert_eq!(actual, Some(2));

        let actual = get_first_bright_x(&buf, 3, 0, BRIGHTNESS);
        assert_eq!(actual, Some(3), "same with initial position");

        let actual = get_first_bright_x(&buf, 4, 0, BRIGHTNESS);
        assert_eq!(actual, None, "not found");

        // NOTE: This spec is wiered, but same with original
        let actual = get_first_bright_x(&buf, 6, 0, BRIGHTNESS);
        assert_eq!(actual, Some(6), "out of bounds");
    }
}
