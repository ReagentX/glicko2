//! Data structures and convienience methods for creating and interacting with rating data

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
    /// let team_1 = glicko2::rating::Rating::new();
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
    /// # Example
    /// ```
    /// let mut new_rating = glicko2::rating::Rating::new();
    /// new_rating.scale_down();
    /// ```
    pub fn scale_down(&mut self) {
        if !self.is_scaled {
            let mu = (self.mu - constants::MU) / constants::RATIO;
            let phi = self.phi / constants::RATIO;
            self.mu = mu;
            self.phi = phi;
            self.is_scaled = true;
        }
    }

    /// Scales a rating down to the nominal scale
    /// # Example
    /// ```
    /// let mut new_rating = glicko2::rating::Rating::new();
    /// new_rating.scale_up();
    /// ```
    pub fn scale_up(&mut self) {
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
    /// let mut new_rating = glicko2::rating::Rating::new();
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

/// Provides functions to handle a single one on one game and update ratings accordingly
pub mod one_on_one {
    use crate::glicko2::algorithm;
    use crate::glicko2::rating::Rating;

    /// Updates ratings for two teams
    /// If the game was a draw, pass `drawn` as `true`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut rating_1 = glicko2::rating::Rating::new();
    /// let mut rating_2 = glicko2::rating::Rating::new();
    /// let (rating_1, rating_2) = glicko2::rating::one_on_one::rate(rating_1, rating_2, false);
    /// ```
    pub fn rate(mut winner: Rating, mut loser: Rating, drawn: bool) -> (Rating, Rating) {
        // drawn is false if Team 1 beat Team 2
        if drawn {
            algorithm::rate(
                &mut winner,
                vec![(super::match_result::Status::Draw, &mut loser)],
            );
            algorithm::rate(
                &mut loser,
                vec![(super::match_result::Status::Draw, &mut winner)],
            );
        } else {
            algorithm::rate(
                &mut winner,
                vec![(super::match_result::Status::Win, &mut loser)],
            );
            algorithm::rate(
                &mut loser,
                vec![(super::match_result::Status::Loss, &mut winner)],
            );
        };
        (winner, loser)
    }

    /// Determines the odds the first team will beat the second team
    ///
    /// # Example
    ///
    /// ```
    /// let mut rating_1 = glicko2::rating::Rating::new();
    /// let mut rating_2 = glicko2::rating::Rating::new();
    /// let odds = glicko2::rating::one_on_one::odds(rating_1, rating_2);
    /// ```
    pub fn odds(mut rating1: Rating, mut rating2: Rating) -> f64 {
        rating1.scale_down();
        rating2.scale_down();
        let expected_score = algorithm::expect_score(
            &rating1,
            &rating2,
            algorithm::reduce_impact(&rating1, &rating2),
        );
        rating1.scale_up();
        rating2.scale_up();
        expected_score
    }

    /// Determines the quality of a matchup, where 1.0 (100%) is a perfect match.
    ///
    /// # Example
    ///
    /// ```
    /// let mut rating_1 = glicko2::rating::Rating::new();
    /// let mut rating_2 = glicko2::rating::Rating::new();
    /// let quality = glicko2::rating::one_on_one::quality(rating_1, rating_2);
    /// ```
    pub fn quality(rating1: Rating, rating2: Rating) -> f64 {
        // 1.0 if perfect match
        let expected_score_1 = odds(rating1, rating2);
        let expected_score_2 = odds(rating2, rating1);
        let advantage = expected_score_1 - expected_score_2; // Advantage team 1 has over team 2
        1.0 - advantage.abs()
    }
}

/// Enums for the Glicko2 values for match outcomes
pub mod match_result {
    use crate::glicko2::constants;

    #[derive(Debug)]
    pub enum Status {
        Win,
        Draw,
        Loss,
    }

    /// Gets the constant float value associated with each outcome
    pub fn val(status: &Status) -> f64 {
        match status {
            Status::Win => constants::WIN,
            Status::Draw => constants::DRAW,
            Status::Loss => constants::LOSS,
        }
    }
}
