use bevy::prelude::*;

pub mod assets;
pub mod camera;
pub mod collision;
pub mod debug;
pub mod enemy;
pub mod hexling;
pub mod map;
pub mod menu;
pub mod movement;
pub mod over_menu;
pub mod pause_menu;
pub mod player;
pub mod reset;
pub mod sound;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Over,
    Paused,
    Playing,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    #[default]
    One,
    Two,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
