use crate::{color::to_gray, finder};

pub(crate) fn sort_column(buf: &mut image::RgbImage, x: u32, black: u32) {
    let height = buf.height();

    let mut y = 0;
    let mut y_end = 0;

    while y_end < height - 1 {
        y = match finder::get_first_not_black_y(buf, x, y, black) {
            Some(y) => y,
            _ => break,
        };

        y_end = finder::get_next_black_y(buf, x, y, black);

        let len = y_end - y;
        let mut line = Vec::with_capacity(len as usize);

        for y in y..y_end {
            let pixel = buf.get_pixel(x, y);
            line.push(*pixel);
        }

        line.sort_by(|a, b| to_gray(a).cmp(&to_gray(b)));

        for i in 0..len {
            let pixel = line.get(i as usize).unwrap();
            buf.put_pixel(x, y + i, *pixel);
        }

        y = y_end + 1;
    }
}

pub(crate) fn sort_row(buf: &mut image::RgbImage, y: u32, black: u32) {
    let width = buf.width();

    let mut x = 0;
    let mut x_end = 0;

    while x_end < width - 1 {
        x = match finder::get_first_not_black_x(buf, x, y, black) {
            Some(x) => x,
            _ => break,
        };

        x_end = finder::get_next_black_x(buf, x, y, black);

        let len = x_end - x;
        let mut line = Vec::with_capacity(len as usize);

        for x in x..x_end {
            let pixel = buf.get_pixel(x, y);
            line.push(*pixel);
        }

        line.sort_by(|a, b| to_gray(a).cmp(&to_gray(b)));

        for i in 0..len {
            let pixel = line.get(i as usize).unwrap();
            buf.put_pixel(x + i, y, *pixel);
        }

        x = x_end + 1;
    }
}
