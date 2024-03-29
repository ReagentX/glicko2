# Glicko2 (Rust Edition)

Glicko2 is an iterative algorithm for ranking opponents or teams in 1v1 games. This is a zero-dependency Rust library implementing this algorithm.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
glicko_2 = "1.0.0"
```

## Sample Usage

The most common usage is to update a series of matches for each team, but this library provides many other convenience methods.

### To update a series of matchups

```rust
use glicko_2::{Rating, Tuning, game::Outcome, algorithm};

/// Tune the rating values, here we use the default
let tuning = Tuning::default();

/// Create a Rating struct for each team
let mut team_to_update = Rating::new(&tuning);
let mut opponent_1 = Rating::new(&tuning);
let mut opponent_2 = Rating::new(&tuning);
let mut opponent_3 = Rating::new(&tuning);
let mut opponent_4 = Rating::new(&tuning);

/// Rate our team against a vector of matchup results
algorithm::rate(
    &mut team_to_update,
    vec![(Outcome::Win, &mut opponent_1),
         (Outcome::Loss, &mut opponent_2),
         (Outcome::Draw, &mut opponent_3),
    ]
);

/// Opponent 4 did not play, so their rating must be decayed
opponent_4.decay();

/// Print our updated rating
println!("{:?}", team_to_update); // { mu: 1500.0, phi: 255.40, sigma: 0.0059, is_scaled: false }
```

### To get the odds one team will beat another

```rust
use glicko_2::{Rating, Tuning, game};

/// Tune the rating values, here we use the default
let tuning = Tuning::default();

/// Create a Rating struct for each team
let mut rating_1 = Rating::new(&tuning);
let mut rating_2 = Rating::new(&tuning);

/// Get odds (percent chance team_1 beats team_2)
let odds = game::odds(&mut rating_1, &mut rating_2);
println!("{}", odds); // 0.5, perfect odds since both teams have the same rating
```

### To determine the quality of a matchup

```rust
use glicko_2::{Rating, Tuning, game};

/// Tune the rating values, here we use the defaults
let tuning = Tuning::default();

/// Create a Rating struct for each team
let mut rating_1 = Rating::new(&tuning);
let mut rating_2 = Rating::new(&tuning);

/// Get odds (the advantage team 1 has over team 2)
let quality = game::quality(&mut rating_1, &mut rating_2);
println!("{}", quality); // 1.0, perfect matchup since both teams have the same rating
```

### To update both team's ratings for a single matchup

```rust
use glicko_2::{Rating, Tuning, game};

/// Tune the rating values, here we use the defaults
let tuning = Tuning::default();

/// Create a Rating struct for each team
let mut rating_1 = Rating::new(&tuning);
let mut rating_2 = Rating::new(&tuning);

/// Update ratings for team_1 beating team_2
game::compete(&mut rating_1, &mut rating_2, false);

/// Print our updated ratings
println!("{:?}", rating_1); // { mu: 1646.47, phi: 307.84, sigma: 0.0059, is_scaled: false }
println!("{:?}", rating_2); // { mu: 1383.42, phi: 306.83, sigma: 0.0059, is_scaled: false }
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

- Difficult to determine the impact of an individual match
- No ratings available in the middle of a rating period
- Ratings are only valid at compute time

## Paper

Mark Glickman developed the Glicko2 algorithm. His paper is available [here](http://www.glicko.net/glicko/glicko2.pdf).
