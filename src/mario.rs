use crate::sprite::*;
use crate::tiles::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Mario;

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
        comands.entity(entity).insert((
            Sprite {
                image: mario_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..Default::default()
            },
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
            RigidBody::Dynamic,
            Collider::cuboid(7.0, 8.0),
            LockedAxes::ROTATION_LOCKED, // para que no se voltee
            Velocity::zero(),
            GravityScale(1.0),
            Ccd::enabled(),
        ));
    }
}


pub struct MarioPlugin;

impl Plugin for MarioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, setup_mario)
            //.add_systems(Update, detect_ground)
            ;
    }
}
