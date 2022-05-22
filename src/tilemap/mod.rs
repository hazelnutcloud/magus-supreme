mod loader;

use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy_ecs_tilemap::prelude::*;
use heron::Velocity;

use crate::player::Player;

use self::loader::{TiledMap, TiledMapBundle, TiledMapPlugin};

pub struct Tilemap;

impl Plugin for Tilemap {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_plugin(TiledMapPlugin)
            .add_startup_system(spawn)
            .add_system(set_texture_filters_to_nearest)
            .add_system(sort_z_by_y);
    }
}

fn spawn(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<TiledMap> = server.load("tilemap/dungeon-tilemap.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(-400.0, -200.0, 1.0),
        ..Default::default()
    });
}

fn sort_z_by_y(
        mut map_query: MapQuery,
        mut player_query: Query<(&mut Transform, ChangeTrackers<Velocity>), With<Player>>
    ) {
    if let Ok((mut player_transform, velocity_changed)) = player_query.get_single_mut() {
        //only execute if player just moved
        if !velocity_changed.is_changed() { return }
        
        //get player sprite's foot position
        let base_x = player_transform.translation.x;
        let base_y = player_transform.translation.y - 9.;

        //check if we are on a tile that we want to sort by y
        if let Ok(_tile) = map_query.get_tile_entity(
            TilePos(
                ((base_x + 400.) / 16.).floor() as u32,
                ((base_y + 200.) / 16.).floor() as u32,
            ),
            0u16,
            2u16,
        ) {
            //if we are, set the player's z-index to be below the layer's z-index which is 20
            player_transform.translation.z = 19.;
        } else {
            player_transform.translation.z = 21.;
        }
    }
}

fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
