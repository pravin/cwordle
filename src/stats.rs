use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct GameStats {
    pub games_played: u32,
    pub games_won: u32,
    pub current_streak: u32,
    pub max_streak: u32,
    pub guess_distribution: HashMap<u32, u32>, // guess_count -> number_of_games
}

impl GameStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn win_percentage(&self) -> f32 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.games_won as f32 / self.games_played as f32) * 100.0
        }
    }

    pub fn average_guesses(&self) -> f32 {
        if self.games_won == 0 {
            0.0
        } else {
            let total_guesses: u32 = self.guess_distribution
                .iter()
                .map(|(guesses, count)| guesses * count)
                .sum();
            total_guesses as f32 / self.games_won as f32
        }
    }

    pub fn record_game(&mut self, won: bool, guess_count: Option<usize>) {
        self.games_played += 1;

        if won {
            self.games_won += 1;
            self.current_streak += 1;
            if self.current_streak > self.max_streak {
                self.max_streak = self.current_streak;
            }

            if let Some(guesses) = guess_count {
                *self.guess_distribution.entry(guesses as u32).or_insert(0) += 1;
            }
        } else {
            self.current_streak = 0;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub struct StatsManager {
    stats: GameStats,
    file_path: PathBuf,
}

impl StatsManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = dirs::home_dir().ok_or("Could not find home directory")?;
        path.push(".cwordle_stats.json");

        let stats = if path.exists() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            GameStats::default()
        };

        Ok(Self {
            stats,
            file_path: path,
        })
    }

    pub fn get_stats(&self) -> &GameStats {
        &self.stats
    }

    pub fn record_game(&mut self, won: bool, guess_count: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
        self.stats.record_game(won, guess_count);
        self.save()
    }

    pub fn reset_stats(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stats.reset();
        self.save()
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self.stats)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_stats_default() {
        let stats = GameStats::default();
        assert_eq!(stats.games_played, 0);
        assert_eq!(stats.games_won, 0);
        assert_eq!(stats.current_streak, 0);
        assert_eq!(stats.max_streak, 0);
        assert_eq!(stats.win_percentage(), 0.0);
        assert_eq!(stats.average_guesses(), 0.0);
    }

    #[test]
    fn test_record_win() {
        let mut stats = GameStats::default();
        stats.record_game(true, Some(3));

        assert_eq!(stats.games_played, 1);
        assert_eq!(stats.games_won, 1);
        assert_eq!(stats.current_streak, 1);
        assert_eq!(stats.max_streak, 1);
        assert_eq!(stats.guess_distribution.get(&3), Some(&1));
        assert_eq!(stats.win_percentage(), 100.0);
        assert_eq!(stats.average_guesses(), 3.0);
    }

    #[test]
    fn test_record_loss() {
        let mut stats = GameStats::default();
        stats.record_game(false, None);

        assert_eq!(stats.games_played, 1);
        assert_eq!(stats.games_won, 0);
        assert_eq!(stats.current_streak, 0);
        assert_eq!(stats.max_streak, 0);
        assert_eq!(stats.win_percentage(), 0.0);
    }

    #[test]
    fn test_win_streak() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(3));
        assert_eq!(stats.current_streak, 1);
        assert_eq!(stats.max_streak, 1);

        stats.record_game(true, Some(4));
        assert_eq!(stats.current_streak, 2);
        assert_eq!(stats.max_streak, 2);

        stats.record_game(true, Some(2));
        assert_eq!(stats.current_streak, 3);
        assert_eq!(stats.max_streak, 3);
    }

    #[test]
    fn test_streak_broken_by_loss() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));
        assert_eq!(stats.current_streak, 2);
        assert_eq!(stats.max_streak, 2);

        stats.record_game(false, None);
        assert_eq!(stats.current_streak, 0);
        assert_eq!(stats.max_streak, 2);
    }

    #[test]
    fn test_max_streak_tracking() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));
        assert_eq!(stats.max_streak, 2);

        stats.record_game(false, None);

        stats.record_game(true, Some(2));
        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));

        assert_eq!(stats.current_streak, 3);
        assert_eq!(stats.max_streak, 3);
    }

    #[test]
    fn test_guess_distribution() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(1));
        stats.record_game(true, Some(2));
        stats.record_game(true, Some(2));
        stats.record_game(true, Some(3));
        stats.record_game(true, Some(3));
        stats.record_game(true, Some(3));

        assert_eq!(stats.guess_distribution.get(&1), Some(&1));
        assert_eq!(stats.guess_distribution.get(&2), Some(&2));
        assert_eq!(stats.guess_distribution.get(&3), Some(&3));
        assert_eq!(stats.guess_distribution.get(&4), None);
    }

    #[test]
    fn test_win_percentage() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));
        stats.record_game(false, None);
        stats.record_game(false, None);

        assert_eq!(stats.games_played, 4);
        assert_eq!(stats.games_won, 2);
        assert_eq!(stats.win_percentage(), 50.0);
    }

    #[test]
    fn test_average_guesses() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(2));
        stats.record_game(true, Some(4));
        stats.record_game(true, Some(6));

        assert_eq!(stats.average_guesses(), 4.0);
    }

    #[test]
    fn test_reset_stats() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));

        assert_eq!(stats.games_played, 2);

        stats.reset();

        assert_eq!(stats.games_played, 0);
        assert_eq!(stats.games_won, 0);
        assert_eq!(stats.current_streak, 0);
        assert_eq!(stats.max_streak, 0);
        assert!(stats.guess_distribution.is_empty());
    }

    #[test]
    fn test_complex_game_scenario() {
        let mut stats = GameStats::default();

        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));
        stats.record_game(false, None);
        stats.record_game(true, Some(2));
        stats.record_game(true, Some(5));
        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));
        stats.record_game(false, None);
        stats.record_game(true, Some(6));
        stats.record_game(true, Some(3));

        assert_eq!(stats.games_played, 10);
        assert_eq!(stats.games_won, 8);
        assert_eq!(stats.current_streak, 2);
        assert_eq!(stats.max_streak, 4);
        assert_eq!(stats.win_percentage(), 80.0);

        assert_eq!(stats.guess_distribution.get(&2), Some(&1));
        assert_eq!(stats.guess_distribution.get(&3), Some(&3));
        assert_eq!(stats.guess_distribution.get(&4), Some(&2));
        assert_eq!(stats.guess_distribution.get(&5), Some(&1));
        assert_eq!(stats.guess_distribution.get(&6), Some(&1));
    }
}