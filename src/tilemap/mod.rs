mod loader;

use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy_ecs_tilemap::prelude::*;

use self::loader::{TiledMapBundle, TiledLoader, process_loaded_tile_maps_client, process_loaded_tile_maps_server, TiledMapServer, TiledMapClient};

// =========================================================
// ====================== CONSTANTS ========================
// =========================================================

pub const TILEMAP_WIDTH: f32 = 800.;
pub const TILEMAP_HEIGHT: f32 = 800.;

// =========================================================
// =================== TILEMAP PLUGIN ======================
// =========================================================

pub struct MagusTilemapPlugin;

impl MagusTilemapPlugin {
    pub fn client() -> MagusTilemapPluginClient {
        MagusTilemapPluginClient
    }
    pub fn server() -> MagusTilemapPluginServer {
        MagusTilemapPluginServer
    }
}

pub struct MagusTilemapPluginServer;

impl Plugin for MagusTilemapPluginServer {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<TiledMapServer>()
            .add_asset_loader(TiledLoader::server())
            .add_system(process_loaded_tile_maps_server)
            .add_startup_system(spawn_server);
    }
}

pub struct MagusTilemapPluginClient;

impl Plugin for MagusTilemapPluginClient {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_asset::<TiledMapClient>()
            .add_asset_loader(TiledLoader::client())
            .add_system(process_loaded_tile_maps_client)
            .add_startup_system(spawn)
            .add_system(set_texture_filters_to_nearest);
    }
}

// =========================================================
// ======================= SYSTEMS =========================
// =========================================================

// ----- spawn tilemap -------
fn spawn(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<TiledMapClient> = server.load("tilemap/dungeon-tilemap.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn spawn_server(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<TiledMapServer> = server.load("tilemap/dungeon-tilemap.tmx");

    commands.spawn()
        .insert(handle);
}

//  boilerplate to make code run 
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
