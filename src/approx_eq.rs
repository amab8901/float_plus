use num_traits::Float;

use crate::RoundToFraction;

pub fn aeq<F: Float + std::fmt::Debug>(a: F, b: F, digits: u32) -> bool {
    let diff = a - b;

    dbg!(&diff);

    let a = a.round_to_fraction(digits);
    let b = b.round_to_fraction(digits);
    let equal = a == b;

    equal
}

#[cfg(test)]
mod tests {
    use super::aeq;

    #[test]
    fn five_digits() {
        let a = 100.123_456_789;
        let b = 100.123_456_712;

        assert!(aeq(a, b, 6));
        assert!(!aeq(a, b, 7));
    }
}
