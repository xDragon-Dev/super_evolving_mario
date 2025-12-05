use bevy::prelude::*;

use crate::mario::*;

// ! GRAVE ERROR DE IMPLEMENTACIÓN, LA CAMARA NUNCA ES CONFIGURADA PARA FUNCIONAR RESPECTO A LAS DIMENSIONES
// ! DEL MAPA IMPORTADO A TRAVÉS DE LDTK, SE RECOMIENDA REFACTORIZACIÓN PARA FUNCIONAR INDEPENDIENTE DEL MAPA
// ! QUE SEA IMPORTADO, LOS VALORES ACTUALES DE INICIALIZACIÓN DE LA CAMARA SON ESTIMADOS (CALCULADOS A OJO)
// ! PARA VERSE LIGERAMENTE BIEN Y ALINEADOS AL MAPA IMPORTADO LDTK
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.25, // ! VALOR OBTENIDO A PRUEBA Y ERROR, PROBANDO QUE SE VE MEJOR Y OCULTA MEJOR LOS ERRORES
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_translation(Vec3::new(225.0, -110.0, 1.0)),
    ));
}

// ! FALLA DE IMPLEMENTACIÓN, LA POSICIÓN HORIZONTAL CAMBIA PERSIGUIENDO AL MARIO CON MAYOR POSICIÓN X
// ! PERO NO TIENE RANGOS DE ACCIÓN, COMO UN RANGO EN EL QUE EL MARIO CON MAYOR X SE PUEDA MOVER SIN AFECTAR
// ! LA CAMARA, O UN RANGO MAXIMO Y MINIMO POSIBLES A LOS CUALES SE PUEDE MOVER LA CAMARA (PARA NO VISUALIZAR
// ! FUERA DEL MAPA)
pub fn camera_follow_mario(
    marios: Query<&Transform, With<Mario>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Mario>)>,
) {
    if let Some(mario_transform) = marios
        .iter()
        .max_by(|a, b| a.translation.x.partial_cmp(&b.translation.x).unwrap())
    {
        if let Ok(mut cam_transform) = camera.single_mut() {
            cam_transform.translation.x = mario_transform.translation.x;
        }
    }
}

pub struct CameraTrackerPlugin;

impl Plugin for CameraTrackerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, camera_follow_mario)
            //.init_resource::<LevelAssetHandle>()
            //.init_resource::<LevelBounds>()
            ;
    }
}

/*

// ? use crate::level::*;
#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct LevelBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub scale: f32,
}

use bevy::asset::LoadState;

pub fn setup_camera_and_bounds(
    mut commands: Commands,
    level_handle_res: Res<LevelAssetHandle>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    asset_server: Res<AssetServer>,
    q_level: Query<(&Transform, &LevelIid), Without<MainCamera>>,
    mut camera_initialized: Local<bool>,
) {
    if *camera_initialized {
        return;
    }

    let project_handle: &Handle<LdtkProject> = &level_handle_res.0;

    match asset_server.load_state(project_handle) {
        LoadState::Loaded => {
            return;
        }
        _ => {}
    }
    if let (Ok((level_transform, level_iid)), Some(ldtk_project)) =
        (q_level.single(), ldtk_projects.get(project_handle))
    {
        if let Some(ldtk_level) = ldtk_project.get_raw_level_by_iid(&level_iid.as_str().into()) {
            //! MAL MUY MAL
            const CAMERA_SCALE: f32 = 3.25;
            let level_width = ldtk_level.px_wid as f32;
            let level_height = ldtk_level.px_hei as f32;

            let min_x = level_transform.translation.x - level_width / 2.0;
            let max_x = level_transform.translation.x + level_width / 2.0;
            let min_y = level_transform.translation.y - level_height / 2.0;
            let max_y = level_transform.translation.y + level_height / 2.0;

            commands.insert_resource(LevelBounds {
                min_x,
                max_x,
                min_y,
                max_y,
                scale: CAMERA_SCALE,
            });
            commands.spawn((
                Projection::Orthographic(OrthographicProjection {
                    scale: CAMERA_SCALE,
                    ..OrthographicProjection::default_2d()
                }),
                *level_transform,
                MainCamera,
            ));
            *camera_initialized = true;
        }
    }
}


pub fn _camera_follow_mario(
    marios: Query<&Transform, With<Mario>>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Mario>)>,
    level_bounds: Res<LevelBounds>,
) {
    if level_bounds.max_x == 0.0 { return; }
    let viewport_half_width = 1920.0 / 2.0 * level_bounds.scale;
    let viewport_half_height = 1080.0 / 2.0 * level_bounds.scale;

    if let Some(mario_transform) = marios
        .iter()
        .max_by(|a, b| a.translation.x.partial_cmp(&b.translation.x).unwrap())
    {
        if let Ok(mut cam_transform) = camera.single_mut() {
            let target_x = mario_transform.translation.x;
            let min_camera_x = level_bounds.min_x + viewport_half_width;
            let max_camera_x = level_bounds.max_x - viewport_half_width;
            cam_transform.translation.x = target_x.clamp(min_camera_x, max_camera_x);
            let center_y = (level_bounds.min_y + level_bounds.max_y) / 2.0;
            cam_transform.translation.y = center_y.clamp(level_bounds.min_y + viewport_half_height, level_bounds.max_y - viewport_half_height);
        }
    }
}

// ? .init_resource::<LevelAssetHandle>()
// ? .init_resource::<LevelBounds>()
*/
