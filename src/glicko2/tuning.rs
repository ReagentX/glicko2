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
    fn new(mu: f64, phi: f64, sigma: f64, tau: f64) -> Self {
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