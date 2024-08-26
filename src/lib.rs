#![cfg_attr(feature = "no_std", no_std)]

pub mod decimals;
pub mod significant_figures;

pub use decimals::approx_eq_dec::ApproxEq;
pub use decimals::not_approx_eq_dec::NApproxEq;
pub use decimals::round_dec::RoundToFraction;

pub use significant_figures::approx_eq_sf::ApproxEqSf;
// pub use significant_figures::not_approx_eq_sf;
pub use significant_figures::round_sf::RoundToSigDig;
