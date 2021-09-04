pub(crate) fn to_gray(color: &image::Rgb<u8>) -> u32 {
    (color.0[0] as u32 + color.0[1] as u32 + color.0[2] as u32) / 3
}
