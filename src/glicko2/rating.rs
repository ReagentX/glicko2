use crate::glicko2::constants;

#[derive(Debug)]
pub struct Rating {
    pub mu: f64,
    pub phi: f64,
    pub sigma: f64,
    pub is_scaled: bool,
}

impl Rating {
    pub fn scale_down(&mut self) {
        let mu = (self.mu - constants::MU) / constants::RATIO;
        let phi = self.phi / constants::RATIO;
        self.mu = mu;
        self.phi = phi;
        self.is_scaled = true;
    }

    pub fn scale_up(&mut self) {
        let mu = (self.mu * constants::RATIO) + constants::MU;
        let phi = self.phi * constants::RATIO;
        self.mu = mu;
        self.phi = phi;
        self.is_scaled = false;
    }

    pub fn decay(&mut self) {
        self.scale_down();
        let vinculum = self.phi.powi(2) + self.sigma.powi(2);
        self.phi = vinculum.sqrt();
        self.scale_up();
    }
}

pub fn make_rating() -> Rating {
    Rating {
        mu: constants::MU,
        phi: constants::PHI,
        sigma: constants::SIGMA,
        is_scaled: false,
    }
}

pub mod one_on_one {
    use crate::glicko2::rating::Rating;

    pub fn rate(rating1: &Rating, rating2: &Rating) {}

    pub fn quality(rating1: &Rating, rating2: &Rating) {}
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
