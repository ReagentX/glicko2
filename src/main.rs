mod glicko2;

use crate::glicko2::algorithm;
use crate::glicko2::constants::{EPSILON, MU, PHI, Q, RATIO, SIGMA, TAU};
use crate::glicko2::rating;
use crate::glicko2::rating::match_result;

fn main() {
    // Type tests
    println!("Type\tVal");

    let win = match_result::Status::Win;
    let win_val = match_result::val(&win);
    println!("{:?}\t{:?}", win, win_val);

    let draw = match_result::Status::Draw;
    let draw_val = match_result::val(&draw);
    println!("{:?}\t{:?}", draw, draw_val);

    let loss = match_result::Status::Loss;
    let loss_val = match_result::val(&loss);
    println!("{:?}\t{:?}", loss, loss_val);
    println!("-----");

    // Constant Tests
    println!("Mu\t{}", MU);
    println!("Phi\t{}", PHI);
    println!("Sigma\t{}", SIGMA);
    println!("Tau\t{}", TAU);
    println!("Epsilon\t{}", EPSILON);
    println!("Q\t{}", Q);
    println!("RATIO\t{}", RATIO);
    println!("-----");

    // Create rating
    let mut new_rating = rating::make_rating();
    println!("{:?}", new_rating);
    new_rating.scale_up();
    println!("{:?}", new_rating);
    new_rating.scale_down();
    println!("{:?}", new_rating);
    new_rating.decay();
    println!("{:?}", new_rating);
    println!("-----");

    // Test expect
    let mut other_rating = rating::Rating {
        mu: 1450.0,
        phi: 200.0,
        sigma: 0.0059,
        is_scaled: false,
    };
    new_rating.scale_down();
    other_rating.scale_down();
    let impact = algorithm::reduce_impact(&new_rating, &other_rating);
    println!("{:?}", impact);
    println!(
        "Expected Outcome Favors t1 {:.2}%",
        algorithm::expect_score(&new_rating, &other_rating, impact) * 100.0
    );
    new_rating.scale_up();
    other_rating.scale_up();
    println!("-----");

    // Test rate
    println!("Previous ratings");
    println!("{:?}", new_rating);
    println!("{:?}", other_rating);
    algorithm::rate(
        &mut new_rating,
        vec![(match_result::Status::Win, &mut other_rating)],
    );
    println!("Reranked ratings");
    println!("{:?}", new_rating);
    println!("{:?}", other_rating);
    println!("-----");

}
