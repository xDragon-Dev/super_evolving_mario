use bevy::prelude::*;

mod camera;
mod genetic_algorithm;
mod movement;
mod level;
mod mario;
mod physics;
mod sprite;

use crate::camera::*;
use crate::genetic_algorithm::*;
use crate::level::*;
use crate::movement::action_schedule_movement::*;
use crate::physics::*;
use crate::sprite::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilesPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(SpritePlugin)
        .add_plugins(CameraTrackerPlugin)
        .add_plugins(AutoMovementPlugin)
        .add_plugins(GeneticAlgorithmPlugin)
        .run();
}
