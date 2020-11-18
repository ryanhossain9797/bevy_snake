mod food;
mod game;
mod snake;

use bevy::{prelude::*, render::pass::ClearColor};
use std::time::Duration;

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
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(SnakeMoveTimer(Timer::new(
            Duration::from_millis(150. as u64),
            true,
        )))
        .add_resource(LastTailPosition::default())
        .add_resource(SnakeSegments::default())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", spawn_snake.system())
        .add_system(snake_movement.system())
        .add_system(snake_timer.system())
        .add_system(food_spawner.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(food_consumption.system())
        .add_system(snake_growth.system())
        .add_system(game_over.system())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_plugins(DefaultPlugins)
        .run();
}

pub fn setup(mut commands: Commands, mut color_materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: color_materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        segment_material: color_materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        food_material: color_materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
}
