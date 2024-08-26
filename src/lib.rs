#![cfg_attr(feature = "no_std", no_std)]

pub mod approx_eq;
pub mod not_approx_eq;
pub mod round_float;

pub use approx_eq::ApproxEq;
pub use not_approx_eq::NApproxEq;
pub use round_float::RoundToFraction;
