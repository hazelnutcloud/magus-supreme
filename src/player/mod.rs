use std::{
    fmt::Debug,
    ops::{Deref, Mul},
    time::Duration,
};

use benimator::{Play, SpriteSheetAnimation};
use bevy::prelude::*;
use heron::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{tilemap::TILEMAP_HEIGHT, GameCollisionLayer};

// =========================================================
// ==================== PLAYER PLUGIN ======================
// =========================================================

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_animations)
            .add_startup_system(spawn)
            .add_system(movement)
            .add_system(movement_animation)
            .add_system(update_z_index);
    }
}

// =========================================================
// ===================== COMPONENTS ========================
// =========================================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementSpeed(f32);

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    body: RigidBody,
    collision_shape: CollisionShape,
    velocity: Velocity,
    acceleration: Acceleration,
    physics_material: PhysicMaterial,
    constraint: RotationConstraints,
    collision_layers: CollisionLayers,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    movement_speed: MovementSpeed,
    #[bundle]
    physics: PhysicsBundle,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        //movement
        input_map.insert(Up, KeyCode::Up);
        input_map.insert(Up, KeyCode::W);

        input_map.insert(Down, KeyCode::Down);
        input_map.insert(Down, KeyCode::S);

        input_map.insert(Left, KeyCode::Left);
        input_map.insert(Left, KeyCode::A);

        input_map.insert(Right, KeyCode::Right);
        input_map.insert(Right, KeyCode::D);

        input_map.insert(Dash, KeyCode::LShift);

        input_map
    }

    fn default_physics() -> PhysicsBundle {
        PhysicsBundle {
            collision_shape: CollisionShape::Capsule {
                half_segment: 5.,
                radius: 7.,
            },
            collision_layers: CollisionLayers::none()
                .with_group(GameCollisionLayer::Player)
                .with_mask(GameCollisionLayer::World),
            constraint: RotationConstraints::lock(),
            body: RigidBody::Dynamic,
            ..Default::default()
        }
    }
}

// =========================================================
// ======================= ACTIONS =========================
// =========================================================

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Dash,
}

// =========================================================
// ====================== RESOURCES ========================
// =========================================================

struct PlayerAtlas(Handle<TextureAtlas>);

struct PlayerAnimations {
    idle: Handle<SpriteSheetAnimation>,
    moving: Handle<SpriteSheetAnimation>,
}

// =========================================================
// ======================= SYSTEMS =========================
// =========================================================

// ------ load assets --------
fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = server.load("spritesheets/wizard.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(26., 27.), 36, 1);
    let handle = atlases.add(atlas);

    commands.insert_resource(PlayerAtlas(handle));
}

// ---- load animations ------
fn load_animations(mut commands: Commands, mut animations: ResMut<Assets<SpriteSheetAnimation>>) {
    let idle = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(150),
    ));

    let moving = animations.add(SpriteSheetAnimation::from_range(
        4..=7,
        Duration::from_millis(150),
    ));

    commands.insert_resource(PlayerAnimations { idle, moving });
}

// ----- spawn player --------
fn spawn(mut commands: Commands, spritesheet: Res<PlayerAtlas>, animations: Res<PlayerAnimations>) {
    commands
        .spawn_bundle(PlayerBundle {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: spritesheet.0.clone(),
                transform: Transform::from_xyz(400., 200., 0.),
                ..Default::default()
            },
            input_manager: InputManagerBundle {
                action_state: ActionState::default(),
                input_map: PlayerBundle::default_input_map(),
            },
            physics: PlayerBundle::default_physics(),
            movement_speed: MovementSpeed(75.),
            player: Player,
        })
        .insert(animations.idle.clone())
        .insert(Play);
}

// ---- handle movement ------
fn movement(
    mut query: Query<(&ActionState<PlayerAction>, &MovementSpeed, &mut Velocity), With<Player>>,
) {
    let (action_state, movement_speed, mut velocity) = query.single_mut();
    if action_state.pressed(PlayerAction::Up) == action_state.pressed(PlayerAction::Down) {
        velocity.linear.y = 0.;
    } else if action_state.pressed(PlayerAction::Up) {
        velocity.linear.y = movement_speed.0;
    } else if action_state.pressed(PlayerAction::Down) {
        velocity.linear.y = -movement_speed.0;
    }

    if action_state.pressed(PlayerAction::Left) == action_state.pressed(PlayerAction::Right) {
        velocity.linear.x = 0.;
    } else if action_state.pressed(PlayerAction::Left) {
        velocity.linear.x = -movement_speed.0;
    } else if action_state.pressed(PlayerAction::Right) {
        velocity.linear.x = movement_speed.0;
    }

    if velocity.linear.x != 0. && velocity.linear.y != 0. {
        velocity.linear = velocity.linear.normalize().mul(movement_speed.0);
    }
}

// --- animate movement ------
fn movement_animation(
    mut query: Query<
        (
            &Velocity,
            &mut TextureAtlasSprite,
            &mut Handle<SpriteSheetAnimation>,
        ),
        With<Player>,
    >,
    animations: Res<PlayerAnimations>,
    mut last_facing_is_left: Local<bool>,
) {
    let (velocity, mut sprite, mut animation) = query.single_mut();

    let is_moving_horizontally = velocity.linear.x != 0.;

    if is_moving_horizontally {
        let is_facing_left = velocity.linear.x < 0.;
        if is_facing_left != *last_facing_is_left {
            *last_facing_is_left = is_facing_left;
        }
    }

    sprite.flip_x = *last_facing_is_left;

    let is_moving = velocity.linear != Vec3::ZERO;

    if is_moving {
        if animation.deref() == &animations.moving {
            return;
        }
        *animation = animations.moving.clone();
    } else {
        if animation.deref() == &animations.idle {
            return;
        }
        *animation = animations.idle.clone();
    }
}

// ---- update z index -------
fn update_z_index(mut query: Query<&mut Transform, (With<Player>, Changed<Transform>)>) {
    if let Ok(mut player) = query.get_single_mut() {
        player.translation.z = TILEMAP_HEIGHT - (player.translation.y - 9.); // distance of player's feet from top of map
    }
}
