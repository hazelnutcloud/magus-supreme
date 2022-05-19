use bevy::prelude::*;

// =========================================================
// ===================== COMPONENTS ========================
// =========================================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Velocity(pub Vec2);
