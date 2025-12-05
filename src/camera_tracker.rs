use crate::mario::*;
use bevy::prelude::*;

pub fn camera_follow_mario(
    marios: Query<&Transform, With<Mario>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Mario>)>,
) {
    if let Some(mario_transform) = marios
        .iter()
        .max_by(|a, b| a.translation.x.partial_cmp(&b.translation.x).unwrap())
    {
        if let Ok(mut cam_transform) = camera.single_mut() {
            cam_transform.translation.x = mario_transform.translation.x;
        }
    }
}

pub struct CameraTrackerPlugin;

impl Plugin for CameraTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow_mario);
    }
}
