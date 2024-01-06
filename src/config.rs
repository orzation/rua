#![allow(dead_code)]

use std::env;

/// The basic configs for whole program, just some keymaps for now.
#[derive(Debug)]
pub struct GlobleConfig {
    pub up_key: char,
    pub down_key: char,
    pub left_key: char,
    pub right_key: char,
    pub mine_key: char,
    pub flag_key: char,
    pub quit_key: char,
}

fn env_or_into_char(key: &str, default: &str) -> char {
    env::var(key)
        .unwrap_or_else(|_| default.to_string())
        .chars()
        .next()
        .unwrap()
}

pub fn globle_config_from_env() -> GlobleConfig {
    return GlobleConfig {
        up_key: env_or_into_char("UP_KEY", "k"),
        down_key: env_or_into_char("DOWN_KEY", "j"),
        left_key: env_or_into_char("LEFT_KEY", "h"),
        right_key: env_or_into_char("RIGHT_KEY", "l"),
        mine_key: env_or_into_char("MINE_KEY", " "),
        flag_key: env_or_into_char("FLAG_KEY", "f"),
        quit_key: env_or_into_char("QUIT_KEY", "q"),
    };
}

/// The configs of game mode.
#[derive(Debug)]
pub struct GameConfig {
    pub height: usize,
    pub width: usize,
    pub bomb: usize,
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

impl GameMode {
    pub fn from_usize(num: usize) -> GameMode {
        match num {
            0 => GameMode::Simple,
            1 => GameMode::Normal,
            _ => GameMode::Hard,
        }
    }
}

impl GameConfig {
    /// new_game_config is used to make a game mode config.
    pub fn from_game_mode(mode: GameMode) -> GameConfig {
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
}

impl ToString for GameMode {
    fn to_string(&self) -> String {
        match self {
            GameMode::Simple => "Simple",
            GameMode::Normal => "Normal",
            GameMode::Hard => "Hard",
        }
        .to_string()
    }
}
