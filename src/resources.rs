use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    DifficultySelect,
    Running,
    Paused,
    GameOver,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Hard,
}

#[derive(Resource)]
pub struct WordList(pub Vec<String>);

#[derive(Resource)]
pub struct Wave {
    pub current: usize,
    pub enemies_remaining: usize,
    pub timer: Timer,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            current: 1,
            enemies_remaining: 5,
            timer: Timer::from_seconds(30.0, TimerMode::Repeating),
        }
    }
}

impl Wave {
    pub fn get_enemy_count(&self) -> usize {
        let primes = [5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        if self.current <= primes.len() {
            primes[self.current - 1]
        } else {
            primes.last().unwrap() + (self.current - primes.len()) * 2
        }
    }
}

impl Default for WordList {
    fn default() -> Self {
        Self(vec![
            "rust".to_string(), "bevy".to_string(), "game".to_string(), "code".to_string(),
            "type".to_string(), "fast".to_string(), "ship".to_string(), "wave".to_string(),
            "grid".to_string(), "neon".to_string(), "laser".to_string(), "blade".to_string(),
        ])
    }
}

impl WordList {
    pub fn get_word(&self, difficulty: Difficulty) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let base_word = &self.0[rng.gen_range(0..self.0.len())];
        
        match difficulty {
            Difficulty::Easy => base_word.to_lowercase(),
            Difficulty::Hard => {
                base_word.chars().map(|c| {
                    if rng.gen_bool(0.5) {
                        c.to_uppercase().next().unwrap()
                    } else {
                        c
                    }
                }).collect()
            }
        }
    }
}
