use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Settings,
    DifficultySelect,
    Running,
    BossWarning,
    Paused,
    GameOver,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Hard,
}

#[derive(Resource, Debug, Clone)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 1.0,
            music_volume: 1.0,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MenuSelection {
    pub selected_index: usize,
    pub item_count: usize,
}

impl MenuSelection {
    pub fn new(item_count: usize) -> Self {
        Self {
            selected_index: 0,
            item_count,
        }
    }
    
    pub fn move_up(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.item_count - 1;
        } else {
            self.selected_index -= 1;
        }
    }
    
    pub fn move_down(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.item_count;
    }
}

#[derive(Resource)]
pub struct ParagraphContent {
    pub lines: Vec<String>,
    pub unique_words: Vec<String>,
}

#[derive(Resource)]
pub struct ContentManager {
    pub paragraphs: Vec<ParagraphContent>,
    pub current_index: usize,
}

#[derive(Resource, Default, Debug)]
pub struct ShouldResetOnStart(pub bool);

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

impl Default for ContentManager {
    fn default() -> Self {
        Self::load_from_files()
    }
}

impl ContentManager {
    pub fn load_from_files() -> Self {
        use std::fs;
        use std::collections::HashSet;
        
        let mut paragraphs = Vec::new();
        
        let content_dir = "content";
        if let Ok(entries) = fs::read_dir(content_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    let lines: Vec<String> = content
                        .lines()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    
                    let mut word_set = HashSet::new();
                    for line in &lines {
                        for word in line.split_whitespace() {
                            let clean_word: String = word
                                .chars()
                                .filter(|c| c.is_alphanumeric())
                                .collect();
                            if !clean_word.is_empty() {
                                word_set.insert(clean_word);
                            }
                        }
                    }
                    
                    let unique_words: Vec<String> = word_set.into_iter().collect();
                    
                    paragraphs.push(ParagraphContent {
                        lines,
                        unique_words,
                    });
                }
            }
        }
        
        if paragraphs.is_empty() {
            println!("Warning: No content files found, using default content");
            let default_lines = vec![
                "The quick brown fox jumps over the lazy dog".to_string(),
                "Programming in Rust is fun and safe".to_string(),
                "Bevy makes game development easy".to_string(),
            ];
            
            let mut word_set = HashSet::new();
            for line in &default_lines {
                for word in line.split_whitespace() {
                    let clean_word: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
                    if !clean_word.is_empty() {
                        word_set.insert(clean_word);
                    }
                }
            }
            
            paragraphs.push(ParagraphContent {
                lines: default_lines,
                unique_words: word_set.into_iter().collect(),
            });
        }
        
        println!("Loaded {} paragraphs", paragraphs.len());
        
        Self {
            paragraphs,
            current_index: 0,
        }
    }
    
    pub fn get_word(&self, difficulty: Difficulty) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if self.paragraphs.is_empty() {
            return "error".to_string();
        }
        
        let current_paragraph = &self.paragraphs[self.current_index];
        if current_paragraph.unique_words.is_empty() {
            return "empty".to_string();
        }
        
        let base_word = &current_paragraph.unique_words[rng.gen_range(0..current_paragraph.unique_words.len())];
        
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
    
    pub fn get_current_lines(&self) -> Vec<String> {
        if self.paragraphs.is_empty() {
            return vec!["No content loaded".to_string()];
        }
        
        self.paragraphs[self.current_index].lines.clone()
    }
    
    pub fn next_paragraph(&mut self) {
        self.current_index = (self.current_index + 1) % self.paragraphs.len();
        println!("Switched to paragraph {}", self.current_index + 1);
    }
}
