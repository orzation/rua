#![allow(dead_code)]

/// The basic configs for whole program, just some keymaps for now.
#[derive(Debug)]
struct GlobleConfig {
    up_key: String,
    down_key: String,
    left_key: String,
    right_key: String,
    mine_key: String,
    flag_key: String,
}

/// The configs of game mode.
#[derive(Debug)]
pub struct GameConfig {
    height: usize,
    width: usize,
    bomb: usize,
}

impl GameConfig {
    pub fn get_size(&self) -> usize {
        self.height * self.width
    }
}

pub enum GameMode {
    Simple,
    Normal,
    Hard,
}

/// new_game_config is used to make a game mode config.
pub fn new_game_config(mode: GameMode) -> GameConfig {
    match mode {
        GameMode::Simple => GameConfig {
            height: 9,
            width: 9,
            bomb: 10,
        },
        GameMode::Normal => GameConfig {
            height: 16,
            width: 16,
            bomb: 40,
        },
        GameMode::Hard => GameConfig {
            height: 16,
            width: 30,
            bomb: 99,
        },
    }
}
