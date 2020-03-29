use std::ops::{Add, AddAssign, Mul};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ColorError {
    InvalidColorError,
    ConversionError(::std::num::ParseIntError),
}

impl From<::std::num::ParseIntError> for ColorError {
    fn from(val: ::std::num::ParseIntError) -> ColorError {
        ColorError::ConversionError(val)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(src: &str) -> Result<Color, ColorError> {
        if !src.starts_with('#') {
            return Err(ColorError::InvalidColorError);
        }
        match src.chars().count() {
            7usize => Ok(Color {
                red: u8::from_str_radix(&src[1..3], 16)? as f32 / 255.0,
                green: u8::from_str_radix(&src[3..5], 16)? as f32 / 255.0,
                blue: u8::from_str_radix(&src[5..], 16)? as f32 / 255.0,
            }),
            _ => Err(ColorError::InvalidColorError),
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self {
        Color {
            red: (self.red + rhs.red).min(1.0),
            green: (self.green + rhs.green).min(1.0),
            blue: (self.blue + rhs.blue).min(1.0),
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self {
        Color {
            red: (self.red * rhs.red).min(1.0),
            green: (self.green * rhs.green).min(1.0),
            blue: (self.blue * rhs.blue).min(1.0),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self {
        assert!(rhs.is_sign_positive());
        Color {
            red: (self.red * rhs).min(1.0),
            green: (self.green * rhs).min(1.0),
            blue: (self.blue * rhs).min(1.0),
        }
    }
}
