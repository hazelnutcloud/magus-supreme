use bevy::{prelude::*, input::mouse::MouseMotion};
use heron::{RigidBody, Velocity};

use crate::{player::{SPAWN_POINT, Player}, camera::ZOOM};

// =========================================================
// ==================== CURSOR PLUGIN ======================
// =========================================================

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite)
            .add_startup_system(spawn)
            .add_system(move_cursor_parent)
            .add_system(move_cursor);
    }
}

// =========================================================
// ===================== COMPONENTS ========================
// =========================================================

#[derive(Component, Default)]
pub struct Cursor;
#[derive(Component, Default)]
pub struct CursorParent;

// =========================================================
// ====================== RESOURCES ========================
// =========================================================
pub struct CursorSprite(Handle<Image>);


// ------ load atlas ---------
fn load_sprite(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let image = server.load("spritesheets/reticle.png");

    commands.insert_resource(CursorSprite(image));
}

// ----- spawn cursor --------
fn spawn(
    mut commands: Commands,
    sprite: Res<CursorSprite>
) {
    let cursor = commands.spawn_bundle(SpriteBundle {
        texture: sprite.0.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Cursor::default())
        .id();

    commands.spawn()
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(SPAWN_POINT[0], SPAWN_POINT[1], 998.)))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(CursorParent::default())
        .add_child(cursor);
}

// -- move cursor parent -----
fn move_cursor_parent(
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut cursor_parent_query: Query<&mut Transform, (Without<Player>, With<CursorParent>)>
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut cursor_parent_transform = cursor_parent_query.single_mut();

        cursor_parent_transform.translation.x = player_transform.translation.x;
        cursor_parent_transform.translation.y = player_transform.translation.y;
    }
}

// ------ move cursor --------
fn move_cursor(
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    mut cursor_evr: EventReader<MouseMotion>,
    windows: Res<Windows>,
) {
    for ev in cursor_evr.iter() {
        let mut cursor_transform = cursor_query.single_mut();

        cursor_transform.translation.x += ev.delta.x / 3.;
        cursor_transform.translation.y += -ev.delta.y / 3.;

        if let Some(window) = windows.get_primary() {
            cursor_transform.translation.x = cursor_transform.translation.x.clamp(
                - window.width() / (2. * ZOOM) + 0.1 * cursor_transform.translation.x,
                window.width() / (2. * ZOOM) + 0.1 * cursor_transform.translation.x
            );
            cursor_transform.translation.y = cursor_transform.translation.y.clamp(
                - window.height() / (2. * ZOOM) + 0.1 * cursor_transform.translation.y,
                window.height() / (2. * ZOOM) + 0.1 * cursor_transform.translation.y
            );
        }
    }
}