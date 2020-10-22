use bevy::prelude::*;

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_default_plugins()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
