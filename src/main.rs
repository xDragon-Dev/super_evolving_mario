use bevy::prelude::*;

mod auto_movement;
mod camera_tracker;
mod genetic_algorithm;
//mod input_movement;  Ya no eres útil
mod mario;
mod physics;
mod sprite;
mod tiles;

use crate::auto_movement::*;
use crate::camera_tracker::*;
use crate::genetic_algorithm::*;
use crate::physics::*;
use crate::sprite::*;
use crate::tiles::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilesPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(SpritePlugin)
        //.add_plugins(InputMovementPlugin)
        .add_plugins(CameraTrackerPlugin)
        .add_plugins(AutoMovementPlugin)
        .add_plugins(GeneticAlgorithmPlugin)
        .run();
}
