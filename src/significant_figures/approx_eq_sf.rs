#![allow(clippy::float_cmp)]

use super::round_sf::RoundToSigDig;

pub trait ApproxEqSf {
    fn aeq_sf(&self, b: Self, significant_digits: u8) -> bool;
}

impl ApproxEqSf for f64 {
    fn aeq_sf(&self, other: Self, significant_digits: u8) -> bool {
        let first = self.round_to_sf(significant_digits);
        let second = other.round_to_sf(significant_digits);

        let aeq_sf = first == second;

        aeq_sf
    }
}

impl ApproxEqSf for Option<f64> {
    fn aeq_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let aeq_sf = first == second;

            return aeq_sf;
        }

        false
    }
}

impl<E> ApproxEqSf for Result<f64, E> {
    fn aeq_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let aeq_sf = first == second;

            return aeq_sf;
        }

        false
    }
}

impl ApproxEqSf for f32 {
    fn aeq_sf(&self, other: Self, significant_digits: u8) -> bool {
        let first = self.round_to_sf(significant_digits);
        let second = other.round_to_sf(significant_digits);

        let aeq = first == second;

        aeq
    }
}

impl ApproxEqSf for Option<f32> {
    fn aeq_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let aeq_sf = first == second;

            return aeq_sf;
        }

        false
    }
}

impl<E> ApproxEqSf for Result<f32, E> {
    fn aeq_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let aeq_sf = first == second;

            return aeq_sf;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::significant_figures::approx_eq_sf::ApproxEqSf;

    #[test]
    fn aeq_sf_f64() {
        let a = 100.123_456_789_f64;
        let b = 100.123_457;
        assert!(a.aeq_sf(b, 9));

        let a = 100.123_454_789_f64;
        let b = 100.123_457;
        assert!(!a.aeq_sf(b, 9));
    }

    #[test]
    fn aeq_sf_option_f64() {
        let a = Some(100.123_456_789_f64);
        let b = Some(100.123_5_f64);
        assert!(a.aeq_sf(b, 7));

        let a = Some(100.123_456_789_f64);
        let b = None;
        assert!(!a.aeq_sf(b, 7));

        let a = None;
        let b = Some(100.123_5_f64);
        assert!(!a.aeq_sf(b, 7));

        let a = None::<f64>;
        let b = None;
        assert!(a.aeq_sf(b, 7));
    }

    #[test]
    fn aeq_sf_result_f64() {
        let a = Ok::<_, Error>(100.123_456_789_f64);
        let b = Ok(100.123_5_f64);
        assert!(a.aeq_sf(b, 7));

        let a = Ok::<_, Error>(100.123_456_789_f64);
        let b = Err(Error::msg("message"));
        assert!(!a.aeq_sf(b, 7));

        let a = Err(Error::msg("message"));
        let b = Ok::<_, Error>(100.123_5_f64);
        assert!(!a.aeq_sf(b, 7));

        let a = Err::<f64, _>(Error::msg("message"));
        let b = Err(Error::msg("message"));
        assert!(a.aeq_sf(b, 7));
    }

    #[test]
    fn aeq_sf_f32() {
        let a = 100.456_7_f32;
        let b = 100.457;

        assert!(a.aeq_sf(b, 6));

        let a = 100.456_4_f32;
        let b = 100.457;
        assert!(!a.aeq_sf(b, 6));
    }

    #[test]
    fn aeq_sf_option_f32() {
        let a = Some(100.456_7_f32);
        let b = Some(100.5_f32);
        assert!(a.aeq_sf(b, 4));

        let a = Some(100.456_7_f32);
        let b = None;
        assert!(!a.aeq_sf(b, 4));

        let a = None;
        let b = Some(100.5_f32);
        assert!(!a.aeq_sf(b, 4));

        let a = None::<f32>;
        let b = None;
        assert!(a.aeq_sf(b, 4));
    }

    #[test]
    fn aeq_sf_result_f32() {
        let a = Ok::<_, Error>(100.456_7_f32);
        let b = Ok(100.5_f32);
        assert!(a.aeq_sf(b, 4));

        let a = Ok::<_, Error>(100.456_7_f32);
        let b = Err(Error::msg("message"));
        assert!(!a.aeq_sf(b, 4));

        let a = Err(Error::msg("message"));
        let b = Ok::<_, Error>(100.5_f32);
        assert!(!a.aeq_sf(b, 4));

        let a = Err::<f32, _>(Error::msg("message"));
        let b = Err(Error::msg("message"));
        assert!(a.aeq_sf(b, 4));
    }
}
