use cast::f32;
use cast::f64;
use num_traits::abs;

pub trait ApproxEq {
    fn aeq(&self, other: Self, decimals: u8) -> bool;
}

impl ApproxEq for f64 {
    fn aeq(&self, other: Self, decimals: u8) -> bool {
        let decimals = f64(decimals);
        let max_diff = 10.0_f64.powf(-decimals);

        let aeq = abs(self - other) < max_diff;
        aeq
    }
}

impl ApproxEq for Option<f64> {
    fn aeq(&self, other: Self, decimals: u8) -> bool {
        if self.is_none() && other.is_none() {
            return true;
        }

        if self.is_some() && other.is_some() {
            let Some(first) = self else {
                return false;
            };
            let Some(second) = other else {
                return false;
            };

            let decimals = f64(decimals);
            let max_diff = 10.0_f64.powf(-decimals);

            let aeq = abs(first - second) < max_diff;

            return aeq;
        }

        false
    }
}

impl<E> ApproxEq for Result<f64, E> {
    fn aeq(&self, other: Self, decimals: u8) -> bool {
        if self.is_err() && other.is_err() {
            return true;
        }

        if self.is_ok() && other.is_ok() {
            let Ok(first) = self else {
                return false;
            };
            let Ok(second) = other else {
                return false;
            };

            let decimals = f64(decimals);
            let max_diff = 10.0_f64.powf(-decimals);

            let aeq = abs(first - second) < max_diff;

            return aeq;
        }

        false
    }
}

impl ApproxEq for f32 {
    fn aeq(&self, other: Self, decimals: u8) -> bool {
        let decimals = f32(decimals);
        let max_diff = 10.0_f32.powf(-decimals);

        let aeq = abs(self - other) < max_diff;

        aeq
    }
}

impl ApproxEq for Option<f32> {
    fn aeq(&self, other: Self, decimals: u8) -> bool {
        if self.is_none() && other.is_none() {
            return true;
        }

        if self.is_some() && other.is_some() {
            let first = self.unwrap();
            let second = other.unwrap();

            let decimals = f32(decimals);
            let max_diff = 10.0_f32.powf(-decimals);

            let aeq = abs(first - second) < max_diff;

            return aeq;
        }

        false
    }
}

impl<E> ApproxEq for Result<f32, E> {
    fn aeq(&self, other: Self, decimals: u8) -> bool {
        if self.is_err() && other.is_err() {
            return true;
        }

        if self.is_ok() && other.is_ok() {
            let Ok(first) = self else {
                return false;
            };
            let Ok(second) = other else {
                return false;
            };

            let decimals = f32(decimals);
            let max_diff = 10.0_f32.powf(-decimals);

            let aeq = abs(first - second) < max_diff;

            return aeq;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::decimals::approx_eq_dec::ApproxEq;

    #[test]
    fn aeq_f64() {
        let a = 100.123_456_789_f64;
        let b = 100.123_456_712_f64;

        assert!(a.aeq(b, 7));
        assert!(!a.aeq(b, 8));
    }

    #[test]
    fn aeq_option_f64() {
        let a = Some(100.123_456_789_f64);
        let b = Some(100.123_456_712_f64);
        assert!(a.aeq(b, 7));
        assert!(!a.aeq(b, 8));

        let a = None::<f64>;
        let b = None;
        assert!(a.aeq(b, 0));

        let a = Some(100.123_456_789_f64);
        let b = None;
        assert!(!a.aeq(b, 0));

        let a = None;
        let b = Some(100.123_456_789_f64);
        assert!(!a.aeq(b, 0));
    }

    #[test]
    fn aeq_result_f64() {
        let a = Ok::<f64, Error>(100.123_456_789_f64);
        let b = Ok::<f64, Error>(100.123_456_712_f64);
        assert!(a.aeq(b, 7));

        let a = Ok::<f64, Error>(100.123_456_789_f64);
        let b = Ok::<f64, Error>(100.123_456_712_f64);
        assert!(!a.aeq(b, 8));

        let a = Err::<f64, Error>(Error::msg(""));
        let b = Err(Error::msg(""));
        assert!(a.aeq(b, 0));

        let a = Ok::<f64, Error>(100.123_456_789_f64);
        let b = Err(Error::msg(""));
        assert!(!a.aeq(b, 0));

        let a = Err(Error::msg(""));
        let b = Ok::<f64, Error>(100.123_456_789_f64);
        assert!(!a.aeq(b, 0));
    }

    #[test]
    fn aeq_f32() {
        let a = 100.123_45_f32;
        let b = 100.123_31_f32;

        assert!(a.aeq(b, 3));
        assert!(!a.aeq(b, 4));
    }

    #[test]
    fn aeq_option_f32() {
        let a = Some(100.678_9_f32);
        let b = Some(100.671_2_f32);
        assert!(a.aeq(b, 2));
        assert!(!a.aeq(b, 3));

        let a = None::<f32>;
        let b = None;
        assert!(a.aeq(b, 0));

        let a = Some(100.678_9_f32);
        let b = None;
        assert!(!a.aeq(b, 0));

        let a = None;
        let b = Some(100.671_2_f32);
        assert!(!a.aeq(b, 0));
    }

    #[test]
    fn aeq_result_f32() {
        let a = Ok::<f32, Error>(100.456_78_f32);
        let b = Ok::<f32, Error>(100.456_71_f32);
        assert!(a.aeq(b, 4));

        let a = Ok::<f32, Error>(100.456_78_f32);
        let b = Ok::<f32, Error>(100.456_71_f32);
        assert!(!a.aeq(b, 5));

        let a = Err::<f32, Error>(Error::msg(""));
        let b = Err(Error::msg(""));
        assert!(a.aeq(b, 0));

        let a = Ok::<f32, Error>(100.456_78_f32);
        let b = Err(Error::msg(""));
        assert!(!a.aeq(b, 0));

        let a = Err(Error::msg(""));
        let b = Ok::<f32, Error>(100.456_78_f32);
        assert!(!a.aeq(b, 0));
    }
}
