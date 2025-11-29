use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Component)]
pub struct Mario;

//#[derive(Component)]
//pub struct WalkingEnemy;

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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteAnimationState {
    Idle,
    Walk,
    Jump,
    //Hurt,
    //Recover,
    Dead,
}

pub struct SpriteAnimationClip {
    pub frames: Vec<usize>,
    pub fps: f32,
}

#[derive(Component)]
pub struct SpriteAnimationSet {
    pub animations: HashMap<SpriteAnimationState, SpriteAnimationClip>,
}

impl SpriteAnimationSet {
    pub fn new(animations: HashMap<SpriteAnimationState, SpriteAnimationClip>) -> Self {
        Self { animations }
    }
}
