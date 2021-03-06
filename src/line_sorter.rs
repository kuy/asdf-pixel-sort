use crate::{color::PColor, finder, Options};

pub(crate) fn sort_column(buf: &mut image::RgbImage, x: u32, options: &Options) {
    let height = buf.height();

    let mut y = 0;
    let mut y_end = 0;

    while y_end < height - 1 {
        y = match finder::get_first_y(buf, x, y, &options.mode) {
            Some(y) => y,
            _ => break,
        };

        y_end = finder::get_next_y(buf, x, y, &options.mode);

        let len = y_end - y;
        let mut line = Vec::with_capacity(len as usize);

        for y in y..y_end {
            let pixel = buf.get_pixel(x, y);
            line.push(*pixel);
        }

        line.sort_by(|a, b| PColor::from(*a).cmp(&(*b).into()));

        for i in 0..len {
            let pixel = line.get(i as usize).unwrap();
            buf.put_pixel(x, y + i, *pixel);
        }

        y = y_end + 1;
    }
}

pub(crate) fn sort_row(buf: &mut image::RgbImage, y: u32, options: &Options) {
    let width = buf.width();

    let mut x = 0;
    let mut x_end = 0;

    while x_end < width - 1 {
        x = match finder::get_first_x(buf, x, y, &options.mode) {
            Some(x) => x,
            _ => break,
        };

        x_end = finder::get_next_x(buf, x, y, &options.mode);

        let len = x_end - x;
        let mut line = Vec::with_capacity(len as usize);

        for x in x..x_end {
            let pixel = buf.get_pixel(x, y);
            line.push(*pixel);
        }

        line.sort_by(|a, b| PColor::from(*a).cmp(&(*b).into()));

        for i in 0..len {
            let pixel = line.get(i as usize).unwrap();
            buf.put_pixel(x + i, y, *pixel);
        }

        x = x_end + 1;
    }
}
