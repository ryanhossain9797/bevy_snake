use super::*;
use bevy::prelude::*;
pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub segment_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
}

pub struct GameOverEvent;

pub fn spawn_snake(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    segments.0 = vec![
        commands
            .spawn(SpriteComponents {
                material: materials.head_material.clone(),
                ..Default::default()
            })
            .with(SnakeHead {
                direction: SnakeDirection::Up,
            })
            .with(SnakeSegment)
            .with(Position { x: 3, y: 3 })
            .with(Shape::square(0.8))
            .current_entity()
            .unwrap(),
        spawn_segment(
            &mut commands,
            &materials.segment_material,
            Position { x: 3, y: 2 },
        ),
    ];
}

pub fn game_over(
    mut commands: Commands,
    mut reader: Local<EventReader<GameOverEvent>>,
    game_over_events: Res<Events<GameOverEvent>>,
    materials: Res<Materials>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<With<Food, Entity>>,
    segments: Query<With<SnakeSegment, Entity>>,
) {
    if reader.iter(&game_over_events).next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.despawn(ent);
        }
        spawn_snake(commands, materials, segments_res);
    }
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Shape, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
