use crate::glicko2::constants;

#[derive(Debug, Copy, Clone)]
pub struct Rating {
    pub mu: f64,
    pub phi: f64,
    pub sigma: f64,
    pub is_scaled: bool,
}

impl Rating {
    /// Create a new instance of a Rating
    /// ```
    /// use glicko2::glicko2::rating::Rating;
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
    pub fn decay(&mut self) {
        if !self.is_scaled {
            self.scale_down();
        }
        let vinculum = self.phi.powi(2) + self.sigma.powi(2);
        self.phi = vinculum.sqrt();
        self.scale_up();
    }
}

pub mod one_on_one {
    use crate::glicko2::algorithm;
    use crate::glicko2::rating::Rating;

    pub fn rate(mut rating1: Rating, mut rating2: Rating, drawn: bool) -> (Rating, Rating) {
        // drawn is false if Team 1 beat Team 2
        if drawn {
            algorithm::rate(
                &mut rating1,
                vec![(super::match_result::Status::Draw, &mut rating2)],
            );
            algorithm::rate(
                &mut rating2,
                vec![(super::match_result::Status::Draw, &mut rating1)],
            );
        } else {
            algorithm::rate(
                &mut rating1,
                vec![(super::match_result::Status::Win, &mut rating2)],
            );
            algorithm::rate(
                &mut rating2,
                vec![(super::match_result::Status::Loss, &mut rating1)],
            );
        };
        (rating1, rating2)
    }

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

    pub fn quality(rating1: Rating, rating2: Rating) -> f64 {
        // 1.0 if perfect match
        let expected_score_1 = odds(rating1, rating2);
        let expected_score_2 = odds(rating2, rating1);
        let advantage = expected_score_1 - expected_score_2; // Advantage team 1 has over team 2
        1.0 - advantage.abs()
    }
}

pub mod match_result {
    use crate::glicko2::constants;

    #[derive(Debug)]
    pub enum Status {
        Win,
        Draw,
        Loss,
    }

    pub fn val(status: &Status) -> f64 {
        return match status {
            Status::Win => constants::WIN,
            Status::Draw => constants::DRAW,
            Status::Loss => constants::LOSS,
        };
    }
}
