use core::fmt::Debug;

use cast::f32;
use cast::f64;
use num_traits::{Float, Zero};
pub trait RoundToSigDig {
    /// Round `float_number` to specified number of significant figures.
    fn round_to_sf(&self, significant_digits: u8) -> Self;
}

impl RoundToSigDig for f64 {
    /// Round `float_number` to specified number of significant figures.
    fn round_to_sf(&self, significant_digits: u8) -> Self
    where
        Self: Float + Debug,
    {
        if self.is_zero() || significant_digits.is_zero() {
            return 0.0;
        }

        let whole_digits = f64(self.round().to_string().len());
        let digit_shift = f64(significant_digits) - whole_digits;
        let shifted_num = self * 10.0.powf(digit_shift);
        let shifted_rounded = shifted_num.round();
        let rounded = shifted_rounded / 10.0.powf(digit_shift);

        rounded
    }
}

impl RoundToSigDig for f32 {
    /// Round `float_number` to specified number of significant figures.
    fn round_to_sf(&self, significant_digits: u8) -> Self
    where
        Self: Float + Debug,
    {
        if self.is_zero() || significant_digits.is_zero() {
            return 0.0;
        }

        let whole_digits = f32(self.round().to_string().len());
        let digit_shift = f32(significant_digits) - whole_digits;
        let shifted_num = self * 10.0.powf(digit_shift);
        let shifted_rounded = shifted_num.round();
        let rounded = shifted_rounded / 10.0.powf(digit_shift);

        rounded
    }
}

#[cfg(test)]
mod tests {
    use super::RoundToSigDig;

    #[test]
    #[allow(clippy::float_cmp)]
    fn round_to_sf_f64() {
        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(9);
        assert_eq!(after, 123.123_457_f64);

        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(5);
        assert_eq!(after, 123.12_f64);

        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(1);
        assert_eq!(after, 100.0);

        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(2);
        assert_eq!(after, 120.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn round_to_sf_f32() {
        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(9);
        assert_eq!(after, 123.123_457_f64);

        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(5);
        assert_eq!(after, 123.12_f64);

        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(1);
        assert_eq!(after, 100.0);

        let before = 123.123_456_789_f64;
        let after = before.round_to_sf(2);
        assert_eq!(after, 120.0);
    }
}
