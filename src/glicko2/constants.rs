/*!
Constants required by the Glicko2 Algorithm
*/

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

/// Glicko-2 scale ratio
pub const RATIO: f64 = 173.7178;

/// Value for win
pub const WIN: f64 = 1.0;

/// Value for draw
pub const DRAW: f64 = 0.5;

/// Value for loss
pub const LOSS: f64 = 0.0;
