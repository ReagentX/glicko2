# Glicko2 (Rust Edition)

Glicko2 is an iterative algorithm for ranking opponents or teams in 1v1 games. This is a zero-dependency Rust library implementing this algorithm.

## Sample Usage

To update a single matchup:

```rust
use glicko2::rating::{Rating, one_on_one};

/// Create a Rating stuct for each team
let rating_1 = Rating::new();
let rating_2 = Rating::new();

/// Update ratings for team_1 beating team_2
let (rating_1, rating_2) = one_on_one::rate(rating_1, rating_2, false);

/// Get odds (percent chance team_1 beats team_2)
let odds = one_on_one::odds(rating_1, rating_2);
println!("{}", odds); /// 0.7086337899806349
```

To update a series of matchups:

```rust
use glicko2::{rating::{Rating, match_result}, algorithm};

/// Create a Rating stuct for each team
let mut team_to_update = Rating::new();
let mut opponent_1 = Rating::new();
let mut opponent_2 = Rating::new();
let mut opponent_3 = Rating::new();

/// Rate our team against a vector of matchup results
algorithm::rate(
    &mut team_to_update,
    vec![(match_result::Status::Win, &mut opponent_1),
         (match_result::Status::Loss, &mut opponent_2),
         (match_result::Status::Draw, &mut opponent_3),
    ]
);

/// Print our updated rating
println!("{:?}", team_to_update); /// { mu: 1500.0, phi: 255.40, sigma: 0.0059, is_scaled: false }
```

## Rating

Each side of a 1v1 competition is assigned a rating and a rating deviation. The rating represents the skill of a player or team, and the rating deviation measures confidence in the rating value.

### Rating Deviation

A team or player's rating deviation decreases with results and increases during periods of inactivity. Rating deviation also depends on volatility, or how consistent a player or team's performance is.

Thus, a confidence interval represents a team's or player's skill: a player with a rating of `1300` and a rating deviation of `25` means the player's real strength lies between `1350` and `1250` with 95% confidence.

### Match Timing Caveat

Since time is a factor in rating deviation, the algorithm assumes all matches within a rating period were played concurrently and use the same values for uncertainty.

## Tuning Parameters

- Rating period length and quantity impact decay in rating deviation
  - Should generally be `{10..15}` matches per team per period
- Initial mu and phi values affect how much teams or players can change
  - Defaults are `1500` and `350` respectively
- Sigma is the base volatility
  - Default to `0.06`
- Tau is the base change constraint; higher means increased weight given to upsets
  - Should be `{0.3..1.2}`

## Problems

- Difficult to determine the impact an individual match has
- No ratings available in the middle of a rating period
- Ratings are only valid at compute time

## Paper

Mark Glickman developed the Glicko2 algorithm. His paper is available [here](http://www.glicko.net/glicko/glicko2.pdf).
