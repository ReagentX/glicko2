/*! 
Provides functions to handle a single one on one game and update ratings accordingly
*/
use crate::glicko2::algorithm;
use crate::glicko2::rating::{Rating, Status};

/// Updates ratings for two teams
/// If the game was a draw, pass `drawn` as `true`.
///
/// # Example
///
/// ```
/// let mut rating_1 = glicko2::rating::Rating::new();
/// let mut rating_2 = glicko2::rating::Rating::new();
/// let (rating_1, rating_2) = glicko2::game::compete(rating_1, rating_2, false);
/// ```
pub fn compete(mut winner: Rating, mut loser: Rating, drawn: bool) -> (Rating, Rating) {
    // drawn is false if Team 1 beat Team 2
    if drawn {
        algorithm::rate(&mut winner, vec![(Status::Draw, &mut loser)]);
        algorithm::rate(&mut loser, vec![(Status::Draw, &mut winner)]);
    } else {
        algorithm::rate(&mut winner, vec![(Status::Win, &mut loser)]);
        algorithm::rate(&mut loser, vec![(Status::Loss, &mut winner)]);
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
/// let odds = glicko2::game::odds(rating_1, rating_2);
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
/// let quality = glicko2::game::quality(rating_1, rating_2);
/// ```
pub fn quality(rating1: Rating, rating2: Rating) -> f64 {
    // 1.0 if perfect match
    let expected_score_1 = odds(rating1, rating2);
    let expected_score_2 = odds(rating2, rating1);
    let advantage = expected_score_1 - expected_score_2; // Advantage team 1 has over team 2
    1.0 - advantage.abs()
}
