use super::*;
use bevy::prelude::*;

pub struct SnakeMoveTimer(pub Timer);

pub struct SnakeHead {
    pub direction: SnakeDirection,
}

pub struct SnakeSegment;

#[derive(Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

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

pub struct GrowthEvent;

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
    segments: ResMut<SnakeSegments>,
    mut game_over_events: ResMut<Events<GameOverEvent>>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut heads: Query<(Entity, &mut SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    let segment_positions = segments
        .0
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();
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
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_events.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_events.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });

        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

pub fn food_consumption(
    mut commands: Commands,
    snake_timer: ResMut<SnakeMoveTimer>,
    mut growth_events: ResMut<Events<GrowthEvent>>,
    food_positions: Query<With<Food, (Entity, &Position)>>,
    head_positions: Query<With<SnakeHead, &Position>>,
) {
    if !snake_timer.0.finished {
        return;
    }
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.despawn(ent);
                growth_events.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    growth_events: Res<Events<GrowthEvent>>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: Local<EventReader<GrowthEvent>>,
    materials: Res<Materials>,
) {
    if growth_reader.iter(&growth_events).next().is_some() {
        segments.0.push(spawn_segment(
            &mut commands,
            &materials.segment_material,
            last_tail_position.0.unwrap(),
        ));
    }
}

pub fn spawn_segment(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn(SpriteComponents {
            material: material.clone(),
            ..SpriteComponents::default()
        })
        .with(SnakeSegment)
        .with(position)
        .with(Shape::square(0.65))
        .current_entity()
        .unwrap()
}
