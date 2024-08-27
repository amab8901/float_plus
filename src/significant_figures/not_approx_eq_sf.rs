#![allow(clippy::float_cmp)]

#[cfg(feature = "num")]
use num::complex::{Complex32, Complex64};

use super::round_sf::RoundToSigDig;

pub trait NApproxEqSf {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool;
}

impl NApproxEqSf for f64 {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        let first = self.round_to_sf(significant_figures);
        let second = other.round_to_sf(significant_figures);

        let nae_sf = first != second;

        nae_sf
    }
}

impl NApproxEqSf for Option<f64> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
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

            let first = first.round_to_sf(significant_figures);
            let second = second.round_to_sf(significant_figures);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

impl<E> NApproxEqSf for Result<f64, E> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
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

            let first = first.round_to_sf(significant_figures);
            let second = second.round_to_sf(significant_figures);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

#[cfg(feature = "num")]
impl NApproxEqSf for Complex64 {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        let real_nae_sf = self.re.nae_sf(other.re, significant_figures);
        let imaginary_nae_sf = self.im.nae_sf(other.im, significant_figures);
        let nae_sf = real_nae_sf || imaginary_nae_sf;

        nae_sf
    }
}

#[cfg(feature = "num")]
impl NApproxEqSf for Option<Complex64> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        if self.is_some() && other.is_some() {
            let first = unsafe { self.unwrap_unchecked() };
            let second = unsafe { other.unwrap_unchecked() };
            let real_nae_sf = first.re.nae_sf(second.re, significant_figures);
            let imaginary_nae_sf = first.im.nae_sf(second.im, significant_figures);
            let nae_sf = real_nae_sf || imaginary_nae_sf;

            return nae_sf;
        }

        if self.is_none() && other.is_none() {
            return false;
        }

        true
    }
}

#[cfg(feature = "num")]
impl<E> NApproxEqSf for Result<Complex64, E> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        if self.is_ok() && other.is_ok() {
            let first = unsafe { self.as_ref().unwrap_unchecked() };
            let second = unsafe { other.as_ref().unwrap_unchecked() };
            let real_nae_sf = first.re.nae_sf(second.re, significant_figures);
            let imaginary_nae_sf = first.im.nae_sf(second.im, significant_figures);
            let nae_sf = real_nae_sf || imaginary_nae_sf;

            return nae_sf;
        }

        if self.is_err() && other.is_err() {
            return false;
        }

        true
    }
}

impl NApproxEqSf for f32 {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        let first = self.round_to_sf(significant_figures);
        let second = other.round_to_sf(significant_figures);

        let aeq = first != second;

        aeq
    }
}

impl NApproxEqSf for Option<f32> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
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

            let first = first.round_to_sf(significant_figures);
            let second = second.round_to_sf(significant_figures);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

impl<E> NApproxEqSf for Result<f32, E> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
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

            let first = first.round_to_sf(significant_figures);
            let second = second.round_to_sf(significant_figures);
            let nae_sf = first != second;

            return nae_sf;
        }

        true
    }
}

#[cfg(feature = "num")]
impl NApproxEqSf for Complex32 {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        let real_nae_sf = self.re.nae_sf(other.re, significant_figures);
        let imaginary_nae_sf = self.im.nae_sf(other.im, significant_figures);
        let nae_sf = real_nae_sf || imaginary_nae_sf;

        nae_sf
    }
}

#[cfg(feature = "num")]
impl NApproxEqSf for Option<Complex32> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        if self.is_some() && other.is_some() {
            let first = unsafe { self.unwrap_unchecked() };
            let second = unsafe { other.unwrap_unchecked() };
            let real_nae_sf = first.re.nae_sf(second.re, significant_figures);
            let imaginary_nae_sf = first.im.nae_sf(second.im, significant_figures);
            let nae_sf = real_nae_sf || imaginary_nae_sf;

            return nae_sf;
        }

        if self.is_none() && other.is_none() {
            return false;
        }

        true
    }
}

#[cfg(feature = "num")]
impl<E> NApproxEqSf for Result<Complex32, E> {
    fn nae_sf(&self, other: Self, significant_figures: u8) -> bool {
        if self.is_ok() && other.is_ok() {
            let first = unsafe { self.as_ref().unwrap_unchecked() };
            let second = unsafe { other.as_ref().unwrap_unchecked() };
            let real_nae_sf = first.re.nae_sf(second.re, significant_figures);
            let imaginary_nae_sf = first.im.nae_sf(second.im, significant_figures);
            let nae_sf = real_nae_sf || imaginary_nae_sf;

            return nae_sf;
        }

        if self.is_err() && other.is_err() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    #[cfg(feature = "num")]
    use num::complex::Complex64;

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

    #[cfg(feature = "num")]
    #[test]
    fn nae_sf_complex_f64() {
        let first = Complex64::new(0.0, 0.0);
        let second = Complex64::new(0.0, 0.0);
        assert!(!first.nae_sf(second, 10));

        let first = Complex64::new(0.1, 0.0);
        let second = Complex64::new(0.0, 0.0);
        assert!(first.nae_sf(second, 10));

        let first = Complex64::new(0.0, 0.1);
        let second = Complex64::new(0.0, 0.0);
        assert!(first.nae_sf(second, 10));

        let first = Complex64::new(0.0, 0.0);
        let second = Complex64::new(0.1, 0.0);
        assert!(first.nae_sf(second, 10));

        let first = Complex64::new(0.0, 0.0);
        let second = Complex64::new(0.0, 0.1);
        assert!(first.nae_sf(second, 10));
    }

    #[cfg(feature = "num")]
    #[test]
    fn nae_sf_complex_option_f64() {
        let first = Some(Complex64::new(0.0, 0.0));
        let second = Some(Complex64::new(0.0, 0.0));
        assert!(!first.nae_sf(second, 10));

        let first = Some(Complex64::new(0.1, 0.0));
        let second = Some(Complex64::new(0.0, 0.0));
        assert!(first.nae_sf(second, 10));

        let first = Some(Complex64::new(0.0, 0.1));
        let second = Some(Complex64::new(0.0, 0.0));
        assert!(first.nae_sf(second, 10));

        let first = Some(Complex64::new(0.0, 0.0));
        let second = Some(Complex64::new(0.1, 0.0));
        assert!(first.nae_sf(second, 10));

        let first = Some(Complex64::new(0.0, 0.0));
        let second = Some(Complex64::new(0.0, 0.1));
        assert!(first.nae_sf(second, 10));

        let first = Some(Complex64::new(0.0, 0.0));
        let second = None;
        assert!(first.nae_sf(second, 10));

        let first = None;
        let second = Some(Complex64::new(0.0, 0.0));
        assert!(first.nae_sf(second, 10));

        let first = None::<Complex64>;
        let second = None;
        assert!(!first.nae_sf(second, 10));
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
