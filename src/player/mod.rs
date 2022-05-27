use std::{
    fmt::Debug,
    ops::Deref,
    time::Duration,
};

use benimator::{SpriteSheetAnimation};
use bevy::{math::const_vec3, prelude::*};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike,
};
use serde::{Deserialize, Serialize};

use crate::tilemap::TILEMAP_HEIGHT;

// =========================================================
// ====================== CONSTANTS ========================
// =========================================================

pub const SPAWN_POINT: Vec3 = const_vec3!([400., 200., 0.]);

// =========================================================
// ==================== PLAYER PLUGIN ======================
// =========================================================

pub struct PlayerPlugin;

impl PlayerPlugin {
    pub fn client() -> PlayerPluginClient {
        PlayerPluginClient
    }

    pub fn server() -> PlayerPluginServer {
        PlayerPluginServer
    }
}
pub struct PlayerPluginClient;

impl Plugin for PlayerPluginClient {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_animations)
            .add_system(map_input)
            .add_system(movement_animation)
            .add_system(movement)
            .add_system(update_z_index);
    }
}

pub struct PlayerPluginServer;

impl Plugin for PlayerPluginServer {
    fn build(&self, app: &mut App) {
        app
            .add_system(movement);
    }
}

// =========================================================
// ===================== COMPONENTS ========================
// =========================================================

// ----- player marker -------
#[derive(Component, Default)]
pub struct Player {
    pub id: u64
}

// ---- movement speed -------
#[derive(Component, Default)]
pub struct MovementSpeed(pub f32);

// ---- player physics -------
#[derive(Bundle)]
pub struct PhysicsBundle {
    body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    constraint: LockedAxes,
}

// ----- player bundle -------
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub movement_speed: MovementSpeed,
    pub input: PlayerInput,
    #[bundle]
    pub physics: PhysicsBundle
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<PlayerAction> {
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

    pub fn default_physics() -> PhysicsBundle {
        PhysicsBundle {
            collider: Collider::capsule_y(5., 5.),
            constraint: LockedAxes::ROTATION_LOCKED,
            body: RigidBody::Dynamic,
            velocity: Velocity::default()
        }
    }
}

#[derive(Serialize, Deserialize, Component, Default)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub dash: bool
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

pub struct PlayerAtlas(pub Handle<TextureAtlas>);

pub struct PlayerAnimations {
    pub idle: Handle<SpriteSheetAnimation>,
    pub moving: Handle<SpriteSheetAnimation>,
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

// ------- map input ---------
fn map_input(
    mut query: Query<(&ActionState<PlayerAction>, &mut PlayerInput)>
) {
    if let Ok((action_state, mut player_input)) = query.get_single_mut() {
        player_input.up = action_state.pressed(PlayerAction::Up);
        player_input.down = action_state.pressed(PlayerAction::Down);
        player_input.left = action_state.pressed(PlayerAction::Left);
        player_input.right = action_state.pressed(PlayerAction::Right);
        player_input.dash = action_state.pressed(PlayerAction::Dash);
    }
}

// ---- handle movement ------
fn movement(
    mut query: Query<(&PlayerInput, &mut Velocity, &MovementSpeed), With<Player>>,
) {
    for (player_input, mut velocity, movement_speed) in query.iter_mut() {
        let x = (player_input.right as i8 - player_input.left as i8) as f32;
        let y = (player_input.up as i8 - player_input.down as i8) as f32;
    
        if x != 0. && y != 0. {
            velocity.linvel = Vec2::new(x, y).normalize() * movement_speed.0;
            return;
        }
    
        velocity.linvel = Vec2::new(x, y) * movement_speed.0;
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
    for (velocity, mut sprite, mut animation) in query.iter_mut() {
        let is_moving_horizontally = velocity.linvel.x != 0.;
    
        if is_moving_horizontally {
            let is_facing_left = velocity.linvel.x < 0.;
            if is_facing_left != *last_facing_is_left {
                *last_facing_is_left = is_facing_left;
            }
        }
    
        sprite.flip_x = *last_facing_is_left;
    
        let is_moving = velocity.linvel != Vec2::ZERO;
    
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
}

// ---- update z index -------
fn update_z_index(mut query: Query<&mut Transform, (With<Player>, Changed<Transform>)>) {
    for mut player in query.iter_mut() {
        player.translation.z = TILEMAP_HEIGHT - (player.translation.y - 9.); // distance of player's feet from top of map
    }
}
