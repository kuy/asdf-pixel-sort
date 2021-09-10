#[macro_export]
macro_rules! assert_sort {
    ($base:literal, $expect:literal) => {
        let base_path = format!("tests/{}.bmp", $base);
        let mut actual = image::open(&base_path).expect(&base_path).to_rgb8();
        sort(&mut actual);
        let expected_path = format!("tests/{}-{}.bmp", $base, $expect);
        let expected = image::open(&expected_path).expect(&expected_path).to_rgb8();
        assert!(expected.as_raw() == actual.as_raw());
    };
}

#[macro_export]
macro_rules! assert_sort_with_options {
    ($base:literal, $expect:literal, $arg:expr) => {
        let base_path = format!("tests/{}.bmp", $base);
        let mut actual = image::open(&base_path).expect(&base_path).to_rgb8();
        sort_with_options(&mut actual, $arg);
        let expected_path = format!("tests/{}-{}.bmp", $base, $expect);
        let expected = image::open(&expected_path).expect(&expected_path).to_rgb8();
        assert!(expected.as_raw() == actual.as_raw());
    };
}
