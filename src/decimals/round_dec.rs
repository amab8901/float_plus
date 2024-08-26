use core::fmt::Debug;

use num_traits::Float;
pub trait RoundToFraction {
    /// Round `float_number` to specified number of digits in the fraction.
    #[allow(clippy::return_self_not_must_use)]
    fn round_to_fraction(&self, digits: u32) -> Self
    where
        Self: Float + Debug,
    {
        let rounded_float = if digits == 0 {
            let rounded_float = self.trunc();
            rounded_float
        } else {
            let ten = Self::from(10.0).unwrap();
            let digits = Self::from(digits).unwrap();
            let round_factor = ten.powf(digits);
            let rounded_float = (self.mul(round_factor)).round() / round_factor;
            rounded_float
        };

        rounded_float
    }
}

impl<F> RoundToFraction for F where F: Float {}

#[cfg(test)]
mod tests {
    use super::RoundToFraction;

    #[test]
    #[allow(clippy::float_cmp)]
    fn five_digits() {
        let before = 100.123_456_789_f64;
        let after = before.round_to_fraction(5);
        assert_eq!(after, 100.123_46);
    }
}
