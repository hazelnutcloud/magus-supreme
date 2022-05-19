use crate::components::{Name, Player, Velocity};
use crate::resources::{Animations, SpriteSheets};
use crate::SystemLabels;
use benimator::Play;
use bevy::prelude::*;

// =========================================================
// ======================== SETUP ==========================
// =========================================================

pub struct Setup;

impl Plugin for Setup {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_set(
            SystemSet::new()
                .with_system(spawn_camera)
                .with_system(spawn_player)
                .label(SystemLabels::Spawn),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    spritesheets: Res<SpriteSheets>,
    animations: Res<Animations>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spritesheets.wizard.clone(),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..default()
        })
        .insert(Player)
        .insert(Name("Player".into()))
        .insert(Velocity(Vec2::ZERO))
        .insert(animations.wizard_idle.clone())
        .insert(Play);
}
