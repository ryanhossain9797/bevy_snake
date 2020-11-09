mod food;
mod game;
mod snake;

use bevy::{prelude::*, render::pass::ClearColor};

use food::*;
use game::*;
use snake::*;

const ARENA_WIDTH: u32 = 60;
const ARENA_HEIGHT: u32 = 40;
const TILE_SIZE: u32 = 20;
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: ARENA_WIDTH * TILE_SIZE,
            height: ARENA_HEIGHT * TILE_SIZE,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", game_setup.system())
        .add_system(snake_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_plugins(DefaultPlugins)
        .run();
}

pub fn setup(mut commands: Commands, mut color_materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: color_materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}
