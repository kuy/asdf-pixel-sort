#[macro_export]
macro_rules! assert_sort {
    ($base:literal, $expect:literal) => {
        let base_path = format!("tests/{}.bmp", $base);
        let mut actual = image::open(&base_path)
            .expect(format!("Not found (base): '{}'", base_path).as_str())
            .to_rgb8();
        sort(&mut actual);
        let expected_path = format!("tests/{}-{}.bmp", $base, $expect);
        let expected = image::open(&expected_path)
            .expect(format!("Not found (expected): '{}'", expected_path).as_str())
            .to_rgb8();
        assert!(expected.as_raw() == actual.as_raw(), "Image not matched");
    };
}

#[macro_export]
macro_rules! assert_sort_with_options {
    ($base:literal, $expect:literal, $arg:expr) => {
        let base_path = format!("tests/{}.bmp", $base);
        let mut actual = image::open(&base_path)
            .expect(format!("Not found (base): '{}'", base_path).as_str())
            .to_rgb8();
        sort_with_options(&mut actual, $arg);
        let expected_path = format!("tests/{}-{}.bmp", $base, $expect);
        if std::env::var("OVERWRITE").unwrap_or_default().as_str() == "true" {
            actual
                .save(&expected_path)
                .expect(format!("Failed to overwrite: '{}'", expected_path).as_str());
        } else {
            let expected = image::open(&expected_path)
                .expect(format!("Not found (expected): '{}'", expected_path).as_str())
                .to_rgb8();
            assert!(expected.as_raw() == actual.as_raw(), "Image not matched");
        }
    };
}
