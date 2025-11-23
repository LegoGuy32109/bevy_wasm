use std::time::Duration;

use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::sprite_render::{AlphaMode2d, TileData, TilemapChunk, TilemapChunkTileData};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const TILE_SIZE_IN_PX: u16 = 32;

const TILE_MAP_PATH: &str = "sprites/StackedTextures.png";
// WARN: CANNOT BE A MULTIPLE OF 6
const NUM_TILES_IN_MAP: u16 = 31;

fn main() {
    App::new()
        .add_plugins(define_plugins())
        .add_systems(Startup, setup)
        .add_systems(Update, (update_tileset_image, consume_action))
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

fn update_tileset_image(
    chunk_query: Single<&TilemapChunk>,
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    let chunk = *chunk_query;
    for event in events.read() {
        if event.is_loaded_with_dependencies(chunk.tileset.id()) {
            let image = images.get_mut(&chunk.tileset).unwrap();
            image.reinterpret_stacked_2d_as_array(NUM_TILES_IN_MAP.into());
        }
    }
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

    // Load textures for tile map
    let tile_textures: Handle<Image> = asset_server.load(TILE_MAP_PATH);

    let chunk_size = UVec2::splat(16);
    let tile_display_size = UVec2::splat(TILE_SIZE_IN_PX.into());

    // Determine data for tile map
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| Some(TileData::from_tileset_index(rng.random_range(1..=6))))
        .collect();

    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: tile_textures,
            alpha_mode: AlphaMode2d::Opaque,
        },
        TilemapChunkTileData(tile_data),
    ));
}

#[derive(Component, Debug)]
struct Action {
    target_entity: Entity,
    direction: Vec3,
    time_started: Duration,
    timer: Timer,
}

fn consume_action(
    mut commands: Commands,
    unprocessed_actions: Query<(Entity, &Action)>,
    mut entities_with_transforms: Query<&mut Transform, With<Player>>,
) {
    for (action_entity, action) in &unprocessed_actions {
        if action.timer.is_finished() {
            info!("Finished {:?}", action);
            commands.entity(action_entity).remove::<Action>();
        }
        let Ok(mut entity_transform) = entities_with_transforms.get_mut(action.target_entity)
        else {
            warn!("Invalid Action {:?}", action);
            commands.entity(action_entity).remove::<Action>();
            continue;
        };

        info!(?entity_transform);

        let fraction_done = action.timer.fraction();
        let start_transform = entity_transform.translation;
        let end_transform = start_transform + (action.direction * f32::from(TILE_SIZE_IN_PX));
        entity_transform.translation = Vec3::lerp(start_transform, end_transform, fraction_done);
    }
}

fn keyboard_movement(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
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
            commands.spawn(Action {
                target_entity: entity,
                direction: dir,
                time_started: time.elapsed(),
                timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            });
        }
    }

    // for mut transform in query.iter_mut() {
    //     info!(?transform);
    //     let mut dir = Vec3::ZERO;
    //
    //     if keyboard_input.just_pressed(KeyCode::KeyE) {
    //         dir.y += 1.0;
    //     }
    //     if keyboard_input.just_pressed(KeyCode::KeyD) {
    //         dir.y -= 1.0;
    //     }
    //     if keyboard_input.just_pressed(KeyCode::KeyS) {
    //         dir.x -= 1.0;
    //     }
    //     if keyboard_input.just_pressed(KeyCode::KeyF) {
    //         dir.x += 1.0;
    //     }
    //
    //     if dir != Vec3::ZERO {
    //         // Normalize so diagonal movement isn’t faster
    //         let dir = dir.normalize();
    //         transform.translation += dir * f32::from(TILE_SIZE_IN_PX);
    //     }
    // }
}
