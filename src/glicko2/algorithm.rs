use crate::glicko2::constants::{Q, TAU, EPSILON};
use crate::glicko2::rating::match_result::val;
use crate::glicko2::rating::match_result::Status;
use crate::glicko2::rating::Rating;

pub fn optimality_criterion(
    &x: &f64,
    &phi: &f64,
    &variance: &f64,
    &difference_squared: &f64,
    &alpha: &f64
) -> f64 {
    let tmp = phi.powi(2) + variance + x.exp();
    let tmp_2 = 2.0 * tmp.powi(2);
    let a = x.exp() * (difference_squared - tmp) / tmp_2;
    let b = (x - alpha) / TAU.powi(2);
    a - b
}

pub fn reduce_impact(rating: &Rating, other_rating: &Rating) -> f64 {
    // Must be called for scaled ratings
    if !rating.is_scaled || !other_rating.is_scaled {
        panic!("Scaled ratings passed to reduce impact!");
    }
    let phi = rating.phi.powi(2) + other_rating.phi.powi(2);
    let phi_sqrt = phi.sqrt();
    let pi_2 = std::f64::consts::PI.powi(2);
    let denominator = 1.0 + (3.0 * phi_sqrt.powi(2)) / pi_2;
    1.0 / denominator.sqrt()
}

pub fn expect_score(rating: &Rating, other_rating: &Rating, impact: f64) -> f64 {
    if !rating.is_scaled || !other_rating.is_scaled {
        panic!("Scaled ratings passed to expect score!");
    }
    let new_impact = -impact * (rating.mu - other_rating.mu);
    1.0 / (1.0 + new_impact.exp())
}

pub fn determine_sigma(rating: &Rating, difference: &f64, variance: &f64) -> f64 {
    let phi = rating.phi;
    let diff_squared = difference.powi(2);
    // 1. Let a = ln(sigma^2)
    let alpha = rating.sigma.powi(2).ln();

    // 2. Set the initial values for the iterative algorithm.
    let mut a = alpha;
    let mut b: f64;
    if diff_squared > { phi.powi(2) + variance } {
        b = { diff_squared - phi.powi(2) - variance }.ln();
    }
    else {
        let mut k = 1.0;
        while optimality_criterion(&{ alpha - k * TAU }, &phi, &variance, &diff_squared, &alpha) < 0.0 {
            k += 1.0;
        }
        b = alpha - k * TAU;
    }

    // 3. Let fA = optimality_criterion(A) and f(B) = optimality_criterion(B)
    let mut f_a = optimality_criterion(&a, &phi, &variance, &diff_squared, &alpha);
    let mut f_b = optimality_criterion(&b, &phi, &variance, &diff_squared, &alpha);

    // 4. While |B-A| > e, carry out the following steps:
    // (a) Let C = A + (A - B)fA / (fB-fA), and let fC = f(C).
    // (b) If fCfB < 0, then set A <- B and fA <- fB; otherwise, just set
    //     fA <- fA/2.
    // (c) Set B <- C and fB <- fC.
    // (d) Stop if |B-A| <= e. Repeat the above three steps otherwise.
    while { b - a }.abs() > EPSILON {
        let c = a + (a - b) * f_a / (f_b - f_a);
        let f_c = optimality_criterion(&c, &phi, &variance, &diff_squared, &alpha);
        if f_c * f_b < 0.0 {
            a = b;
            f_a = f_b;
        }
        else {
            f_a /= 2.0;
        }
        b = c;
        f_b = f_c;
    }

    // 5. Once |B-A| <= e, set s' <- e^(A/2)
    1.0f64.exp().powf(a / 2.0)
}

pub fn rate(rating: &mut Rating, outcomes: Vec<(Status, &mut Rating)>) {
    // Outcome is a list of outcomes for a set of games between two teams, i.e.
    //   a vector tuples like [(WIN, rating2), ...]

    // Step 2. For each player, convert the rating and RD's onto the
    //         Glicko-2 scale.
    rating.scale_down();

    // Step 3. Compute the quantity v. This is the estimated variance of the
    //         team's/player's rating based only on game outcomes.
    // Step 4. Compute the quantity difference, the estimated improvement in
    //         rating by comparing the pre-period rating to the performance
    //         rating based only on game outcomes.
    let mut d_square_inv = 0.0;
    let mut variance_inv = 0.0;
    let mut difference = 0.0;

    for (score, other_rating) in outcomes {
        other_rating.scale_down();
        let impact = reduce_impact(&rating, &other_rating);
        let expected = expect_score(&rating, &other_rating, impact);
        let expected_inv = expected * (1.0 - expected);
        variance_inv += impact.powi(2) * expected_inv;
        difference += impact * (val(&score) - expected);
        d_square_inv += expected_inv * (Q.powi(2) * impact.powi(2));
        other_rating.scale_up();
    }

    difference /= variance_inv.max(0.0001);
    let variance = 1.0 / variance_inv;
    let denom = rating.phi.powi(-2) + d_square_inv;
    // let mut mu = rating.mu + Q / denom * (difference / variance_inv);
    let mut phi = { 1.0 / denom }.sqrt();

    // Step 5. Determine the new value, Sigma', ot the sigma. This
    //         computation requires iteration.
    let sigma = determine_sigma(&rating, &difference, &variance);

    // Step 6. Update the rating deviation to the new pre-rating period
    //         value, Phi*.
    let phi_star = { phi.powi(2) + sigma.powi(2) }.sqrt();

    // Step 7. Update the rating and RD to the new values, Mu' and Phi'.
    phi = 1.0 / { { 1.0 / phi_star }.powi(2) + { 1.0 / variance } }.sqrt();
    let mu = { rating.mu + phi }.powi(2) * (difference / variance);

    // Step 8. Convert ratings and RD's back to original scale.
    rating.mu = mu;
    rating.phi = phi;
    rating.sigma = sigma;
    rating.scale_up();  // Since this is a reference, we can just scale it back
}
