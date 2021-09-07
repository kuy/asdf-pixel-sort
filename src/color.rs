/// Representation of a color with Processing compatible
#[derive(Clone, Eq, PartialEq)]
pub struct PColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Default for PColor {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
}

impl std::fmt::Debug for PColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PColor ({}, {}, {}, a={})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

impl std::cmp::Ord for PColor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_raw().cmp(&other.as_raw())
    }
}

impl std::cmp::PartialOrd for PColor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<image::Rgb<u8>> for PColor {
    fn from(c: image::Rgb<u8>) -> Self {
        PColor::new(c.0[0], c.0[1], c.0[2])
    }
}

impl From<i32> for PColor {
    fn from(c: i32) -> Self {
        PColor::from_raw(c)
    }
}

impl PColor {
    /// Creates a new `PColor` struct with RGB.
    ///
    /// # Example
    ///
    /// ```
    /// # use asdf_pixel_sort::PColor;
    /// let color = PColor::new(32, 32, 32);
    /// ```
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            ..Default::default()
        }
    }

    /// Creates a new `PColor` struct from a signed integer.
    /// This function expects an internal representation of Processing's RGB color.
    ///
    /// # Example
    ///
    /// ```
    /// # use asdf_pixel_sort::PColor;
    /// let color = PColor::new(11, 220, 0);
    /// assert_eq!(color, PColor::from_raw(-16000000));
    /// assert_eq!(color, color.as_raw().into());
    /// ```
    ///
    /// # Inverse operation
    ///
    /// [`PColor::from_raw()`] method is an inverse operation for [`PColor::as_raw()`] function.
    ///
    /// ```
    /// # use asdf_pixel_sort::PColor;
    /// let value = -13000000;
    /// assert_eq!(value, PColor::from_raw(value).as_raw());
    /// ```
    pub fn from_raw(color: i32) -> Self {
        let bytes = color.to_be_bytes();
        Self::new(bytes[1], bytes[2], bytes[3]).with_alpha(bytes[0])
    }

    /// Sets alpha value.
    ///
    /// # Example
    ///
    /// ```
    /// # use asdf_pixel_sort::PColor;
    /// let color = PColor::new(11, 220, 0).with_alpha(128);
    /// ```
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    /// Returns an internal representation of Processing's RGB color as a signed integer.
    ///
    /// Ref. https://processing.org/reference/color_datatype.html
    ///
    /// # Example
    ///
    /// ```
    /// # use asdf_pixel_sort::PColor;
    /// let color = PColor::new(11, 220, 0);
    /// assert_eq!(color.as_raw(), -16000000);
    /// assert_eq!(color, color.as_raw().into());
    /// ```
    ///
    /// # Inverse operation
    ///
    /// [`PColor::as_raw()`] method is an inverse operation for [`PColor::from_raw()`] function.
    ///
    /// ```
    /// # use asdf_pixel_sort::PColor;
    /// let color = PColor::new(11, 220, 0);
    /// assert_eq!(color, color.as_raw().into());
    /// assert_eq!(color, PColor::from_raw(color.as_raw())); // Same above
    /// ```
    pub fn as_raw(&self) -> i32 {
        i32::from_be_bytes([self.alpha, self.red, self.green, self.blue])
    }

    /// Computes a brightness value of Processing between `0` to `255`.
    pub fn brightness(&self) -> u8 {
        self.red.max(self.green).max(self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcolor_default() {
        let expected = PColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        };
        assert_eq!(expected, PColor::default());
    }

    #[test]
    fn test_pcolor_new() {
        let expected = PColor {
            red: 0,
            green: 127,
            blue: 255,
            alpha: 255,
        };
        assert_eq!(expected, PColor::new(0, 127, 255));
    }

    #[test]
    fn test_pcolor_from_raw() {
        let expected = PColor {
            red: 11,
            green: 220,
            blue: 0,
            alpha: 255,
        };
        assert_eq!(expected, PColor::from_raw(-16000000));

        let expected = PColor {
            red: 57,
            green: 162,
            blue: 192,
            alpha: 255,
        };
        assert_eq!(expected, PColor::from_raw(-13000000));
    }

    #[test]
    fn test_pcolor_with_alpha() {
        let expected = PColor {
            red: 128,
            green: 32,
            blue: 64,
            alpha: 96,
        };
        assert_eq!(expected, PColor::new(128, 32, 64).with_alpha(96));
    }

    #[test]
    fn test_pcolor_val() {
        let expected = -16000000;
        assert_eq!(expected, PColor::new(11, 220, 0).as_raw());

        let expected = -13000000;
        assert_eq!(expected, PColor::new(57, 162, 192).as_raw());
    }

    #[test]
    fn test_pcolor_brightness() {
        let expected = 220;
        assert_eq!(expected, PColor::new(11, 220, 0).brightness());

        let expected = 0;
        assert_eq!(expected, PColor::new(0, 0, 0).brightness());
    }

    #[test]
    fn test_pcolor_cmp() {
        let c1 = PColor::new(11, 220, 0);
        let c2 = PColor::new(57, 162, 192);
        assert_eq!(std::cmp::Ordering::Less, c1.cmp(&c2));
        assert!(c1 < c2);
        assert_eq!(std::cmp::Ordering::Greater, c2.cmp(&c1));
        assert!(c2 > c1);
    }

    #[test]
    fn test_pcolor_eq() {
        assert!(PColor::default().eq(&PColor::default()));
        assert!(PColor::default() == PColor::default());

        let c1 = PColor::new(57, 162, 192);
        let c2 = PColor::new(57, 162, 192);
        assert!(c1.eq(&c2));
        assert!(c2.eq(&c1));
        assert!(c1 == c2);
        assert!(c2 == c1);
    }
}
