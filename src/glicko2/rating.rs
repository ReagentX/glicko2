/*!
Data structures and convenience methods for creating and interacting with rating data
*/

use crate::glicko2::{constants, tuning::Tuning};

/// Represents a team's Glicko2 rating (mu), distribution (phi), and volatility (sigma).
#[derive(Debug, Copy, Clone)]
pub struct Rating<'a> {
    pub mu: f64,
    pub phi: f64,
    pub sigma: f64,
    pub is_scaled: bool,
    pub(crate) tuning: &'a Tuning,
}

impl<'a> Rating<'a> {
    /// Create a new instance of a Rating based on the provided tuning parameters.
    ///
    /// # Example
    /// ```
    /// use glicko_2::{Rating, Tuning};
    ///
    /// let tuning = Tuning::default();
    /// let team_1 = Rating::new(&tuning);
    /// ```
    pub fn new(tuning: &Tuning) -> Rating {
        Rating {
            mu: tuning.mu,
            phi: tuning.phi,
            sigma: tuning.sigma,
            is_scaled: false,
            tuning,
        }
    }

    /// Scales a rating down to the Glicko2 scale
    pub(crate) fn scale_down(&mut self) {
        if !self.is_scaled {
            let mu = (self.mu - self.tuning.mu) / constants::RATIO;
            let phi = self.phi / constants::RATIO;
            self.mu = mu;
            self.phi = phi;
            self.is_scaled = true;
        }
    }

    /// Scales a rating up to the nominal scale
    pub(crate) fn scale_up(&mut self) {
        if self.is_scaled {
            let mu = (self.mu * constants::RATIO) + self.tuning.mu;
            let phi = self.phi * constants::RATIO;
            self.mu = mu;
            self.phi = phi;
            self.is_scaled = false;
        }
    }

    /// Decay a rating for a team that has not played during a period.
    /// # Example
    /// ```
    /// use glicko_2::{Rating, Tuning};
    /// 
    /// let tuning = Tuning::default();
    /// let mut new_rating = Rating::new(&tuning);
    /// 
    /// new_rating.decay();
    /// ```
    pub fn decay(&mut self) {
        if !self.is_scaled {
            self.scale_down();
        }
        let vinculum = self.phi.powi(2) + self.sigma.powi(2);
        self.phi = vinculum.sqrt();
        self.scale_up();
    }
}
