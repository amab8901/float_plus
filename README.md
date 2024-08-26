# FLoat Plus

## `aeq(..)` method

`aeq` means "approximately equal".
``` rust
    use float_plus::aeq;

    let a = 100.123_456_789;
    let b = 100.123_456_712;

    assert!(aeq(a, b, 6));
    assert!(!aeq(a, b, 7));
```


## `RoundToFraction` trait
``` rust
    use float_plus::RoundToFraction;

    let before = 100.123_456_789_f64;
    let after = before.round_to_fraction(5);
    assert_eq!(after, 100.123_46);
```