#![allow(clippy::float_cmp)]

use super::round_sf::RoundToSigDig;

pub trait NApproxEqSf {
    fn nae_sf(&self, b: Self, significant_digits: u8) -> bool;
}

impl NApproxEqSf for f64 {
    fn nae_sf(&self, other: Self, significant_digits: u8) -> bool {
        let first = self.round_to_sf(significant_digits);
        let second = other.round_to_sf(significant_digits);

        let nae_sf = first != second;

        nae_sf
    }
}

impl NApproxEqSf for Option<f64> {
    fn nae_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            dbg!(&first);
            dbg!(&second);

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

impl<E> NApproxEqSf for Result<f64, E> {
    fn nae_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

impl NApproxEqSf for f32 {
    fn nae_sf(&self, other: Self, significant_digits: u8) -> bool {
        let first = self.round_to_sf(significant_digits);
        let second = other.round_to_sf(significant_digits);

        let aeq = first != second;

        aeq
    }
}

impl NApproxEqSf for Option<f32> {
    fn nae_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

impl<E> NApproxEqSf for Result<f32, E> {
    fn nae_sf(&self, other: Self, significant_digits: u8) -> bool {
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

            let first = first.round_to_sf(significant_digits);
            let second = second.round_to_sf(significant_digits);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::significant_figures::not_approx_eq_sf::NApproxEqSf;

    #[test]
    fn nae_sf_f64() {
        let a = 100.123_456_789_f64;
        let b = 100.123_457;
        assert!(!a.nae_sf(b, 9));

        let a = 100.123_454_789_f64;
        let b = 100.123_457;
        assert!(a.nae_sf(b, 9));
    }

    #[test]
    fn nae_sf_option_f64() {
        let a = Some(100.123_456_789_f64);
        let b = Some(100.123_5_f64);
        assert!(!a.nae_sf(b, 7));

        let a = Some(100.123_456_789_f64);
        let b = None;
        assert!(a.nae_sf(b, 7));

        let a = None;
        let b = Some(100.123_5_f64);
        assert!(a.nae_sf(b, 7));

        let a = None::<f64>;
        let b = None;
        assert!(!a.nae_sf(b, 7));
    }

    #[test]
    fn nae_sf_result_f64() {
        let a = Ok::<_, Error>(100.123_456_789_f64);
        let b = Ok(100.123_5_f64);
        assert!(!a.nae_sf(b, 7));

        let a = Ok::<_, Error>(100.123_456_789_f64);
        let b = Err(Error::msg("message"));
        assert!(a.nae_sf(b, 7));

        let a = Err(Error::msg("message"));
        let b = Ok::<_, Error>(100.123_5_f64);
        assert!(a.nae_sf(b, 7));

        let a = Err::<f64, _>(Error::msg("message"));
        let b = Err(Error::msg("message"));
        assert!(!a.nae_sf(b, 7));
    }

    #[test]
    fn nae_sf_f32() {
        let a = 100.456_7_f32;
        let b = 100.457;

        assert!(!a.nae_sf(b, 6));

        let a = 100.456_4_f32;
        let b = 100.457;
        assert!(a.nae_sf(b, 6));
    }

    #[test]
    fn nae_sf_option_f32() {
        let a = Some(100.456_7_f32);
        let b = Some(100.5_f32);
        assert!(!a.nae_sf(b, 4));

        let a = Some(100.456_7_f32);
        let b = None;
        assert!(a.nae_sf(b, 4));

        let a = None;
        let b = Some(100.5_f32);
        assert!(a.nae_sf(b, 4));

        let a = None::<f32>;
        let b = None;
        assert!(!a.nae_sf(b, 4));
    }

    #[test]
    fn nae_sf_result_f32() {
        let a = Ok::<_, Error>(100.456_7_f32);
        let b = Ok(100.5_f32);
        assert!(!a.nae_sf(b, 4));

        let a = Ok::<_, Error>(100.456_7_f32);
        let b = Err(Error::msg("message"));
        assert!(a.nae_sf(b, 4));

        let a = Err(Error::msg("message"));
        let b = Ok::<_, Error>(100.5_f32);
        assert!(a.nae_sf(b, 4));

        let a = Err::<f32, _>(Error::msg("message"));
        let b = Err(Error::msg("message"));
        assert!(!a.nae_sf(b, 4));
    }
}
