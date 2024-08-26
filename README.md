# FLoat Plus

## `ApproxEq::aeq(..)` trait method

`aeq` means "approximately equal".
``` rust
    use float_plus::approx_eq::ApproxEq;

    let a = 100.123_456_789_f64;
    let b = 100.123_456_712_f64;

    assert!(a.aeq(b, 7));
    assert!(!a.aeq(b, 8));
```

## `NApproxEq::nae(..)` trait method
`nae` means "not approximately equal".
``` rust
    use float_plus::approx_eq::ApproxNe;
    
    let a = 100.123_456_789_f64;
    let b = 100.123_456_712_f64;

    assert!(!a.nae(b, 7));
    assert!(a.nae(b, 8));
```


## `RoundToFraction` trait
``` rust
    use float_plus::RoundToFraction;

    let before = 100.123_456_789_f64;
    let after = before.round_to_fraction(5);
    assert_eq!(after, 100.123_46);
```



