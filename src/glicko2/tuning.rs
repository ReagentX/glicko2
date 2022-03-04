/**
Tuning parameters used for rating and algorithm calculations
*/

use crate::constants;

#[derive(Debug)]
pub struct Tuning {
    pub mu: f64,
    pub phi: f64,
    pub sigma: f64,
    pub tau: f64,
}

impl Tuning {
    #[allow(clippy::too_many_arguments)]
    /// Create custom tuning parameters for the Glicko2 algorithm.
    /// The default option uses the values provided by the paper.
    /// 
    /// # Example
    /// 
    /// ```
    /// use glicko2::Tuning;
    /// 
    /// let default_tuning = Tuning::default();
    /// let custom_tuning = Tuning::new(1200.0, 200.0, 0.05, 0.6);
    /// ```
    pub fn new(mu: f64, phi: f64, sigma: f64, tau: f64) -> Self {
        Self {
            mu,
            phi,
            sigma,
            tau,
        }
    }
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            mu: constants::MU,
            phi: constants::PHI,
            sigma: constants::SIGMA,
            tau: constants::TAU,
        }
    }
}
