use num_traits::{abs, Float, Signed};

pub fn aeq<F: Float + std::fmt::Debug + Signed>(a: F, b: F, digits: u8) -> bool {
    let diff = abs(a - b);

    let ten = unsafe { F::from(10).unwrap_unchecked() };
    let digits = unsafe { F::from(digits).unwrap_unchecked() };

    let max_diff = ten.powf(-digits);

    let approximately_equal = diff < max_diff;

    approximately_equal
}

#[cfg(test)]
mod tests {
    use super::aeq;

    #[test]
    fn five_digits() {
        let a = 100.123_456_789;
        let b = 100.123_456_712;

        assert!(aeq(a, b, 7));
        assert!(!aeq(a, b, 8));
    }
}
