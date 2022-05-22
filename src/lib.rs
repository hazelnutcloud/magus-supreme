use heron::PhysicsLayer;

pub mod camera;
pub mod player;
pub mod tilemap;

//  physics collision layers --
#[derive(PhysicsLayer)]
pub enum GameCollisionLayer {
    World,
    Player,
}
