mod loader;

use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy_ecs_tilemap::prelude::*;

use self::loader::{TiledMap, TiledMapBundle, TiledMapPlugin};

pub const TILEMAP_WIDTH: f32 = 800.;
pub const TILEMAP_HEIGHT: f32 = 800.;

pub struct Tilemap;

impl Plugin for Tilemap {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_plugin(TiledMapPlugin)
            .add_startup_system(spawn)
            .add_system(set_texture_filters_to_nearest);
    }
}

fn spawn(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<TiledMap> = server.load("tilemap/dungeon-tilemap.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(mut texture) = textures.get_mut(handle) {
                texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::COPY_DST;
            }
        }
    }
}
