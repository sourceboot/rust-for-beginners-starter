//! Lab 01 — the game announces itself.
//!
//! The three constants below are finished — read them, they are the game's
//! shape, and every later lab takes them as arguments instead of assuming
//! them. The two functions are yours to write; the project text walks you
//! through both, and your tests go in a `mod tests` at the bottom.

/// The game's name, in one place.
pub const GAME_NAME: &str = "lantern";

/// How long the secret word is.
pub const WORD_LEN: usize = 5;

/// How many guesses a player gets.
pub const MAX_GUESSES: usize = 6;
