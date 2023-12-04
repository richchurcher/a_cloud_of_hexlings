use bevy::prelude::*;

pub mod collision;
pub mod debug;
pub mod map;
pub mod menu;
pub mod movement;
pub mod pause_menu;
pub mod player;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Paused,
    Playing,
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
