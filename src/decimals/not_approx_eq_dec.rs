use cast::f32;
use cast::f64;
use num_traits::abs;

pub trait NApproxEq {
    fn nae(&self, other: Self, decimals: u8) -> bool;
}

impl NApproxEq for f64 {
    fn nae(&self, other: Self, decimals: u8) -> bool {
        let decimals = f64(decimals);
        let max_diff = 10.0_f64.powf(-decimals);

        let nae = abs(self - other) > max_diff;
        nae
    }
}

impl NApproxEq for Option<f64> {
    fn nae(&self, other: Self, decimals: u8) -> bool {
        if self.is_none() && other.is_none() {
            return false;
        }

        if self.is_some() && other.is_some() {
            let Some(first) = self else {
                return true;
            };
            let Some(second) = other else {
                return true;
            };

            let decimals = f64(decimals);
            let max_diff = 10.0_f64.powf(-decimals);

            let nae = abs(first - second) > max_diff;

            return nae;
        }

        true
    }
}

impl<E> NApproxEq for Result<f64, E> {
    fn nae(&self, other: Self, decimals: u8) -> bool {
        if self.is_err() && other.is_err() {
            return false;
        }

        if self.is_ok() && other.is_ok() {
            let Ok(first) = self else {
                return true;
            };
            let Ok(second) = other else {
                return true;
            };

            let decimals = f64(decimals);
            let max_diff = 10.0_f64.powf(-decimals);

            let nae = abs(first - second) > max_diff;

            return nae;
        }

        true
    }
}

impl NApproxEq for f32 {
    fn nae(&self, other: Self, decimals: u8) -> bool {
        let decimals = f32(decimals);
        let max_diff = 10.0_f32.powf(-decimals);

        let nae = abs(self - other) > max_diff;

        nae
    }
}

impl NApproxEq for Option<f32> {
    fn nae(&self, other: Self, decimals: u8) -> bool {
        if self.is_none() && other.is_none() {
            return false;
        }

        if self.is_some() && other.is_some() {
            let first = self.unwrap();
            let second = other.unwrap();

            let decimals = f32(decimals);
            let max_diff = 10.0_f32.powf(-decimals);

            let nae = abs(first - second) > max_diff;

            return nae;
        }

        true
    }
}

impl<E> NApproxEq for Result<f32, E> {
    fn nae(&self, other: Self, decimals: u8) -> bool {
        if self.is_err() && other.is_err() {
            return false;
        }

        if self.is_ok() && other.is_ok() {
            let Ok(first) = self else {
                return true;
            };
            let Ok(second) = other else {
                return true;
            };

            let decimals = f32(decimals);
            let max_diff = 10.0_f32.powf(-decimals);

            let nae = abs(first - second) > max_diff;

            return nae;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::decimals::not_approx_eq_dec::NApproxEq;

    #[test]
    fn nae_f64() {
        let a = 100.123_456_789_f64;
        let b = 100.123_456_712_f64;

        assert!(!a.nae(b, 7));
        assert!(a.nae(b, 8));
    }

    #[test]
    fn nae_option_f64() {
        let a = Some(100.123_456_789_f64);
        let b = Some(100.123_456_712_f64);
        assert!(!a.nae(b, 7));
        assert!(a.nae(b, 8));

        let a = None::<f64>;
        let b = None;
        assert!(!a.nae(b, 0));

        let a = Some(100.123_456_789_f64);
        let b = None;
        assert!(a.nae(b, 0));

        let a = None;
        let b = Some(100.123_456_789_f64);
        assert!(a.nae(b, 0));
    }

    #[test]
    fn nae_result_f64() {
        let a = Ok::<f64, Error>(100.123_456_789_f64);
        let b = Ok::<f64, Error>(100.123_456_712_f64);
        assert!(!a.nae(b, 7));

        let a = Ok::<f64, Error>(100.123_456_789_f64);
        let b = Ok::<f64, Error>(100.123_456_712_f64);
        assert!(a.nae(b, 8));

        let a = Err::<f64, Error>(Error::msg(""));
        let b = Err(Error::msg(""));
        assert!(!a.nae(b, 0));

        let a = Ok::<f64, Error>(100.123_456_789_f64);
        let b = Err(Error::msg(""));
        assert!(a.nae(b, 0));

        let a = Err(Error::msg(""));
        let b = Ok::<f64, Error>(100.123_456_789_f64);
        assert!(a.nae(b, 0));
    }

    #[test]
    fn nae_f32() {
        let a = 100.123_45_f32;
        let b = 100.123_31_f32;

        assert!(!a.nae(b, 3));
        assert!(a.nae(b, 4));
    }

    #[test]
    fn nae_option_f32() {
        let a = Some(100.678_9_f32);
        let b = Some(100.671_2_f32);
        assert!(!a.nae(b, 2));
        assert!(a.nae(b, 3));

        let a = None::<f32>;
        let b = None;
        assert!(!a.nae(b, 0));

        let a = Some(100.678_9_f32);
        let b = None;
        assert!(a.nae(b, 0));

        let a = None;
        let b = Some(100.671_2_f32);
        assert!(a.nae(b, 0));
    }

    #[test]
    fn nae_result_f32() {
        let a = Ok::<f32, Error>(100.456_78_f32);
        let b = Ok::<f32, Error>(100.456_71_f32);
        assert!(!a.nae(b, 4));

        let a = Ok::<f32, Error>(100.456_78_f32);
        let b = Ok::<f32, Error>(100.456_71_f32);
        assert!(a.nae(b, 5));

        let a = Err::<f32, Error>(Error::msg(""));
        let b = Err(Error::msg(""));
        assert!(!a.nae(b, 0));

        let a = Ok::<f32, Error>(100.456_78_f32);
        let b = Err(Error::msg(""));
        assert!(a.nae(b, 0));

        let a = Err(Error::msg(""));
        let b = Ok::<f32, Error>(100.456_78_f32);
        assert!(a.nae(b, 0));
    }
}
