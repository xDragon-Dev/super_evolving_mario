use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::mario::*;

#[derive(Component, PartialEq)]
pub enum FacingDirection {
    Right,
    Left,
}

#[derive(Component)]
pub struct SpriteSelectorHelper {
    pub timer: Timer,
    pub index: usize,
}

impl SpriteSelectorHelper {
    pub fn new(timer: Timer, index: usize) -> Self {
        Self { timer, index }
    }
}

pub struct SpriteAnimationClip {
    pub frames: Vec<usize>,
    pub fps: f32,
}

#[derive(Component)]
pub struct SpriteAnimationSet {
    pub animations: HashMap<MarioGlobalState, SpriteAnimationClip>,
}

impl SpriteAnimationSet {
    pub fn new(animations: HashMap<MarioGlobalState, SpriteAnimationClip>) -> Self {
        Self { animations }
    }
}

pub fn flip_sprite(mut sprites: Query<(&mut Sprite, &FacingDirection)>) {
    for (mut sprite, facing_direction) in &mut sprites {
        sprite.flip_x = *facing_direction == FacingDirection::Left;
    }
}

pub fn update_sprite_animation(
    mut sprite_data: Query<(
        &mut Sprite,
        &mut SpriteSelectorHelper,
        &MarioGlobalState,
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

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flip_sprite)
            .add_systems(Update, update_sprite_animation);
    }
}
