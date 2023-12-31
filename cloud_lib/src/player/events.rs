use bevy::prelude::*;

// Charge: player is sending hexlings away from themselves.
#[derive(Event)]
pub struct ChargeEvent(pub Entity);

// Recall: player is calling the hexlings home.
#[derive(Event)]
pub struct RecallEvent(pub Entity);

// SpawnHexling: does what it says on the tin.
#[derive(Event)]
pub struct SpawnHexlingEvent(pub Entity);
