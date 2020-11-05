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
    App::build().run();
}
