use crate::mario::*;
use crate::sprite::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn update_mario_state(
    mut states: Query<&mut SpriteAnimationState, With<Mario>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut state in &mut states {
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::ArrowRight]) {
            *state = SpriteAnimationState::Walk;
        } else if keys.pressed(KeyCode::Space) {
            *state = SpriteAnimationState::Jump;
        } else {
            *state = SpriteAnimationState::Idle;
        }
    }
}

pub fn update_mario_facing_direction(
    mut sprites: Query<&mut FacingDirection, With<Mario>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut facing_direction in &mut sprites {
        if keys.pressed(KeyCode::ArrowRight) {
            *facing_direction = FacingDirection::Right;
        } else if keys.pressed(KeyCode::ArrowLeft) {
            *facing_direction = FacingDirection::Left;
        }
    }
}

pub fn mario_horizontal_move(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Velocity, With<Mario>>,
) {
    let speed = 200.0;

    for mut vel in &mut q {
        if keyboard.pressed(KeyCode::ArrowLeft) {
            vel.linvel.x = -speed;
        } else if keyboard.pressed(KeyCode::ArrowRight) {
            vel.linvel.x = speed;
        } else {
            vel.linvel.x = 0.0;
        }
    }
}

pub fn mario_jump(keyboard: Res<ButtonInput<KeyCode>>, mut q: Query<&mut Velocity, With<Mario>>) {
    let jump_speed = 400.0;

    for mut vel in &mut q {
        if keyboard.just_pressed(KeyCode::Space) {
            vel.linvel.y = jump_speed;
        }
    }
}

pub struct InputMovementPlugin;

impl Plugin for InputMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_mario_state)
            .add_systems(Update, update_mario_facing_direction)
            .add_systems(Update, mario_horizontal_move)
            .add_systems(Update, mario_jump);
    }
}
