use crate::one_max;
use crate::problem_settings::ProblemSettings;
use crate::{Model, Step};
use crossbeam_utils::thread;
use genetic_algorithm::crossover::genome_crossover::Crossover;
use genetic_algorithm::genome::fitness_function::FitnessFunction;
use genetic_algorithm::genome::population::{Individual, Population, ProblemType};
use genetic_algorithm::mutation::genome_mutation::{Mutate, StringMutation};
use genetic_algorithm::selection::genome_selection::SelectIndividual;
use log::{info, warn, LevelFilter};
use rand::prelude::*;
use rand::rngs::StdRng;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use std::thread::Thread;
#[derive(Default, Copy, Clone, Debug)]
struct OneMaxFitnessFunction;

struct InternalState {
    current_gen: u64,
    crossover: Box<dyn Crossover<T = String> + Send + Sync>,
    selector: Box<dyn SelectIndividual<String> + Send + Sync>,
    mutation: Box<dyn Mutate<T = String> + Send + Sync>,
}

pub struct OneMax {
    internal_state: InternalState,
    is_started: bool,
    pop: Option<Population<String>>,
    seed: [u8; 32],
    population_size: u64,
}
impl OneMax {
    pub fn new(
        current_gen: u64,
        is_started: bool,
        crossover: Box<dyn Crossover<T = String> + Send + Sync>,
        selector: Box<dyn SelectIndividual<String> + Send + Sync>,
        mutation: Box<dyn Mutate<T = String> + Send + Sync>,
        seed: &[u8; 32],
        population_size: u64,
    ) -> OneMax {
        let internal_state = InternalState {
            current_gen,
            crossover,
            selector,
            mutation,
        };

        OneMax {
            internal_state,
            is_started,
            pop: None,
            seed: *seed,
            population_size,
        }
    }

    pub fn population(&mut self) -> &Option<Population<String>> {
        &self.pop
    }

    pub fn current_gen(&mut self) -> u64 {
        self.internal_state.current_gen
    }
}

impl ProblemSettings for OneMax {
    fn on_start(&mut self) {
        //        let mut one_max = self.clone();
        //        let mut one_max_lock = self.clone();
        if let Some(pop) = &mut self.pop {
            info!("old pop");
            step_one(pop.borrow_mut(), &mut self.internal_state);
        } else {
            info!("new pop");
            let mut pop = init_string_pop(
                self.seed,
                self.population_size,
                Box::new(OneMaxFitnessFunction::default()),
            );
            self.pop = Some(pop);
            step_one(&mut self.pop.clone().unwrap(), &mut self.internal_state)
        }
        self.internal_state.current_gen += 1;
    }

    fn on_pause(&mut self) {
        unimplemented!()
    }

    fn on_stop(&mut self) {
        unimplemented!()
    }
}

impl FitnessFunction for OneMaxFitnessFunction {
    type T = String;

    fn calculate_fitness(&mut self, individual: &String) -> f64 {
        let mut fitness = 0.0;
        for char in individual.chars() {
            if char.eq(&'1') {
                fitness += 1.0;
            }
        }
        fitness
    }
}

fn step_one(pop: &mut Population<String>, one_max: &mut InternalState) {
    pop.crossover(
        one_max.crossover.as_mut(),
        one_max.selector.as_mut(),
        Box::new(OneMaxFitnessFunction::default()),
    );

    pop.mutate(
        one_max.mutation.as_mut(),
        Box::new(OneMaxFitnessFunction::default()),
    );

    pop.print_pop();
}

fn init_string_pop(
    seed: [u8; 32],
    pop_size: u64,
    mut one_max_problem: Box<dyn FitnessFunction<T = String>>,
) -> Population<String> {
    let string_len = 100;

    let mut population_list = Vec::new();
    let mut seed_gen = SeedableRng::from_seed(seed);
    for _ in 0..pop_size {
        let new_string_individual = generate_string_individual_one_max(string_len, &mut seed_gen);
        let fitness = one_max_problem.calculate_fitness(&new_string_individual);
        population_list.push(Individual::new(new_string_individual, fitness));
    }

    let population = Population::new(population_list, ProblemType::Max);
    population
}

fn generate_string_individual_one_max(range: u32, seed_gen: &mut StdRng) -> String {
    let mut new_string_individual = String::new();
    let characters = vec!['0', '1'];

    for _ in 0..range {
        let location = seed_gen.gen_range(0, characters.len()) as usize;
        new_string_individual.push(characters[location]);
    }
    new_string_individual
}
