use image::RgbImage;

use crate::Mode;

mod find_by_black;
mod find_by_brightness;
mod find_by_white;

use find_by_black::*;
use find_by_brightness::*;
use find_by_white::*;

pub(crate) fn get_first_x(buf: &RgbImage, x_start: u32, y: u32, mode: &Mode) -> Option<u32> {
    match mode {
        Mode::Black(black) => get_first_not_black_x(buf, x_start, y, black),
        Mode::Brightness(value) => get_first_bright_x(buf, x_start, y, *value),
        Mode::White(white) => get_first_not_white_x(buf, x_start, y, white),
    }
}

pub(crate) fn get_next_x(buf: &RgbImage, x_start: u32, y: u32, mode: &Mode) -> u32 {
    match mode {
        Mode::Black(black) => get_next_black_x(buf, x_start, y, black),
        Mode::Brightness(value) => get_next_dark_x(buf, x_start, y, *value),
        Mode::White(white) => get_next_white_x(buf, x_start, y, white),
    }
}

pub(crate) fn get_first_y(buf: &RgbImage, x: u32, y_start: u32, mode: &Mode) -> Option<u32> {
    match mode {
        Mode::Black(black) => get_first_not_black_y(buf, x, y_start, black),
        Mode::Brightness(value) => get_first_bright_y(buf, x, y_start, *value),
        Mode::White(white) => get_first_not_white_y(buf, x, y_start, white),
    }
}

pub(crate) fn get_next_y(buf: &RgbImage, x: u32, y_start: u32, mode: &Mode) -> u32 {
    match mode {
        Mode::Black(black) => get_next_black_y(buf, x, y_start, black),
        Mode::Brightness(value) => get_next_dark_y(buf, x, y_start, *value),
        Mode::White(white) => get_next_white_y(buf, x, y_start, white),
    }
}
