use crate::resources::{Animations, SpriteSheets};
use crate::SystemLabels;
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use std::time::Duration;

// =========================================================
// ===================== LOAD ASSETS =======================
// =========================================================

pub struct Loader;

impl Plugin for Loader {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set_to_stage(
            StartupStage::PreStartup,
            SystemSet::new()
                .with_system(load_sprite_sheets)
                .with_system(load_animations)
                .label(SystemLabels::Load),
        );
    }
}

fn load_sprite_sheets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let wizard = server.load("wizard.png");
    let wizard_atlas = TextureAtlas::from_grid(wizard, Vec2::splat(32.), 36, 1);
    let wizard_handle = textures.add(wizard_atlas);

    commands.insert_resource(SpriteSheets {
        wizard: wizard_handle,
    });
}

fn load_animations(mut commands: Commands, mut animations: ResMut<Assets<SpriteSheetAnimation>>) {
    let wizard_idle = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(150),
    ));

    commands.insert_resource(Animations { wizard_idle });
}
