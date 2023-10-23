use crate::neural_network::{NeuralNetwork, NeuralNetworkFactory};

type KCAL = f64;

const MAX_SLEEPINESS: f64 = 100.0;

/// Factory for creating new creatures, will house the logic for creating
/// offspring and mutating them.
pub struct CreatureFactory {
    starting_energy: KCAL,
    starting_size: f64,

    starting_health: f64,

    neural_network_factory: NeuralNetworkFactory,
    merging_strategy: fn(f64, f64) -> f64,
}

impl CreatureFactory {
    pub fn new() -> CreatureFactory {
        CreatureFactory {
            starting_energy: 1000.0,
            starting_size: 1.0,
            starting_health: 100.0,
            neural_network_factory: NeuralNetworkFactory::new(&[3, 3, 2], 0.1),
            merging_strategy: |a, b| 0.5 * a + 0.5 * b,
        }
    }

    pub fn create_random(&self) -> Creature {
        Creature {
            energy: self.starting_energy,
            age: 0.0,
            sleepiness: 0.0,
            hunger: 0.0,
            health: self.starting_health,
            max_speed: 1.0,
            size: self.starting_size,
            reach: 1.0,

            x: 0.0,
            y: 0.0,

            speed: 0.0,
            direction: 0.0,

            state: CreatureState::Awake,
            neural_network: self.neural_network_factory.create_random(),
        }
    }

    pub fn create_multiple_random(&self, n: usize) -> Vec<Creature> {
        (0..n).map(|_| self.create_random()).collect()
    }

    pub fn create_offspring(&self, parent1: &Creature, parent2: &Creature) -> Creature {
        Creature {
            energy: self.starting_energy,
            age: 0.0,
            sleepiness: 0.0,
            hunger: 0.0,
            health: self.starting_health,
            max_speed: (self.merging_strategy)(parent1.max_speed, parent2.max_speed),
            size: self.starting_size,
            reach: (self.merging_strategy)(parent1.reach, parent2.reach),

            x: (self.merging_strategy)(parent1.x, parent2.x),
            y: (self.merging_strategy)(parent1.y, parent2.y),

            speed: 0.0,
            direction: 0.0,

            state: CreatureState::Awake,
            neural_network: self
                .neural_network_factory
                .create_offspring(&parent1.neural_network, &parent2.neural_network),
        }
    }

    pub fn create_offspring_mutated(&self, parent1: &Creature, parent2: &Creature) -> Creature {
        Creature {
            energy: self.starting_energy,
            age: 0.0,
            sleepiness: 0.0,
            hunger: 0.0,
            health: self.starting_health,
            max_speed: (self.merging_strategy)(parent1.max_speed, parent2.max_speed),
            size: self.starting_size,
            reach: (self.merging_strategy)(parent1.reach, parent2.reach),

            x: (self.merging_strategy)(parent1.x, parent2.x),
            y: (self.merging_strategy)(parent1.y, parent2.y),

            speed: 0.0,
            direction: 0.0,

            state: CreatureState::Awake,
            neural_network: self.neural_network_factory.create_offspring(
                &parent1.neural_network.mutate(0.1),
                &parent2.neural_network.mutate(0.1),
            ),
        }
    }
}

#[derive(PartialEq, Clone)]
enum CreatureState {
    Awake,
    Asleep,
    Dead,
    Eating,
}

#[derive(Clone)]
pub struct Creature {
    energy: KCAL,

    // Age increases the noise in the neural network
    age: f64,

    // Sleepiness increases the noise in the neural network
    sleepiness: f64,

    // Hunger increases the noise in the neural network
    hunger: f64,
    health: f64,
    max_speed: f64, // m/s
    reach: f64,     // m

    // Size increases the energy and hunger consumption
    size: f64,

    // Position
    x: f64,
    y: f64,

    // Movement
    speed: f64,
    direction: f64,

    state: CreatureState,

    neural_network: NeuralNetwork,
}

impl Creature {
    pub fn update(&mut self, dt: f64) {
        self.age += dt;
        self.energy -= dt * self.size * 10.0;
        self.sleepiness += dt * 0.1;
        self.hunger += dt * self.size * 0.1;

        if self.energy < 0.0 || self.health < 0.0 {
            self.state = CreatureState::Dead;
        }

        if self.sleepiness > MAX_SLEEPINESS {
            self.state = CreatureState::Asleep;
        }

        if self.state == CreatureState::Asleep {
            self.sleepiness -= dt * 10.0;
        }

        if self.state == CreatureState::Eating && self.hunger < 1.0 {
            self.state = CreatureState::Awake;
        }
    }
}

impl std::fmt::Display for Creature {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!(
            "Age: {} Energy: {} Sleepiness: {} Hunger: {}",
            self.age, self.energy, self.sleepiness, self.hunger
        );
        Ok(())
    }
}
