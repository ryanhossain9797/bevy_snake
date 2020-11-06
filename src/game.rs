use super::*;
use bevy::prelude::*;
pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
}

pub fn game_setup(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteComponents {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(SnakeHead);
}
