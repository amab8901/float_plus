# FLoat Plus

This library contains traits that extend the capabilities of `f32` and `f64`.

## `RoundToSigDig::round_to_sf(..)`
``` rust
    let before = 123.123_456_789_f64;
    let after = before.round_to_sf(9);
    assert_eq!(after, 123.123_457_f64);
```

## `ApproxEqSf::aeq_sf(..)`
``` rust
    let a = 100.123_456_789_f64;
    let b = 100.123_457;
    assert!(a.aeq_sf(b, 9));

    let a = 100.123_454_789_f64;
    let b = 100.123_457;
    assert!(!a.aeq_sf(b, 9));
```

## `NApproxEqSf::nae_sf(..)`
``` rust
    let a = 100.123_456_789_f64;
    let b = 100.123_457;
    assert!(!a.nae_sf(b, 9));

    let a = 100.123_454_789_f64;
    let b = 100.123_457;
    assert!(a.nae_sf(b, 9));
```

## `ApproxEq::aeq(..)`
``` rust
    use float_plus::approx_eq::ApproxEq;

    let a = 100.123_456_789_f64;
    let b = 100.123_456_712_f64;

    assert!(a.aeq(b, 7));
    assert!(!a.aeq(b, 8));
```

## `NApproxEq::nae(..)` 
``` rust
    use float_plus::approx_eq::ApproxNe;
    
    let a = 100.123_456_789_f64;
    let b = 100.123_456_712_f64;

    assert!(!a.nae(b, 7));
    assert!(a.nae(b, 8));
```

## `RoundToFraction`
``` rust
    use float_plus::RoundToFraction;

    let before = 100.123_456_789_f64;
    let after = before.round_to_fraction(5);
    assert_eq!(after, 100.123_46);
```
