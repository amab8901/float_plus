#![cfg_attr(feature = "no_std", no_std)]

pub mod decimals;
pub mod significant_figures;

pub use decimals::approx_eq_dec::ApproxEq;
pub use decimals::not_approx_eq_dec::NApproxEq;
pub use decimals::round_float_dec::RoundToFraction;
