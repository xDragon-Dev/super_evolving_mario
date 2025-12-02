use bevy::prelude::*;

mod camera_tracker;
mod input_movement;
mod mario;
mod physics;
mod sprite;
mod tiles;
mod ga;

use crate::camera_tracker::*;
use crate::input_movement::*;
use crate::mario::*;
use crate::physics::*;
use crate::sprite::*;
use crate::tiles::*;
use crate::ga::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilesPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(MarioPlugin)
        .add_plugins(SpritePlugin)
        .add_plugins(InputMovementPlugin)
        .add_plugins(CameraTrackerPlugin)
        .run();
}
