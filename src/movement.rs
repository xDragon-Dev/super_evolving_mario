pub mod action_schedule_movement {
    use std::{collections::VecDeque, time::Duration};

    use bevy::prelude::*;
    use bevy_rapier2d::prelude::*;
    use rand::Rng;
    use serde::Serialize;

    use crate::mario::*;
    use crate::sprite::*;

    // ! CREO QUE EXISTE UNA MEJOR FORMA DE HACERLO PERO YA AHORITA DEJALO ASÍ
    use crate::genetic_algorithm::AgentState;

    // ? ADVERTENCIA NO ES UNA CREACIÓN DE ESTADO DE MARIO BROS EXTRA
    // ? DEFINE ESTADO DE UNA ACCIÓN, NO ESTADO GENERAL DE LA ENTIDAD
    #[derive(Serialize, Clone, Copy, Debug)]
    pub enum MarioMovement {
        MoveLeft,
        MoveRight,
        Jump,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct MarioAction {
        pub time_point: f32,
        pub duration: f32,
        pub movement: MarioMovement,
    }

    #[derive(Debug)]
    pub enum MovementState {
        Press,
        Release,
    }

    #[derive(Debug)]
    pub struct PendingAction {
        movement: MarioMovement,
        movement_state: MovementState,
        remaining_time: f32,
    }

    impl MarioMovement {
        pub fn random() -> Self {
            match rand::random_range(0..=2) {
                0 => Self::Jump,
                1 => Self::MoveRight,
                2 => Self::MoveLeft,
                _ => Self::random(),
            }
        }
    }

    impl MarioAction {
        pub fn new_random() -> Self {
            let mut rand = rand::rng();
            let movement = MarioMovement::random();

            // ! SE IMPLEMENTA ASÍ PARA EVITAR VOLAR, PERO TODAVÍA SE REQUIERE CORREGIR LAS FISICAS
            // ! INCLUSO CON ESTE TRUCO LOS PERSONAJES TODAVÍA PUEDEN VOLAR, CORREGIR FISICAS URGENTE
            let (time_point, duration) = match movement {
                MarioMovement::Jump => (rand.random_range(0.0..40.0), 0.0),
                _ => (rand.random_range(0.0..40.0), rand.random_range(1.0..3.0)),
            };

            Self {
                time_point,
                duration,
                movement,
            }
        }
    }

    impl ActiontSet {
        pub fn new_random() -> Self {
            let mut rand = rand::rng();
            let mut dna = Vec::<MarioAction>::new();
            let range: u8 = rand.random_range(25..40);
            let mut mario_action: MarioAction;
            for _ in 0..range {
                mario_action = MarioAction::new_random();
                dna.push(mario_action);
            }
            Self(dna)
        }
    }

    impl std::convert::From<ActiontSet> for ActionSchedule {
        fn from(value: ActiontSet) -> Self {
            let mut all_events = Vec::<(MarioMovement, MovementState, f32)>::new();

            for mario_action in &value.0 {
                all_events.push((
                    mario_action.movement,
                    MovementState::Press,
                    mario_action.time_point,
                ));
                all_events.push((
                    mario_action.movement,
                    MovementState::Release,
                    mario_action.time_point + mario_action.duration,
                ));
            }
            all_events.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
            let mut pending_actions = VecDeque::<PendingAction>::new();
            let mut current_time = 0.0;
            for event in all_events {
                let time_difference = event.2 - current_time;
                let pending_action = PendingAction {
                    movement: event.0,
                    movement_state: event.1,
                    remaining_time: time_difference,
                };
                pending_actions.push_back(pending_action);
                current_time = event.2;
            }
            let first_delay = pending_actions.front().map_or(0.0, |pa| pa.remaining_time);
            Self {
                next_action_timer: Timer::from_seconds(first_delay, TimerMode::Once),
                pending_actions,
            }
        }
    }

    #[derive(Component, Debug)]
    pub struct ActionSchedule {
        pub next_action_timer: Timer,
        pub pending_actions: VecDeque<PendingAction>,
    }

    #[derive(Component, Serialize, Clone, Debug)]
    pub struct ActiontSet(pub Vec<MarioAction>);

    #[derive(Component, Default, Debug)]
    pub struct MarioCurrentActions {
        moving_left: bool,
        moving_right: bool,
        jumping: bool,
    }

    pub fn update_mario_movement(
        mut q_mario: Query<(&mut Velocity, &MarioCurrentActions, &AgentState), With<Mario>>,
    ) {
        let horizontal_speed = 200.0;
        let jump_speed = 400.0;

        for (mut vel, actions, agent_state) in &mut q_mario {
            // ! USO DE AGENT STATE, POSIBLE REFACTORIZACÓN A FUTURO
            if agent_state.finished {
                continue;
            }

            if actions.moving_left && !actions.moving_right {
                vel.linvel.x = -horizontal_speed;
            } else if actions.moving_right && !actions.moving_left {
                vel.linvel.x = horizontal_speed;
            } else {
                vel.linvel.x = 0.0;
            }
            if actions.jumping {
                vel.linvel.y = jump_speed;
            }
        }
    }

    pub fn execute_action_schedule(
        time: Res<Time>,
        mut q_mario: Query<
            (
                &mut ActionSchedule,
                &mut MarioCurrentActions,
                &mut AgentState,
            ),
            With<Mario>,
        >,
    ) {
        for (mut schedule, mut current_actions, mut agent_state) in &mut q_mario {
            // ! USO DE AGENT STATE, POSIBLE REFACTORIZACÓN A FUTURO
            if agent_state.finished {
                continue;
            }
            if schedule
                .next_action_timer
                .tick(time.delta())
                .just_finished()
            {
                let next_action = match schedule.pending_actions.pop_front() {
                    Some(action) => action,
                    None => {
                        agent_state.finished = true;
                        continue;
                    }
                };
                let new_state = match next_action.movement_state {
                    MovementState::Press => true,
                    MovementState::Release => false,
                };
                match next_action.movement {
                    MarioMovement::MoveLeft => {
                        current_actions.moving_left = new_state;
                    }
                    MarioMovement::MoveRight => {
                        current_actions.moving_right = new_state;
                    }
                    MarioMovement::Jump => {
                        current_actions.jumping = new_state;
                    }
                }
                let next_delay = schedule
                    .pending_actions
                    .front()
                    .map_or(0.0, |next_event| next_event.remaining_time);
                schedule
                    .next_action_timer
                    .set_duration(Duration::from_secs_f32(next_delay));
                schedule.next_action_timer.reset();
            }
        }
    }

    pub fn update_mario_sprite_state(
        mut states: Query<
            (&mut MarioGlobalState, &MarioCurrentActions, &mut AgentState),
            With<Mario>,
        >,
    ) {
        for (mut state, current_action, agent_state) in &mut states {
            // ! USO DE AGENT STATE, POSIBLE REFACTORIZACÓN A FUTURO
            if agent_state.finished {
                continue;
            }

            if current_action.jumping {
                *state = MarioGlobalState::Jump
            } else if current_action.moving_left
                || current_action.moving_right
                    && !(current_action.moving_right && current_action.moving_left)
            {
                *state = MarioGlobalState::Walk
            } else if current_action.moving_right && current_action.moving_left
                || (!current_action.moving_right && !current_action.moving_left)
            {
                *state = MarioGlobalState::Idle
            }
        }
    }

    pub fn update_mario_sprite_facing_direction(
        mut sprites: Query<
            (&mut FacingDirection, &MarioCurrentActions, &mut AgentState),
            With<Mario>,
        >,
    ) {
        for (mut facing_direction, current_action, agent_state) in &mut sprites {
            // ! USO DE AGENT STATE, POSIBLE REFACTORIZACÓN A FUTURO
            if agent_state.finished {
                continue;
            }

            if current_action.moving_left && !current_action.moving_right {
                *facing_direction = FacingDirection::Left;
            } else if current_action.moving_right && !current_action.moving_left {
                *facing_direction = FacingDirection::Right;
            }
        }
    }

    pub struct AutoMovementPlugin;

    impl Plugin for AutoMovementPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, update_mario_movement)
                .add_systems(Update, execute_action_schedule)
                .add_systems(Update, update_mario_sprite_state)
                .add_systems(Update, update_mario_sprite_facing_direction);
        }
    }

    pub fn generate_random_movement_components()
    -> (ActiontSet, ActionSchedule, MarioCurrentActions, AgentState) {
        let action_set = ActiontSet::new_random();
        let action_schedule = ActionSchedule::from(action_set.clone());
        let mario_current_actions = MarioCurrentActions::default();
        let agent_state = AgentState::default();

        (
            action_set,
            action_schedule,
            mario_current_actions,
            agent_state,
        )
    }
}

/*
mod keyboard_input_movement {
    use crate::mario::*;
    use crate::sprite::*;
    use bevy::prelude::*;
    use bevy_rapier2d::prelude::*;

    pub fn update_mario_state(
        mut states: Query<&mut MarioGlobalState, With<Mario>>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        for mut state in &mut states {
            if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::ArrowRight]) {
                *state = MarioGlobalState::Walk;
            } else if keys.pressed(KeyCode::Space) {
                *state = MarioGlobalState::Jump;
            } else {
                *state = MarioGlobalState::Idle;
            }
        }
    }

    pub fn update_mario_facing_direction(
        mut sprites: Query<&mut FacingDirection, With<Mario>>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        for mut facing_direction in &mut sprites {
            if keys.pressed(KeyCode::ArrowRight) {
                *facing_direction = FacingDirection::Right;
            } else if keys.pressed(KeyCode::ArrowLeft) {
                *facing_direction = FacingDirection::Left;
            }
        }
    }

    pub fn mario_horizontal_move(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut q: Query<&mut Velocity, With<Mario>>,
    ) {
        let speed = 200.0;

        for mut vel in &mut q {
            if keyboard.pressed(KeyCode::ArrowLeft) {
                vel.linvel.x = -speed;
            } else if keyboard.pressed(KeyCode::ArrowRight) {
                vel.linvel.x = speed;
            } else {
                vel.linvel.x = 0.0;
            }
        }
    }

    pub fn mario_jump(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut q: Query<&mut Velocity, With<Mario>>,
    ) {
        let jump_speed = 400.0;

        for mut vel in &mut q {
            if keyboard.just_pressed(KeyCode::Space) {
                vel.linvel.y = jump_speed;
            }
        }
    }

    pub struct InputMovementPlugin;

    impl Plugin for InputMovementPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, update_mario_state)
                .add_systems(Update, update_mario_facing_direction)
                .add_systems(Update, mario_horizontal_move)
                .add_systems(Update, mario_jump);
        }
    }
}
 */