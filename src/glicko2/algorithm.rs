/*!
The math behind the Glicko2 algorithm
*/
use crate::glicko2::{constants::EPSILON, game::Outcome, rating::Rating};

/// This function reduces the impact of games as a function of an opponent's rating deviation.
pub(crate) fn reduce_impact(rating: &Rating, other_rating: &Rating) -> f64 {
    // Must be called for scaled ratings
    if !rating.is_scaled || !other_rating.is_scaled {
        panic!("Unscaled ratings passed to reduce impact!");
    }
    let phi = rating.phi.powi(2) + other_rating.phi.powi(2);
    let phi_sqrt = phi.sqrt();
    let pi_2 = std::f64::consts::PI.powi(2);
    let denominator = 1.0 + (3.0 * phi_sqrt.powi(2)) / pi_2;
    1.0 / denominator.sqrt()
}

/// The expected outcome of a game given two sets of ratings.
pub(crate) fn expect_score(rating: &Rating, other_rating: &Rating, impact: f64) -> f64 {
    if !rating.is_scaled || !other_rating.is_scaled {
        panic!("Unscaled ratings passed to expect score!");
    }
    let new_impact = -impact * (rating.mu - other_rating.mu);
    1.0 / (1.0 + new_impact.exp())
}

/// Determine the new value for volatility given a set of ratings.
fn determine_sigma(rating: &Rating, difference: &f64, variance: &f64) -> f64 {
    let phi = rating.phi;
    let diff_squared = difference.powi(2);
    // 1. Let a = ln(sigma^2)
    let alpha = rating.sigma.powi(2).ln();

    // Define optimality criterion as a closure so we do not pass references for the above
    let optimality_criterion = |x: f64| -> f64 {
        let tmp = phi.powi(2) + variance + x.exp();
        let tmp_2 = 2.0 * tmp.powi(2);
        let a = x.exp() * (diff_squared - tmp) / tmp_2;
        let b = (x - alpha) / rating.tuning.tau.powi(2);
        a - b
    };

    // 2. Set the initial value for the iterative algorithm
    let mut a = alpha;
    let mut b = if diff_squared > (phi.powi(2) + variance) {
        (diff_squared - phi.powi(2) - variance).ln()
    } else {
        let mut k = 1.0;
        while optimality_criterion(alpha - k * rating.tuning.tau) < 0.0 {
            k += 1.0;
        }
        alpha - k * rating.tuning.tau
    };

    // 3. Let fA = optimality_criterion(A) and f(B) = optimality_criterion(B)
    let mut f_a = optimality_criterion(a);
    let mut f_b = optimality_criterion(b);

    // 4. While |B-A| > e, carry out the following steps:
    // (a) Let C = A + (A - B)fA / (fB-fA), and let fC = f(C).
    // (b) If fCfB < 0, then set A <- B and fA <- fB; otherwise, just set
    //     fA <- fA/2.
    // (c) Set B <- C and fB <- fC.
    // (d) Stop if |B-A| <= e. Repeat the above three steps otherwise.
    while (b - a).abs() > EPSILON {
        let c = a + (a - b) * f_a / (f_b - f_a);
        let f_c = optimality_criterion(c);
        if f_c * f_b < 0.0 {
            a = b;
            f_a = f_b;
        } else {
            f_a /= 2.0;
        }
        b = c;
        f_b = f_c;
    }

    // 5. Once |B-A| <= e, set s' <- e^(A/2)
    1.0f64.exp().powf(a / 2.0)
}

/// Given a team and a set of outcomes in a period, update the team's ratings.
/// Because this modifies the rating of the team in-place, you may want to pass a copy
/// if you wish to preserve old ratings.
///
/// # Example
///
/// ```
/// use glicko_2::{Rating, Tuning, game::Outcome};
///
/// let tuning = Tuning::default();
/// 
/// let mut team_to_update = Rating::new(&tuning);
/// let mut opponent_1 = Rating::new(&tuning);
/// let mut opponent_2 = Rating::new(&tuning);
/// let mut opponent_3 = Rating::new(&tuning);
///
/// glicko_2::algorithm::rate(
///     &mut team_to_update,
///     vec![(Outcome::Win, &mut opponent_1),
///          (Outcome::Loss, &mut opponent_2),
///          (Outcome::Draw, &mut opponent_3),
///      ]
/// );
/// ```
pub fn rate(rating: &mut Rating, outcomes: Vec<(Outcome, &mut Rating)>) {
    // Outcome is a list of outcomes for a set of games between two teams, i.e.
    //   a vector tuples like [(WIN, rating2), ...]

    // Step 2. For each player, convert the rating and rating deviation onto the
    //         Glicko-2 scale.
    rating.scale_down();

    // Step 3. Compute the quantity v. This is the estimated variance of the
    //         team's/player's rating based only on game outcomes.
    // Step 4. Compute the quantity difference, the estimated improvement in
    //         rating by comparing the pre-period rating to the performance
    //         rating based only on game outcomes.
    let mut variance_inv = 0.0;
    let mut difference = 0.0;

    for (score, other_rating) in outcomes {
        other_rating.scale_down();
        let impact = reduce_impact(rating, other_rating);
        let expected = expect_score(rating, other_rating, impact);
        let expected_inv = expected * (1.0 - expected);
        variance_inv += impact.powi(2) * expected_inv;
        difference += impact * (score.val() - expected);
        other_rating.scale_up();
    }

    difference /= variance_inv.max(0.0001);
    let variance = 1.0 / variance_inv;

    // Step 5. Determine the new value, Sigma', or the sigma. This
    //         computation requires iteration.
    let sigma = determine_sigma(rating, &difference, &variance);

    // Step 6. Update the rating deviation to the new pre-rating period
    //         value, Phi*.
    let phi_star = (rating.phi.powi(2) + sigma.powi(2)).sqrt();

    // Step 7. Update the rating and rating deviation to the new values, Mu' and Phi'.
    let phi = 1.0 / ((1.0 / phi_star).powi(2) + (1.0 / variance)).sqrt();
    let mu = (rating.mu + phi).powi(2) * (difference / variance);

    // Step 8. Convert rating and rating deviation back to original scale.
    rating.mu = mu;
    rating.phi = phi;
    rating.sigma = sigma;
    rating.scale_up(); // Since this is a reference, we can just scale it back
}
