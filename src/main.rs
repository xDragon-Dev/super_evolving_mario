use bevy::prelude::*;

mod components;
mod entities;
mod systems;

use entities::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (flip_sprite, update_mario_facing_direction).chain())
        .add_systems(
            Update,
            (update_mario_state, update_sprite_animation).chain(),
        )
        .run();
}