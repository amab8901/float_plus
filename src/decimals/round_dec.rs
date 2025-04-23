use core::fmt::Debug;

use num_traits::Float;
pub trait RoundToFraction {
    /// Round `float_number` to specified number of digits in the fraction. Rounds to nearest integer. Halfway cases are rounded down.
    fn round_to_fraction_halfway_up(&self, digits: u32) -> Self
    where
        Self: Float + Debug,
    {
        if &self.round() == self {
            return *self;
        }

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

    /// Round `float_number` to specified number of digits in the fraction. Rounds to nearest integer. Halfway cases are rounded up.
    fn round_to_fraction_halfway_down(&self, digits: u32) -> Self
    where
        Self: Float + Debug,
    {
        if &self.round() == self {
            return *self;
        }

        let rounded_float = if digits == 0 {
            let rounded_float = self.trunc();
            rounded_float
        } else {
            let ten = Self::from(10.0).unwrap();
            let digits = Self::from(digits).unwrap();
            let round_factor = ten.powf(digits);
            let multiplied = self.mul(round_factor);
            let floor = multiplied.floor();
            let last_decimal = multiplied - floor;
            let half = Self::from(0.5).unwrap();

            let rounded_elevation = if half < last_decimal {
                multiplied.round()
            } else {
                multiplied.floor()
            };

            let rounded_float = rounded_elevation / round_factor;
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
        let after = before.round_to_fraction_halfway_up(5);
        assert_eq!(after, 100.123_46);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn halfway_up_in_halfway_case() {
        let before = 100.1555_f64;
        let after = before.round_to_fraction_halfway_up(3);
        assert_eq!(after, 100.156);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn halfway_up_dont_round_whole_number() {
        let before = 3_465_572_798_000_935_000.0_f64;
        let after: f64 = before.round_to_fraction_halfway_up(8);
        assert_eq!(after, 3_465_572_798_000_935_000.0_f64);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn halfway_down_in_halfway_case() {
        let before = 100.412_215_f64;
        let after = before.round_to_fraction_halfway_down(5);
        assert_eq!(after, 100.41221);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn halfway_down_in_down_case() {
        let before = 100.994_f64;
        let after = before.round_to_fraction_halfway_down(2);
        assert_eq!(after, 100.99);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn halfway_down_in_up_case() {
        let before = 100.996_f64;
        let after = before.round_to_fraction_halfway_down(2);
        assert_eq!(after, 101.00);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn halfway_down_dont_round_whole_number() {
        let before = 3_465_572_798_000_935_000.0_f64;
        let after: f64 = before.round_to_fraction_halfway_down(8);
        assert_eq!(after, 3_465_572_798_000_935_000.0_f64);
    }
}
