/// Representation of a color with Processing compatible
#[derive(Clone, Eq, PartialEq, PartialOrd)]
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
        self.val().cmp(&other.val())
    }
}

impl From<image::Rgb<u8>> for PColor {
    fn from(c: image::Rgb<u8>) -> Self {
        PColor::new(c.0[0], c.0[1], c.0[2])
    }
}

impl PColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            ..Default::default()
        }
    }

    /// Sets alpha value.
    ///
    /// # Example
    ///
    /// ```
    /// let color = PColor::new(11, 220, 0).with_alpha(128);
    /// ```
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    /// Returns `int` representation of `color` type in Processing.
    pub fn val(&self) -> i32 {
        i32::from_be_bytes([self.alpha, self.red, self.green, self.blue])
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
        assert_eq!(expected, PColor::new(11, 220, 0).val());

        let expected = -13000000;
        assert_eq!(expected, PColor::new(57, 162, 192).val());
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
