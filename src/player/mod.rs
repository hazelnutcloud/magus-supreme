use benimator::{SpriteSheetAnimation, Play};
use bevy::prelude::*;

// =========================================================
// ==================== PLAYER PLUGIN ======================
// =========================================================

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_startup_system(spawn);
    }
}

// =========================================================
// ===================== COMPONENTS ========================
// =========================================================

#[derive(Component)]
struct Player;

// =========================================================
// ====================== RESOURCES ========================
// =========================================================

struct PlayerAtlas(Handle<TextureAtlas>);
struct PlayerAnimations {
    idle: Handle<SpriteSheetAnimation>
}

// =========================================================
// ======================= SYSTEMS =========================
// =========================================================

fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = server.load("wizard.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(32.), 36, 1);
    let handle = atlases.add(atlas);

    commands.insert_resource(PlayerAtlas(handle));
}

fn spawn(
    mut commands: Commands,
    spritesheet: Res<PlayerAtlas>,
    animations: Res<PlayerAnimations>
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spritesheet.0.clone(),
            ..default()
        })
        .insert(Player)
        .insert(animations.idle.clone())
        .insert(Play);
}

