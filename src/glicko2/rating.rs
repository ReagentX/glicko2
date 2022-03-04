/*!
Data structures and convenience methods for creating and interacting with rating data
*/

use crate::glicko2::constants;

/// Represents a team's Glicko2 rating (mu), distribution (phi), and volatility (sigma).
#[derive(Debug, Copy, Clone)]
pub struct Rating {
    pub mu: f64,
    pub phi: f64,
    pub sigma: f64,
    pub is_scaled: bool,
}

impl Rating {
    /// Create a new instance of a Rating with default constants.
    ///
    /// # Example
    /// ```
    /// use glicko2::Rating;
    ///
    /// let team_1 = Rating::new();
    /// ```
    pub fn new() -> Rating {
        Rating {
            mu: constants::MU,
            phi: constants::PHI,
            sigma: constants::SIGMA,
            is_scaled: false,
        }
    }

    /// Scales a rating down to the Glicko2 scale
    pub(crate) fn scale_down(&mut self) {
        if !self.is_scaled {
            let mu = (self.mu - constants::MU) / constants::RATIO;
            let phi = self.phi / constants::RATIO;
            self.mu = mu;
            self.phi = phi;
            self.is_scaled = true;
        }
    }

    /// Scales a rating up to the nominal scale
    pub(crate) fn scale_up(&mut self) {
        if self.is_scaled {
            let mu = (self.mu * constants::RATIO) + constants::MU;
            let phi = self.phi * constants::RATIO;
            self.mu = mu;
            self.phi = phi;
            self.is_scaled = false;
        }
    }

    /// Decay a rating for a team that has not played during a period
    /// # Example
    /// ```
    /// use glicko2::Rating;
    ///
    /// let mut new_rating = Rating::new();
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

impl Default for Rating {
    fn default() -> Self {
        Self::new()
    }
}
