//use crate::mario::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::{/*LdtkEntity,*/ LdtkIntCell};
use bevy_rapier2d::prelude::*;

/*
#[derive(Bundle, LdtkEntity, Default)]
pub struct MarioBundle {
    mario: Mario,
}
*/

#[derive(Component, Default)]
pub struct KillZone;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct KillZoneBundle {
    kill_zone: KillZone,
}

#[derive(Component, Default)]
pub struct Solid;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct SolidBundle {
    solid: Solid,
}

#[derive(Component, Default)]
pub struct Goal;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct GoalBundle {
    goal: Goal,
}

pub fn viewport_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.25,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_translation(Vec3::new(225.0, -110.0, 1.0)),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk").into(),
        ..Default::default()
    });
}

pub fn setup_kill_zone_cells(mut commands: Commands, solids: Query<Entity, Added<KillZone>>) {
    for e in solids.iter() {
        commands.entity(e).insert((
            Collider::cuboid(8.0, 8.0),
            RigidBody::Fixed,
            Ccd::enabled(),
            Sensor,
            CollisionGroups {
                memberships: Group::from_bits_retain(2),
                filters: Group::from_bits_retain(1),
            },
        ));
    }
}

pub fn setup_solid_cells(mut commands: Commands, solids: Query<Entity, Added<Solid>>) {
    for e in solids.iter() {
        commands.entity(e).insert((
            Collider::cuboid(8.0, 8.0),
            RigidBody::Fixed,
            Ccd::enabled(),
            CollisionGroups {
                memberships: Group::from_bits_retain(2),
                filters: Group::from_bits_retain(1),
            },
        ));
    }
}

pub fn setup_goal_cells(mut commands: Commands, solids: Query<Entity, Added<Goal>>) {
    for e in solids.iter() {
        commands.entity(e).insert((
            Collider::cuboid(8.0, 8.0),
            RigidBody::Fixed,
            Sensor,
            Ccd::enabled(),
            CollisionGroups {
                memberships: Group::from_bits_retain(2),
                filters: Group::from_bits_retain(1),
            },
        ));
    }
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                int_grid_rendering: IntGridRendering::Invisible,
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .insert_resource(LevelSelection::Uid(0))
            .register_ldtk_int_cell::<KillZoneBundle>(1)
            .register_ldtk_int_cell::<SolidBundle>(2)
            .register_ldtk_int_cell::<GoalBundle>(3)
            //.register_ldtk_entity::<MarioBundle>("Mario")
            .add_systems(Update, setup_kill_zone_cells)
            .add_systems(Update, setup_solid_cells)
            .add_systems(Update, setup_goal_cells)
            .add_systems(Startup, viewport_setup);
    }
}
