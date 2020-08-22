#![allow(dead_code)]
mod glicko2;

#[cfg(test)]
mod test {
    use crate::glicko2::algorithm;
    use crate::glicko2::constants::{EPSILON, MU, PHI, Q, RATIO, SIGMA, TAU};
    use crate::glicko2::rating;
    use crate::glicko2::rating::match_result;

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
        let new_rating = rating::make_rating();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 1500.0);
        assert_eq!(new_rating.phi, 350.0);
        assert_eq!(new_rating.sigma, 0.006);
        assert_eq!(new_rating.is_scaled, false);
    }

    #[test]
    fn scale_down() {
        let mut new_rating = rating::make_rating();
        new_rating.scale_down();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 0.0);
        assert_eq!(new_rating.phi, 2.014761872416068);
        assert_eq!(new_rating.sigma, 0.006);
        assert_eq!(new_rating.is_scaled, true);
    }

    #[test]
    fn scale_down_already_down() {
        let mut new_rating = rating::make_rating();
        new_rating.scale_down();
        new_rating.scale_down();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 0.0);
        assert_eq!(new_rating.phi, 2.014761872416068);
        assert_eq!(new_rating.sigma, 0.006);
        assert_eq!(new_rating.is_scaled, true);
    }

    #[test]
    fn scale_up() {
        let mut new_rating = rating::make_rating();
        new_rating.scale_down();
        new_rating.scale_up();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 1500.0);
        assert_eq!(new_rating.phi, 350.0);
        assert_eq!(new_rating.sigma, 0.006);
        assert_eq!(new_rating.is_scaled, false);
    }

    #[test]
    fn scale_up_already_up() {
        let mut new_rating = rating::make_rating();
        new_rating.scale_up();
        println!("{:?}", new_rating);
        assert_eq!(new_rating.mu, 1500.0);
        assert_eq!(new_rating.phi, 350.0);
        assert_eq!(new_rating.sigma, 0.006);
        assert_eq!(new_rating.is_scaled, false);
    }

    #[test]
    fn reduce_impact() {
        let mut new_rating = rating::make_rating();
        let mut other_rating = rating::Rating {
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
        let new_rating = rating::make_rating();
        let other_rating = rating::Rating {
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
        let mut new_rating = rating::make_rating();
        let mut other_rating = rating::Rating {
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
        assert_eq!(new_rating.is_scaled, false);
    }
}
