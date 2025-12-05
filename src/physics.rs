use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::mario::*;
use crate::tiles::*;
use crate::genetic_algorithm::*;

pub fn handle_rapier_events(
    mut collision_events: MessageReader<CollisionEvent>,
    mut q_mario: Query<(&mut AgentState, &mut MarioGlobalState, &mut Velocity, &mut GravityScale), With<Mario>>,
    q_kill_zone: Query<&KillZone>, // Para verificar si la otra entidad es una KillZone
    q_goal: Query<&Goal>,         // Para verificar si la otra entidad es la Meta
) {
    for event in collision_events.read() {
        
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            let (mario_entity, other_entity) = 
                if q_mario.contains(*entity1) {
                    (*entity1, *entity2)
                } else if q_mario.contains(*entity2) {
                    (*entity2, *entity1)
                } else {
                    continue;
                };
            if q_kill_zone.contains(other_entity) {
                if let Ok((mut agent_state,mut sprite_state, mut velocity, mut gravity_scale)) = q_mario.get_mut(mario_entity) {
                    agent_state.finished = true;
                    *sprite_state = MarioGlobalState::Dead;
                    *velocity = Velocity::zero();
                    (*gravity_scale).0 = 0.0;
                    println!("Alguien ha muerto")
                }
            }
            if q_goal.contains(other_entity) {
                if let Ok((mut agent_state, mut sprite_state, _,_)) = q_mario.get_mut(mario_entity) {
                    agent_state.finished = true; 
                    *sprite_state = MarioGlobalState::Idle;
                    println!("Alguien ha ganado")
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
            .add_plugins(RapierDebugRenderPlugin::default())
        ;
    }
}
