use once_cell::sync::Lazy;

use crate::PColor;

/// Options to configure behaviours.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Options {
    /// Sorting mode.
    pub mode: Mode,

    /// Sorting direction.
    pub direction: Direction,
}

/// Default value of [`Mode::Black`].
pub static DEFAULT_BLACK: Lazy<PColor> = Lazy::new(|| PColor::new(11, 220, 0));

/// Default value of [`Mode::Brightness`].
pub static DEFAULT_BRIGHTNESS: u8 = 60;

/// Default value of [`Mode::White`].
pub static DEFAULT_WHITE: Lazy<PColor> = Lazy::new(|| PColor::new(57, 162, 192));

/// Sorting modes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Black mode with a threshold color.
    Black(PColor),

    /// Brightness mode with a threshold value `0` to `255`.
    Brightness(u8),

    /// White mode with a threshold color.
    White(PColor),
}

impl Default for Mode {
    fn default() -> Self {
        Self::brightness()
    }
}

impl Mode {
    /// Black mode with a default threshold.
    pub fn black() -> Self {
        Self::Black(DEFAULT_BLACK.clone())
    }

    /// Brightness mode with a default threshold.
    pub fn brightness() -> Self {
        Self::Brightness(DEFAULT_BRIGHTNESS)
    }

    /// White mode with a default threshold.
    pub fn white() -> Self {
        Self::White(DEFAULT_WHITE.clone())
    }
}

/// Sorting direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Both column and row.
    Both,

    /// Only column.
    Column,

    /// Only row.
    Row,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Both
    }
}

impl Direction {
    /// Checks if this direction has [`Direction::Column`].
    pub fn has_column(&self) -> bool {
        match self {
            Self::Both | Self::Column => true,
            _ => false,
        }
    }

    /// Checks if this direction has [`Direction::Row`].
    pub fn has_row(&self) -> bool {
        match self {
            Self::Both | Self::Row => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_default() {
        let expected = Options {
            mode: Mode::Brightness(60),
            direction: Direction::Both,
        };
        assert_eq!(expected, Options::default());
    }

    #[test]
    fn test_mode_default() {
        let expected = Mode::Brightness(60);
        assert_eq!(expected, Mode::default());
    }

    #[test]
    fn test_mode_black() {
        let color = PColor::new(11, 220, 0);
        let expected = Mode::Black(color);
        assert_eq!(expected, Mode::black());
    }

    #[test]
    fn test_mode_brightness() {
        let value = 60;
        let expected = Mode::Brightness(value);
        assert_eq!(expected, Mode::brightness());
    }

    #[test]
    fn test_mode_white() {
        let color = PColor::new(57, 162, 192);
        let expected = Mode::White(color);
        assert_eq!(expected, Mode::white());
    }

    #[test]
    fn test_direction_default() {
        let expected = Direction::Both;
        assert_eq!(expected, Direction::default());
    }

    #[test]
    fn test_direction_has_column() {
        assert!(Direction::Both.has_column());
        assert!(Direction::Column.has_column());
        assert!(!Direction::Row.has_column());
    }

    #[test]
    fn test_direction_has_row() {
        assert!(Direction::Both.has_row());
        assert!(!Direction::Column.has_row());
        assert!(Direction::Row.has_row());
    }
}
