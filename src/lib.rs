#![allow(dead_code)]
#[doc = include_str!("../README.md")]

// Expose the module
pub mod glicko2;
// Re-export so we can use these without reaching into the crate
pub use crate::glicko2::{algorithm, constants, rating};

#[cfg(test)]
mod tests {
    use crate::glicko2::algorithm;
    use crate::glicko2::constants::{EPSILON, MU, PHI, Q, RATIO, SIGMA, TAU};
    use crate::glicko2::rating::{match_result, one_on_one, Rating};

    #[test]
    fn win() {
        let win = match_result::Status::Win;
        let win_val = match_result::val(&win);
        println!("{:?}\t{:?}", win, win_val);
        assert_eq!(win_val, 1.)
    }

    #[test]
    fn draw() {
        let draw = match_result::Status::Draw;
        let draw_val = match_result::val(&draw);
        println!("{:?}\t{:?}", draw, draw_val);
        assert_eq!(draw_val, 0.5)
    }

    #[test]
    fn loss() {
        let loss = match_result::Status::Loss;
        let loss_val = match_result::val(&loss);
        println!("{:?}\t{:?}", loss, loss_val);
        assert_eq!(loss_val, 0.0)
    }

    #[test]
    fn constants() {
        assert_eq!(MU, 1500.0);
        assert_eq!(PHI, 350.0);
        assert_eq!(SIGMA, 0.006);
        assert_eq!(TAU, 1.3);
        assert_eq!(EPSILON, 0.0000001);
        assert_eq!(Q, 0.005756462732485115);
        assert_eq!(RATIO, 173.7178);
    }

    #[test]
    fn create_rating() {
        let new_rating = Rating::new();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 1500.0);
        assert_eq!(new_rating.phi, 350.0);
        assert_eq!(new_rating.sigma, 0.006);
        assert!(!new_rating.is_scaled);
    }

    #[test]
    fn scale_down() {
        let mut new_rating = Rating::new();
        new_rating.scale_down();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 0.0);
        assert_eq!(new_rating.phi, 2.014761872416068);
        assert_eq!(new_rating.sigma, 0.006);
        assert!(new_rating.is_scaled);
    }

    #[test]
    fn scale_down_already_down() {
        let mut new_rating = Rating::new();
        new_rating.scale_down();
        new_rating.scale_down();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 0.0);
        assert_eq!(new_rating.phi, 2.014761872416068);
        assert_eq!(new_rating.sigma, 0.006);
        assert!(new_rating.is_scaled);
    }

    #[test]
    fn scale_up() {
        let mut new_rating = Rating::new();
        new_rating.scale_down();
        new_rating.scale_up();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 1500.0);
        assert_eq!(new_rating.phi, 350.0);
        assert_eq!(new_rating.sigma, 0.006);
        assert!(!new_rating.is_scaled);
    }

    #[test]
    fn scale_up_already_up() {
        let mut new_rating = Rating::new();
        new_rating.scale_up();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 1500.0);
        assert_eq!(new_rating.phi, 350.0);
        assert_eq!(new_rating.sigma, 0.006);
        assert!(!new_rating.is_scaled);
    }

    #[test]
    fn reduce_impact() {
        let mut new_rating = Rating::new();
        let mut other_rating = Rating {
            mu: 1450.0,
            phi: 200.0,
            sigma: 0.0059,
            is_scaled: false,
        };
        new_rating.scale_down();
        other_rating.scale_down();
        let impact = algorithm::reduce_impact(&new_rating, &other_rating);
        assert_eq!(impact, 0.6158349285183401);
    }

    #[test]
    #[should_panic]
    fn reduce_impact_unscaled() {
        let new_rating = Rating::new();
        let other_rating = Rating {
            mu: 1450.0,
            phi: 200.0,
            sigma: 0.0059,
            is_scaled: false,
        };
        let impact = algorithm::reduce_impact(&new_rating, &other_rating);
        assert_eq!(impact, 0.0);
    }

    #[test]
    fn compare() {
        let mut new_rating = Rating::new();
        let mut other_rating = Rating {
            mu: 1450.0,
            phi: 200.0,
            sigma: 0.0059,
            is_scaled: false,
        };
        algorithm::rate(
            &mut new_rating,
            vec![(match_result::Status::Win, &mut other_rating)],
        );
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 1643.2406803139988);
        assert_eq!(new_rating.phi, 297.7383025722689);
        assert_eq!(new_rating.sigma, 0.005999997552929708);
        assert!(!new_rating.is_scaled);
    }

    #[test]
    fn rate_win() {
        let new_rating = Rating::new();
        let other_rating = Rating {
            mu: 1450.0,
            phi: 200.0,
            sigma: 0.0059,
            is_scaled: false,
        };
        let (new_rating, other_rating) = one_on_one::rate(new_rating, other_rating, false);
        println!("New: {:?}", new_rating);
        assert_eq!(new_rating.mu, 1643.2406803139988);
        assert_eq!(new_rating.phi, 297.7383025722689);
        assert_eq!(new_rating.sigma, 0.005999997552929708);
        assert!(!new_rating.is_scaled);

        println!("Other: {:?}", other_rating);
        assert_eq!(other_rating.mu, 1476.3887184474581);
        assert_eq!(other_rating.phi, 188.4371651087283);
        assert_eq!(other_rating.sigma, 0.005899995780089439);
        assert!(!other_rating.is_scaled);
    }

    #[test]
    fn rate_draw() {
        let new_rating = Rating::new();
        let other_rating = Rating {
            mu: 1450.0,
            phi: 200.0,
            sigma: 0.0059,
            is_scaled: false,
        };
        let (new_rating, other_rating) = one_on_one::rate(new_rating, other_rating, true);
        println!("New: {:?}", new_rating);
        assert_eq!(new_rating.mu, 1486.1105693882885);
        assert_eq!(new_rating.phi, 297.7383025710809);
        assert_eq!(new_rating.sigma, 0.0059999938227804145);
        assert!(!new_rating.is_scaled);

        println!("Other: {:?}", other_rating);
        assert_eq!(other_rating.mu, 1502.4424933614428);
        assert_eq!(other_rating.phi, 187.0189343872027);
        assert_eq!(other_rating.sigma, 0.005899991810542997);
        assert!(!other_rating.is_scaled);
    }

    #[test]
    fn odds() {
        // Create a rating object for each team
        let rating_1 = Rating::new();
        let rating_2 = Rating::new();

        // Update ratings for team_1 beating team_2
        let (rating_1, rating_2) = one_on_one::rate(rating_1, rating_2, false);

        // Get odds (percent chance team_1 beats team_2)
        let odds = one_on_one::odds(rating_1, rating_2);
        println!("{:?}", odds);
        assert_eq!(odds, 0.7086337899806349);
    }

    #[test]
    fn quality_team_1_advantage() {
        let new_rating = Rating::new();
        let other_rating = Rating {
            mu: 1450.0,
            phi: 200.0,
            sigma: 0.0059,
            is_scaled: false,
        };
        let quality = one_on_one::quality(new_rating, other_rating);
        println!("{:?}", quality);
        assert_eq!(quality, 0.9116055444116669);
    }

    #[test]
    fn quality_team_2_advantage() {
        let new_rating = Rating::new();
        let other_rating = Rating {
            mu: 1450.0,
            phi: 200.0,
            sigma: 0.0059,
            is_scaled: false,
        };
        let quality = one_on_one::quality(other_rating, new_rating);
        println!("{:?}", quality);
        assert_eq!(quality, 0.9116055444116669);
    }
}
