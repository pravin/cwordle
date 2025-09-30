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