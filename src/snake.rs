use super::*;
use bevy::prelude::*;
use std::time::Duration;

pub struct SnakeMoveTimer(pub Timer);

pub struct SnakeHead {
    pub direction: SnakeDirection,
}

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

#[derive(PartialEq, Copy, Clone)]
pub enum SnakeDirection {
    Left,
    Up,
    Right,
    Down,
}

impl SnakeDirection {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

pub fn snake_timer(time: Res<Time>, mut snake_timer: ResMut<SnakeMoveTimer>) {
    snake_timer.0.tick(time.delta_seconds);
}

pub fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    snake_timer: ResMut<SnakeMoveTimer>,
    mut heads: Query<(Entity, &mut SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, mut head)) = heads.iter_mut().next() {
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        let direction: SnakeDirection = if keyboard_input.pressed(KeyCode::Left) {
            SnakeDirection::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            SnakeDirection::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            SnakeDirection::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            SnakeDirection::Right
        } else {
            head.direction
        };
        if direction != head.direction.opposite() {
            head.direction = direction;
        }
        if !snake_timer.0.finished {
            return;
        }
        match &head.direction {
            SnakeDirection::Left => {
                head_pos.x -= 1;
            }
            SnakeDirection::Right => {
                head_pos.x += 1;
            }
            SnakeDirection::Up => {
                head_pos.y += 1;
            }
            SnakeDirection::Down => {
                head_pos.y -= 1;
            }
        }
    }
}
