//use crate::mario::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::{/*LdtkEntity,*/ LdtkIntCell};

use crate::physics::{
    generate_physical_sensor_map_components, generate_physical_solid_map_components,
};

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

// * CONSERVAMOS EL HANDLE DEL ASSET DEL MUNDO PARA PODER UTILIZAR SUS METADADOS DEL MUNDO
// * COMO POR EJEMPLO EN
//#[derive(Resource, Default)]
//pub struct LevelAssetHandle(pub Handle<LdtkProject>);

pub fn viewport_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_project_handle: Handle<LdtkProject> = asset_server.load::<LdtkProject>("map.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: ldtk_project_handle.clone().into(),
        ..Default::default()
    });
    //commands.insert_resource(LevelAssetHandle(ldtk_project_handle));
}

pub fn setup_kill_zone_cells(mut commands: Commands, solids: Query<Entity, Added<KillZone>>) {
    for e in solids.iter() {
        commands
            .entity(e)
            .insert(generate_physical_sensor_map_components());
    }
}

pub fn setup_solid_cells(mut commands: Commands, solids: Query<Entity, Added<Solid>>) {
    for e in solids.iter() {
        commands
            .entity(e)
            .insert(generate_physical_solid_map_components());
    }
}

pub fn setup_goal_cells(mut commands: Commands, solids: Query<Entity, Added<Goal>>) {
    for e in solids.iter() {
        commands
            .entity(e)
            .insert(generate_physical_sensor_map_components());
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
