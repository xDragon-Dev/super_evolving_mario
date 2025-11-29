use crate::components::*;
use bevy::prelude::*;

pub fn flip_sprite(mut sprites: Query<(&mut Sprite, &FacingDirection)>) {
    for (mut sprite, facing_direction) in &mut sprites {
        sprite.flip_x = *facing_direction == FacingDirection::Left;
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

pub fn update_sprite_animation(
    mut sprite_data: Query<(
        &mut Sprite,
        &mut SpriteSelectorHelper,
        &SpriteAnimationState,
        &SpriteAnimationSet,
    )>,
    time: Res<Time>,
) {
    for (mut sprite, mut sprite_helper, animation_state, animation_set) in &mut sprite_data {
        if sprite_helper.timer.tick(time.delta()).just_finished() {
            if let Some(animation_clip) = animation_set.animations.get(animation_state) {
                let frames = &animation_clip.frames;
                sprite_helper.timer.reset();
                sprite_helper
                    .timer
                    .set_duration(std::time::Duration::from_secs_f32(1.0 / animation_clip.fps));
                sprite_helper.index = (sprite_helper.index + 1) % frames.len();
                sprite.texture_atlas.as_mut().unwrap().index = frames[sprite_helper.index];
            }
        }
    }
}

/* 
// TODO: ESTO ES UNA COPIA DE LA FUNCIÓN "update_mario_facing_direction", su correcta implementación está pendiente
// !: Debe actualizarse al colisionar con un borde de hitbox vertical
pub fn update_walking_enemy_facing_direction(
    mut sprites: Query<&mut FacingDirection, With<WalkingEnemy>>,
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
*/

