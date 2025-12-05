use crate::mario::*;
use crate::movement::action_schedule_movement::*;
use crate::physics::*;

use bevy::prelude::*;
use rand::prelude::*;

// --- ESTADOS Y RECURSOS DE CONTROL ---

#[derive(Resource)]
pub struct GeneticAlgorithmConfig {
    pub population_size: u32,
    pub mutation_rate: f32,
    pub crossover_rate: f32,
    pub generations: u32,
    pub elitism: u8,
    pub tournament_k: u8,
}

#[derive(Resource, Default)]
pub struct GenerationTracker {
    pub current_generation: u32,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GeneticAlgorithmState {
    /// Esperando a que los assets carguen e iniciar la primera población.
    #[default]
    WaitingToStart,
    /// Los agentes están vivos, moviéndose y siendo evaluados.
    RunningGeneration,
    /// Todos los agentes han terminado/muerto. Es hora de crear la siguiente generación.
    GenerationComplete,
    /// El número máximo de generaciones ha sido alcanzado.
    Finished,
}

// --- COMPONENTES ---

#[derive(Component)]
pub struct AgentState {
    pub fitness: f32,
    pub finished: bool, // true si murió o terminó el nivel
}

impl std::default::Default for AgentState {
    fn default() -> Self {
        Self {
            fitness: 0.0,
            finished: false,
        }
    }
}

impl std::default::Default for GeneticAlgorithmConfig {
    fn default() -> Self {
        Self {
            population_size: 200,
            mutation_rate: 0.2,
            crossover_rate: 0.9,
            generations: 500,
            elitism: 2,
            tournament_k: 3,
        }
    }
}

// --- SISTEMAS DEL ALGORITMO GENÉTICO ---

/// Inicializa la primera población, transicionando de WaitingToStart a RunningGeneration.
pub fn generate_initial_population(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    genetic_algorithm_config: Res<GeneticAlgorithmConfig>,
    mut next_ga_state: ResMut<NextState<GeneticAlgorithmState>>,
) {
    // 💥 NOTA: Debes asegurarte de que "Small_mario.png" ya esté cargado antes de llamar a esto
    let mario_texture: Handle<Image> = asset_server.load("Small_mario.png"); 
    let layout = TextureAtlasLayout::from_grid(UVec2::new(17, 16), 7, 1, None, None); 
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layout.add(layout);

    for _ in 0..genetic_algorithm_config.population_size {
        commands.spawn((
            generate_mario_entity(mario_texture.clone(), texture_atlas_layout.clone()),
            generate_physical_entity_components(),
            generate_random_movement_components(),
        ));
    }
    info!("Población inicial generada.");
    next_ga_state.set(GeneticAlgorithmState::RunningGeneration);
}


pub fn update_agent_fitness(mut q_mario: Query<(&Transform, &mut AgentState), With<Mario>>) {
    for (transform, mut agent_state) in &mut q_mario {
        // Solo actualizar si el agente aún no ha terminado.
        if !agent_state.finished {
            let current_distance = transform.translation.x;
            if current_distance > agent_state.fitness {
                agent_state.fitness = current_distance;
            }
            // NOTA: Debes tener otro sistema que ponga finished=true
            // (e.g., al colisionar con un enemigo o caer fuera del mundo).
        }
    }
}

/// Sistema que comprueba si todos los agentes han terminado o muerto.
pub fn check_generation_end(
    q_agents: Query<&AgentState, With<Mario>>,
    mut next_ga_state: ResMut<NextState<GeneticAlgorithmState>>,
) {
    if q_agents.is_empty() {
        // La población no se ha spawneado aún, ignorar.
        return;
    }

    // Comprueba si CUALQUIER agente NO ha terminado.
    let any_agent_running = q_agents.iter().any(|state| !state.finished);

    if !any_agent_running {
        // Todos los agentes tienen finished = true.
        info!("Generación terminada. Transicionando a GenerationComplete.");
        next_ga_state.set(GeneticAlgorithmState::GenerationComplete);
    }
}


/// Maneja la transición entre generaciones o el final del GA.
pub fn transition_generations(
    commands: Commands,
    ga_config: Res<GeneticAlgorithmConfig>,
    mut tracker: ResMut<GenerationTracker>,
    mut next_ga_state: ResMut<NextState<GeneticAlgorithmState>>,
    q_mario: Query<(Entity, &ActiontSet, &AgentState, &Transform), With<Mario>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    tracker.current_generation += 1;
    
    info!(
        "Iniciando transición. Generación actual: {}",
        tracker.current_generation
    );

    // 1. CONDICIÓN DE PARADA
    if tracker.current_generation > ga_config.generations {
        info!("Algoritmo Genético completado después de {} generaciones.", ga_config.generations);
        next_ga_state.set(GeneticAlgorithmState::Finished);
        return;
    }

    // 2. CREAR Y SPAWNEAR NUEVA GENERACIÓN
    create_next_generation(
        commands,
        ga_config,
        q_mario,
        asset_server,
        texture_atlas_layout,
    );

    // 3. Volver a RUNNING
    next_ga_state.set(GeneticAlgorithmState::RunningGeneration);
}

// ------------------------------------------------------------------------------------------------
// NOTA: Las funciones select_tournament, crossover, mutate y create_next_generation
//       (con ligeras modificaciones en la firma de create_next_generation) 
//       se mantienen sin cambios importantes en su lógica interna.
// ------------------------------------------------------------------------------------------------

// ******* Lógica de selección, cruce, mutación (Mantenida de tu código) *******
// (Funciones select_tournament, crossover, mutate, create_next_generation)

pub fn select_tournament(population: &[(&ActiontSet, &AgentState)], k: u8) -> ActiontSet {
    let mut rng = rand::rng(); // Corregido: Usar thread_rng para un generador local
    let mut best_agent: Option<(&ActiontSet, &AgentState)> = None;
    for _ in 0..k {
        let current_agent = population.choose(&mut rng).unwrap();
        if best_agent.is_none() || current_agent.1.fitness > best_agent.unwrap().1.fitness {
            best_agent = Some(*current_agent);
        }
    }
    best_agent.unwrap().0.clone()
}

pub fn crossover(parent1: &ActiontSet, parent2: &ActiontSet) -> ActiontSet {
    let mut rng = rand::rng();
    let dna1 = &parent1.0;
    let dna2 = &parent2.0;
    let min_len = dna1.len().min(dna2.len());
    if min_len < 2 {
        return parent1.clone();
    }
    // Corregido: rng.gen_range(min..max) para obtener rango
    let crossover_point = rng.random_range(1..min_len); 
    let mut child_actions = Vec::new();
    child_actions.extend_from_slice(&dna1[0..crossover_point]);
    child_actions.extend_from_slice(&dna2[crossover_point..]);
    ActiontSet(child_actions)
}

/// Aplica mutación al ActiontSet con una probabilidad dada.
pub fn mutate(action_set: &mut ActiontSet, mutation_rate: f32) {
    let mut rng = rand::rng();
    for action in action_set.0.iter_mut() {
        if rng.random_bool(mutation_rate as f64) {
            match rng.random_range(0..3) {
                0 => {
                    action.movement = match action.movement {
                        MarioMovement::MoveLeft => MarioMovement::MoveRight,
                        MarioMovement::MoveRight => MarioMovement::Jump,
                        MarioMovement::Jump => MarioMovement::MoveLeft,
                    };
                }
                1 => {
                    // Corregido: Usar gen_range
                    action.time_point = (action.time_point + rng.random_range(-0.5..0.5)).max(0.0);
                }
                _ => {
                    action.duration = (action.duration + rng.random_range(-0.2..0.2)).max(0.05);
                }
            }
        }
    }
}

pub fn create_next_generation(
    mut commands: Commands,
    ga_config: Res<GeneticAlgorithmConfig>,
    q_mario: Query<(Entity, &ActiontSet, &AgentState, &Transform), With<Mario>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::rng();
    let population: Vec<(&ActiontSet, &AgentState)> = q_mario
        .iter()
        .map(|(_, dna, state, _)| (dna, state))
        .collect();
        
    // Generación de Assets (Optimización: podrías mover esto al setup)
    let mario_texture: Handle<Image> = asset_server.load("Small_mario.png"); 
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layout.add(TextureAtlasLayout::from_grid(
            UVec2::new(17, 16),
            7,
            1,
            None,
            None,
        ));

    let mut sorted_population = q_mario.iter().collect::<Vec<_>>();
    sorted_population.sort_unstable_by(|a, b| {
        b.2.fitness
            .partial_cmp(&a.2.fitness)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let elitism_count = ga_config.elitism as usize;
    let elite_agents = &sorted_population[0..elitism_count];
    let mut new_population_dna: Vec<ActiontSet> = Vec::new();
    for (_, dna, _, _) in elite_agents {
        new_population_dna.push((*dna).clone());
    }

    while new_population_dna.len() < ga_config.population_size as usize {
        let mut child_dna: ActiontSet;
        if rng.random_bool(ga_config.crossover_rate as f64) {
            let parent1_dna = select_tournament(&population, ga_config.tournament_k);
            let parent2_dna = select_tournament(&population, ga_config.tournament_k);
            child_dna = crossover(&parent1_dna, &parent2_dna);
        } else {
            child_dna = select_tournament(&population, ga_config.tournament_k);
        }
        if rng.random_bool(ga_config.mutation_rate as f64) {
            mutate(&mut child_dna, ga_config.mutation_rate);
        }
        new_population_dna.push(child_dna);
    }
    
    // 💥 DESPAWN DE LA GENERACIÓN VIEJA
    for (entity, _, _, _) in q_mario.iter() {
        commands.entity(entity).despawn();
    }

    // 💥 SPAWN DE LA NUEVA GENERACIÓN
    for dna in new_population_dna {
        commands.spawn((
            generate_mario_entity(mario_texture.clone(), texture_atlas_layout.clone()),
            generate_physical_entity_components(),
            dna.clone(),
            ActionSchedule::from(dna),
            MarioCurrentActions::default(),
            AgentState::default(), // El nuevo agente comienza sin terminar
        ));
    }
}
// ******* FIN de Lógica de selección, cruce, mutación *******


pub struct GeneticAlgorithmPlugin;

impl Plugin for GeneticAlgorithmPlugin {
    fn build(&self, app: &mut App) {
        // Inicializar estados y recursos
        app.init_state::<GeneticAlgorithmState>()
            .insert_resource(GeneticAlgorithmConfig::default())
            .init_resource::<GenerationTracker>();

        // 1. Inicialización de la primera población (solo se ejecuta una vez)
        app.add_systems(
            Startup,
            // NOTA: Recomiendo crear un sistema para esperar que "Small_mario.png" cargue antes de este.
            generate_initial_population.run_if(in_state(GeneticAlgorithmState::WaitingToStart)),
        );

        // 2. Ejecución de la Generación
        app.add_systems(
            Update,
            (
                // 2a. Actualiza la aptitud de los agentes vivos
                update_agent_fitness, 
                // 2b. Comprueba si todos han terminado. Si es así, transiciona.
                check_generation_end,
            )
                .run_if(in_state(GeneticAlgorithmState::RunningGeneration)),
        );

        // 3. Transición de Generación
        app.add_systems(
            Update,
            // Llama a la lógica de selección/crossover y transiciona de vuelta a Running.
            transition_generations.run_if(in_state(GeneticAlgorithmState::GenerationComplete)),
        );
        
        // 4. Finalización
        app.add_systems(
            Update,
            // Opcional: Sistema para limpiar o mostrar el resultado final
            (|mut tracker: ResMut<GenerationTracker>| {
                if tracker.current_generation > 0 {
                    info!("GA Finalizado en Generación {}", tracker.current_generation - 1);
                    tracker.current_generation = 0; // Evita spam
                }
            })
            .run_if(in_state(GeneticAlgorithmState::Finished))
        );
    }
}