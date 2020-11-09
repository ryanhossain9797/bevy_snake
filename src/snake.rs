use super::*;
use bevy::prelude::*;
use std::time::Duration;
pub struct SnakeHead;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Shape {
    pub width: f32,
    pub height: f32,
}

impl Shape {
    pub fn square(x: f32) -> Self {
        Shape {
            width: x,
            height: x,
        }
    }
}

pub fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<With<SnakeHead, &mut Position>>,
) {
    for mut pos in head_positions.iter_mut() {
        keyboard_input
            .get_pressed()
            .into_iter()
            .for_each(|keycode| match keycode {
                KeyCode::Left => pos.x -= 1,
                KeyCode::Right => pos.x += 1,
                KeyCode::Up => pos.y += 1,
                KeyCode::Down => pos.y -= 1,
                _ => {}
            });
    }
}
