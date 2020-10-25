use super::*;

use bevy::prelude::*;

pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(SnakeHeadMaterial::new(
        materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    ));
    commands.insert_resource(SnakeSegmentMaterial::new(
        materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    ));
    commands.insert_resource(FoodMaterial::new(
        materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    ));
}

pub fn game_setup(
    mut commands: Commands,
    snake_head_material: Res<SnakeHeadMaterial>,
    snake_segment_material: Res<SnakeSegmentMaterial>,
) {
    spawn_snake(
        &mut commands,
        snake_head_material.handle(),
        snake_segment_material.handle(),
        Position { x: 10, y: 10 },
    );
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&HeadSize, &mut Sprite)>) {
    for (size, mut sprite) in &mut q.iter() {
        let window = windows.get_primary().unwrap();
        sprite.size = Vec2::new(
            size.width as f32 / ARENA_WIDTH as f32 * window.width as f32,
            size.height as f32 / ARENA_HEIGHT as f32 * window.height as f32,
        );
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(p: f32, bound_window: f32, bound_game: f32) -> f32 {
        p / bound_game * bound_window - (bound_window / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in &mut q.iter() {
        transform.set_translation(Vec3::new(
            convert(pos.x as f32, window.width as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height as f32, ARENA_HEIGHT as f32),
            0.0,
        ))
    }
}
