use crate::auto_movement::*;
use crate::mario::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::ActiveEvents;
use rand::prelude::*;

#[derive(Resource)]
pub struct GeneticAlgorithmConfig {
    population_size: u32,
    mutation_rate: f32,
    crossover_rate: f32,
    generations: u32,
    elitism: u8,
    tournament_k: u8,
}

#[derive(Component)]
pub struct AgentState {
    pub fitness: f32,
    pub finished: bool, // true si murió o terminó el nivel
}

impl std::default::Default for AgentState {
    fn default() -> Self {
        Self { fitness: 0.0, finished: false }
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

pub fn generate_initial_population(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    genetic_algorithm_config: Res<GeneticAlgorithmConfig>,
) {
    let mario_texture: Handle<Image> = asset_server.load("Small_mario.png"); //Existe y he llegado a crear una entidad visible con esto en el pasado
    let layout = TextureAtlasLayout::from_grid(UVec2::new(17, 16), 7, 1, None, None); //medidas correctas
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layout.add(layout);

    for _ in 0..genetic_algorithm_config.population_size {
        let id = commands
            .spawn(generate_mario_entity(
                mario_texture.clone(),
                texture_atlas_layout.clone(),
            ))
            .id();
        //Agregados unos componentes extras (para otras lógicas)
        let action_set = ActiontSet::new_random();
        let action_schedule = ActionSchedule::from(action_set.clone());
        let mario_current_actions = MarioCurrentActions::default();
        let agent_state = AgentState::default();
        commands.entity(id).insert((
            action_set,
            action_schedule,
            mario_current_actions,
            agent_state,
            ActiveEvents::COLLISION_EVENTS
        ));
    }
}

pub fn update_agent_fitness(mut q_mario: Query<(&Transform, &mut AgentState), With<Mario>>) {
    for (transform, mut agent_state) in &mut q_mario {
        let current_distance = transform.translation.x;
        if current_distance > agent_state.fitness {
            agent_state.fitness = current_distance;
        }
    }
}

pub fn select_tournament(population: &[(&ActiontSet, &AgentState)], k: u8) -> ActiontSet {
    let mut rng = rand::rng();
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
    let (mario_texture, texture_atlas_layout) = (
        asset_server.load("Small_mario.png"),
        texture_atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::new(17, 16),7,1,None,None))
    );
    
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
    for (entity, _, _, _) in q_mario.iter() {
        commands.entity(entity).despawn();
    }

    for dna in new_population_dna {
        let id = commands
            .spawn(generate_mario_entity(
                mario_texture.clone(),
                texture_atlas_layout.clone(),
            ))
            .id();
        commands.entity(id).insert((
            dna.clone(),
            ActionSchedule::from(dna),
            MarioCurrentActions::default(),
            AgentState {
                fitness: 0.0,
                finished: false,
            },
        ));
    }
}

pub struct GeneticAlgorithmPlugin;

impl Plugin for GeneticAlgorithmPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneticAlgorithmConfig::default())
            .add_systems(Startup, generate_initial_population);
    }
}
