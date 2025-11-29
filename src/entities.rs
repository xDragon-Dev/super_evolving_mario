use crate::components::*;
use bevy::{platform::collections::HashMap, post_process::bloom::Bloom, prelude::*};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let mario_texture: Handle<Image> = asset_server.load("small_mario.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(17, 16), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layout.add(layout);

    commands.spawn((
        Sprite {
            image: mario_texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..Default::default()
        },
        Transform::from_scale(Vec3::new(5.5, 5.5, 5.5)),
        SpriteAnimationState::Idle,
        FacingDirection::Right,
        SpriteSelectorHelper::new(Timer::from_seconds(1.0 / 15.0, TimerMode::Once), 0),
        SpriteAnimationSet::new(HashMap::from([
            (
                SpriteAnimationState::Idle,
                SpriteAnimationClip {
                    frames: vec![0],
                    fps: 15.0,
                },
            ),
            (
                SpriteAnimationState::Walk,
                SpriteAnimationClip {
                    frames: vec![1, 2, 3],
                    fps: 15.0,
                },
            ),
            (
                SpriteAnimationState::Jump,
                SpriteAnimationClip {
                    frames: vec![5],
                    fps: 15.0,
                },
            ),
            (
                SpriteAnimationState::Dead,
                SpriteAnimationClip {
                    frames: vec![6],
                    fps: 15.0,
                },
            ),
        ])),
        Bloom::default(),
        Mario,
    ));
}
