use super::*;
use bevy::prelude::*;
use rand::prelude::random;
use std::time::Duration;

pub struct FoodSpawnTimer(Timer);

impl FoodSpawnTimer {
    pub fn default() -> Self {
        Self(Timer::new(Duration::from_millis(5000), true))
    }
}

pub struct FoodMaterial(Handle<ColorMaterial>);
pub struct Food;

impl FoodMaterial {
    pub fn new(handle: Handle<ColorMaterial>) -> Self {
        Self(handle)
    }
    pub fn handle(&self) -> Handle<ColorMaterial> {
        self.0
    }
}

pub fn food_spawner_system(
    mut commands: Commands,
    food_material: Res<FoodMaterial>,
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        commands
            .spawn(SpriteComponents {
                material: food_material.0,
                ..Default::default()
            })
            .with(Food)
            .with(Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
            .with(HeadSize::square(0.8));
    }
}
