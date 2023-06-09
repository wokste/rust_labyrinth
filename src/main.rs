mod ai;
mod combat;
mod game;
mod map;
mod modelgen;
mod physics;
mod player;
mod procgen;
mod rendering;
mod ui;
mod weapon;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_plugin(EguiPlugin)
        .add_plugin(ui::UIPlugin)
        .add_startup_system(game_setup)
        .add_startup_system(level_setup.after(game_setup))
        .insert_resource(map::MapData::default())
        .insert_resource(rendering::SpriteResource::default())
        .insert_resource(GameInfo::default())
        .add_system(player::player_input)
        .add_system(player::update_map)
        .add_system(ai::ai_los.after(player::update_map))
        .add_system(ai::ai_fire.after(ai::ai_los))
        .add_system(physics::do_physics.after(player::player_input))
        .add_system(weapon::check_projectile_creature_collisions)
        .add_system(weapon::fire_weapons.after(player::player_input).after(ai::ai_fire))

        .add_system(rendering::face_camera.after(physics::do_physics))
        .add_system(rendering::animate_sprites)
        //.add_system(ui::hud::render_hud)

        .run();
}

fn game_setup(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut render_res: ResMut<rendering::SpriteResource>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.5;

    let texture = asset_server.load("C:/Users/wokste/Desktop/labyrinth_textures2.png");

    render_res.material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: true,
        ..default()
        //Color::WHITE.into()
    });
}

/// set up the game
fn level_setup(
    mut commands: Commands,
    mut map_data: ResMut<map::MapData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<rendering::SpriteResource>,
) {
    map_data.map = procgen::make_map(fastrand::u8(1..=5));

    // The actual map
    commands.spawn(PbrBundle {
        mesh: meshes.add( modelgen::map_to_mesh(&map_data.map)),
        material: render_res.material.clone(),
        ..default()
    });

    // Player
    let player_pos = map_data.map.random_square();
    let player_pos = Vec3::new(player_pos.x as f32 + 0.5, 0.7, player_pos.z as f32 + 0.5);
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(player_pos).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(player::PlayerBundle::default());
    map_data.player_pos = player_pos;

    for _ in 1 .. 20 {
        ai::spawn_monster(&mut commands, &map_data, &mut meshes, &mut render_res);
    }
}

// This resource tracks the game's score
#[derive(Resource, Default)]
pub struct GameInfo {
//    hp: i32,
//    hp_max: i32,
    pub score: i32,
    pub coins : i32,
}