use crate::sprite::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Mario;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarioGlobalState {
    Idle,
    Walk,
    Jump,
    Dead,
}

/*
pub fn setup_mario(
    mut comands: Commands,
    entities: Query<Entity, Added<Mario>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mario_texture: Handle<Image> = asset_server.load("Small_mario.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(17, 16), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layout.add(layout);

    for entity in entities {
        comands.entity(entity).insert(generate_mario_entity(
            mario_texture.clone(),
            texture_atlas_layout.clone(),
        ));
    }
}
*/

pub fn generate_mario_entity(
    mario_texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> (
    Sprite,
    MarioGlobalState,
    FacingDirection,
    SpriteSelectorHelper,
    SpriteAnimationSet,
    Transform,
    RigidBody,
    Collider,
    LockedAxes,
    Velocity,
    GravityScale,
    Ccd,
    Mario,
    CollisionGroups,
) {
    (
        Sprite {
            image: mario_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..Default::default()
        },
        MarioGlobalState::Idle,
        FacingDirection::Right,
        SpriteSelectorHelper::new(Timer::from_seconds(1.0 / 15.0, TimerMode::Once), 0),
        SpriteAnimationSet::new(HashMap::from([
            (
                MarioGlobalState::Idle,
                SpriteAnimationClip {
                    frames: vec![0],
                    fps: 15.0,
                },
            ),
            (
                MarioGlobalState::Walk,
                SpriteAnimationClip {
                    frames: vec![1, 2, 3],
                    fps: 15.0,
                },
            ),
            (
                MarioGlobalState::Jump,
                SpriteAnimationClip {
                    frames: vec![5],
                    fps: 15.0,
                },
            ),
            (
                MarioGlobalState::Dead,
                SpriteAnimationClip {
                    frames: vec![6],
                    fps: 15.0,
                },
            ),
        ])),
        Transform::from_translation(Vec3::new(225.0, -110.0, 2.0)),
        RigidBody::Dynamic,
        Collider::cuboid(7.0, 8.0),
        LockedAxes::ROTATION_LOCKED, // para que no se voltee
        Velocity::zero(),
        GravityScale(1.0),
        Ccd::enabled(),
        Mario,
        CollisionGroups {
            memberships: Group::from_bits_retain(1),
            filters: Group::from_bits_retain(2),
        },
    )
}
