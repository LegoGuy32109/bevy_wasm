use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};

const DISTANCE: f32 = 32.;

fn main() {
    App::new()
        .add_plugins(define_plugins())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, keyboard_movement)
        .run();
}

fn define_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                // fill entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            // server won't check for meta files won't clog with 404s
            // if needed in future try AssetMetaCheck::Paths(...)
            meta_check: AssetMetaCheck::Never,
            ..default()
        })
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2d, Camera::default()));

    // Load a sprite for the player; you must have an image at "assets/Dwarf.png"
    let dwarf_texture = asset_server.load("sprites/Dwarf.png");
    commands.spawn((
        Sprite::from_image(dwarf_texture),
        Transform::from_xyz(1., 1., 1.),
        Player,
    ));

    // Load rock sprite
    let rock_texture = asset_server.load("sprites/Rock.png");

    let chunk_size = UVec2::splat(64);
    let tile_display_size = UVec2::splat(32);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| Some(TileData::from_tileset_index(0)))
        .collect();

    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: rock_texture,
            ..default()
        },
        TilemapChunkTileData(tile_data),
    ));
}

fn keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    // time: Res<Time<Fixed>>,
) {
    for mut transform in query.iter_mut() {
        let mut dir = Vec3::ZERO;

        if keyboard_input.just_pressed(KeyCode::KeyE) {
            dir.y += 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            dir.y -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            dir.x -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::KeyF) {
            dir.x += 1.0;
        }

        if dir != Vec3::ZERO {
            // Normalize so diagonal movement isn’t faster
            let dir = dir.normalize();
            transform.translation += dir * DISTANCE
        }
    }
}
