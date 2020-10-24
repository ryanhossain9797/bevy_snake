use bevy::prelude::*;
use std::time::Duration;

pub struct SnakeMoveTimer(Timer);

impl SnakeMoveTimer {
    pub fn default() -> Self {
        Self(Timer::new(Duration::from_millis(150), true))
    }
}

pub struct SnakeHead {
    pub direction: Direction,
    pub next_segment: Entity,
}

impl SnakeHead {
    pub fn new(first_segment: Entity) -> Self {
        Self {
            direction: Direction::Up,
            next_segment: first_segment,
        }
    }
}

pub struct SnakeHeadMaterial(Handle<ColorMaterial>);

impl SnakeHeadMaterial {
    pub fn new(handle: Handle<ColorMaterial>) -> Self {
        Self(handle)
    }
    pub fn handle(&self) -> Handle<ColorMaterial> {
        self.0
    }
}

#[derive(Default)]
pub struct SnakeSegment {
    next_segment: Option<Entity>,
}

pub struct SnakeSegmentMaterial(Handle<ColorMaterial>);

impl SnakeSegmentMaterial {
    pub fn new(handle: Handle<ColorMaterial>) -> Self {
        Self(handle)
    }
    pub fn handle(&self) -> Handle<ColorMaterial> {
        self.0
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct HeadSize {
    pub width: f32,
    pub height: f32,
}

impl HeadSize {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

//Transform component comes from SpriteComponents
pub fn snake_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_timer: ResMut<SnakeMoveTimer>,
    mut snake_heads: Query<(&mut SnakeHead, &mut Position)>,
    segments: Query<&mut SnakeSegment>,
    positions: Query<&mut Position>,
) {
    snake_timer.0.tick(time.delta_seconds);
    for (mut head, mut head_pos) in &mut snake_heads.iter() {
        let mut direction = head.direction;

        if keyboard_input.pressed(KeyCode::Left) {
            direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction = Direction::Down;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction = Direction::Up;
        }

        if direction != head.direction.opposite() {
            head.direction = direction;
        }

        if snake_timer.0.finished {
            let mut last_position = *head_pos;
            let mut segment_entity = head.next_segment;
            loop {
                let segment = segments.get::<SnakeSegment>(segment_entity).unwrap();
                let mut segment_position = positions.get_mut::<Position>(segment_entity).unwrap();
                std::mem::swap(&mut last_position, &mut *segment_position);
                if let Some(n) = segment.next_segment {
                    segment_entity = n;
                } else {
                    break;
                }
            }

            match head.direction {
                Direction::Left => head_pos.x -= 1,
                Direction::Right => head_pos.x += 1,
                Direction::Up => head_pos.y += 1,
                Direction::Down => head_pos.y -= 1,
            }
        }
    }
}

pub fn spawn_segment(commands: &mut Commands, material: Handle<ColorMaterial>, position: Position) {
    commands
        .spawn(SpriteComponents {
            material,
            ..Default::default()
        })
        .with(SnakeSegment { next_segment: None })
        .with(position)
        .with(HeadSize::square(0.65));
}
