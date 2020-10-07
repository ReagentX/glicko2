use std::f64::consts::LN_10;

/// Default rating
pub const MU: f64 = 1500.0;

/// Default variance
pub const PHI: f64 = 350.0;

/// Default volatility
pub const SIGMA: f64 = 0.006;

/// Default sensitivity to upsets
pub const TAU: f64 = 1.3;

/// Default convergence tolerance
pub const EPSILON: f64 = 0.0000001;

/// A constant which is used to standardize the logistic function to `1/(1+exp(-x))` from `1/(1+10^(-r/400))`
pub const Q: f64 = LN_10 / 400.0;

/// Glicko-2 scale ratio
pub const RATIO: f64 =  173.7178;

/// Value for win
pub const WIN: f64 = 1.0;

/// Value for draw
pub const DRAW: f64 = 0.5;

/// Value for loss
pub const LOSS: f64 = 0.0;
