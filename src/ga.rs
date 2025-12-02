use bevy::prelude::*;


use bevy::prelude::*;

// Definición de las acciones que componen el genoma (DNA)
#[derive(Debug, Clone, Copy)]
pub enum MarioAction {
    MoveHorizontal(f32), // -1.0, 0.0, 1.0
    Jump,
    Wait(f32), // Duración en segundos
}

/// Componente que guarda la secuencia completa del genoma.
#[derive(Component, Clone)]
pub struct DNA {
    pub actions: Vec<MarioAction>,
}

/// Componente que guarda el estado de la simulación y el desempeño del agente.
#[derive(Component)]
pub struct AgentState {
    /// Puntero a la acción actual en `DNA.actions`.
    pub execution_pointer: usize,
    /// El puntaje: Distancia máxima recorrida en X, o número de monedas, etc.
    pub fitness: f32,
    /// Indica si el agente ha completado su simulación (murió, o se acabó el tiempo/acciones).
    pub finished: bool,
}


#[derive(Resource)]
pub struct GeneticAlgorithmConfig{
    population_size: u32,
    mutation_rate: f32,
    crossover_rate: f32,
    generations: u32,
    elitism: u8,
    tournament_k: u8
}

impl std::default::Default for GeneticAlgorithmConfig {
    fn default() -> Self {
        Self { population_size: 200, mutation_rate: 0.2, crossover_rate: 0.9, generations: 500, elitism: 2, tournament_k: 3 }
    }
}

pub fn generate_initial_population(comands: Commands, ga_config: Res<GeneticAlgorithmConfig>){

}

/* 
Implementaciones faltantes
Función fitnes (calculo de que tan lejos llegó un individuo)
selección de torneo (seleccionar k individuos de la población y de esos escoger al mejor)
crossover (generar un nuevo set de movimientos a partir de 2 padres, con una posibilidad de crossover rate)
mutación (agarrar un set de movimientos y hacer una pequeña modificacion esperando un posible mejor resultado)


en un flujo normal se haría lo siguiente, crear una población de {population size}
escoger solo los {elitism} mejores para la siguiente generación (pase directo)
nueva_población.agregar_individuo({elitism} individuos_de_la_nueva_generación)
al resto de la población:
si te tocó crossover (con una posibilidad de {crossover_rate}):
    padre1 = selección de torneo(population, TOURNAMENT_K) //que viva el mejor, 1 de {tournament_k}
    padre2 = selección de torneo(population, TOURNAMENT_K) //que viva el mejor, 1 de {tournament_k}
    individuo_nueva_generación = crossover(padre1, padre2)
else:
    individuo_nueva_generación = tournament_selection(population, TOURNAMENT_K) //Pasará cualquiera que viva el torneo
si te tocó mutación (con una posibilidad de {mutation_rate}):
    individuo_nueva_generación = mutacion(individuo_nueva_generación)

nueva_población.agregar_individuo(individuo_nueva_generación)
*/

struct GeneticAlgorithmPlugin;

impl Plugin for GeneticAlgorithmPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneticAlgorithmConfig::default());
    }
}