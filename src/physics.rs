use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::genetic_algorithm::*;
use crate::level::*;
use crate::mario::*;

pub fn handle_rapier_events(
    mut collision_events: MessageReader<CollisionEvent>,
    mut q_mario: Query<
        (
            &mut AgentState,
            &mut MarioGlobalState,
            &mut Velocity,
            &mut GravityScale,
        ),
        With<Mario>,
    >,
    q_kill_zone: Query<&KillZone>, // Para verificar si la otra entidad es una KillZone
    q_goal: Query<&Goal>,          // Para verificar si la otra entidad es la Meta
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            let (mario_entity, other_entity) = if q_mario.contains(*entity1) {
                (*entity1, *entity2)
            } else if q_mario.contains(*entity2) {
                (*entity2, *entity1)
            } else {
                continue;
            };
            if q_kill_zone.contains(other_entity) {
                if let Ok((mut agent_state, mut sprite_state, mut velocity, mut gravity_scale)) =
                    q_mario.get_mut(mario_entity)
                {
                    agent_state.finished = true;
                    *sprite_state = MarioGlobalState::Dead;
                    *velocity = Velocity::zero();
                    (*gravity_scale).0 = 0.0;
                }
            }
            if q_goal.contains(other_entity) {
                if let Ok((mut agent_state, mut sprite_state, _, _)) = q_mario.get_mut(mario_entity)
                {
                    agent_state.finished = true;
                    *sprite_state = MarioGlobalState::Idle;
                }
            }
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_systems(Update, handle_rapier_events.after(PhysicsSet::SyncBackend))
            //.add_plugins(RapierDebugRenderPlugin::default())
            ;
    }
}

pub fn generate_physical_entity_components() -> (
    RigidBody,
    Collider,
    LockedAxes,
    Velocity,
    GravityScale,
    Ccd,
    CollisionGroups,
    ActiveEvents,
) {
    (
        RigidBody::Dynamic,
        Collider::cuboid(7.0, 8.0),
        LockedAxes::ROTATION_LOCKED, // para que no se voltee
        Velocity::zero(),
        GravityScale(1.0),
        Ccd::enabled(),
        CollisionGroups {
            memberships: Group::from_bits_retain(1),
            filters: Group::from_bits_retain(2),
        },
        ActiveEvents::COLLISION_EVENTS,
    )
}

pub fn generate_physical_solid_map_components() -> (RigidBody, Collider, Ccd, CollisionGroups) {
    (
        RigidBody::Fixed,
        Collider::cuboid(8.0, 8.0),
        Ccd::enabled(),
        CollisionGroups {
            memberships: Group::from_bits_retain(2),
            filters: Group::from_bits_retain(1),
        },
    )
}

pub fn generate_physical_sensor_map_components()
-> (RigidBody, Collider, Ccd, CollisionGroups, Sensor) {
    (
        RigidBody::Fixed,
        Collider::cuboid(8.0, 8.0),
        Ccd::enabled(),
        CollisionGroups {
            memberships: Group::from_bits_retain(2),
            filters: Group::from_bits_retain(1),
        },
        Sensor,
    )
}
