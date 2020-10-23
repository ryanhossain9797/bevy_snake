use bevy::prelude::*;

pub struct SnakeHead;
pub struct SnakeHeadMaterial(Handle<ColorMaterial>);

impl SnakeHeadMaterial {
    pub fn new(handle: Handle<ColorMaterial>) -> Self {
        Self(handle)
    }
    pub fn handle(&self) -> Handle<ColorMaterial> {
        self.0
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
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_heads: Query<(&SnakeHead, &mut Position)>,
) {
    for (_, mut pos) in &mut snake_heads.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}
