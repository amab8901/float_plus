#![cfg_attr(feature = "no_std", no_std)]

pub mod approx_eq;
pub mod round_float;

pub use round_float::RoundToFraction;
