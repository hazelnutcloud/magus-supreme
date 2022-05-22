use heron::PhysicsLayer;

pub mod camera;
pub mod player;
pub mod tilemap;

#[derive(PhysicsLayer)]
pub enum GameCollisionLayer {
    World,
    Player
}
