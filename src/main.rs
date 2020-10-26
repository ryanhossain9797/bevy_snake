mod food;
mod game;
mod snake;

use bevy::{prelude::*, render::pass::ClearColor};

use food::*;
use game::*;
use snake::*;

const ARENA_WIDTH: u32 = 60;
const ARENA_HEIGHT: u32 = 40;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Snake!".to_string(), // <--
            width: ARENA_WIDTH * 20,     // <--
            height: ARENA_HEIGHT * 20,   // <--
            ..Default::default()         // <--
        })
        .add_resource(ClearColor(Color::rgb(0.03, 0.03, 0.03)))
        .add_resource(SnakeMoveTimer::default())
        .add_resource(FoodSpawnTimer::default())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", game_setup.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(snake_movement_system.system())
        .add_system(food_spawner_system.system())
        .add_system(game_over_system.system())
        .add_event::<GameOverEvent>()
        .add_default_plugins()
        .run();
}
