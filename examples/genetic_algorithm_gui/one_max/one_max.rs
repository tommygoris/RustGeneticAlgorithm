use crate::problem_settings::ProblemSettings;
use crate::{Model, Step};

use genetic_algorithm::crossover::genome_crossover::Crossover;
use genetic_algorithm::genome::fitness_function::FitnessFunction;
use genetic_algorithm::genome::population::{Individual, Population, ProblemType};
use genetic_algorithm::mutation::genome_mutation::{Mutate, StringMutation};
use genetic_algorithm::selection::genome_selection::SelectIndividual;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::thread::Thread;

#[derive(Default, Copy, Clone, Debug)]
struct OneMaxFitnessFunction;

struct InternalState {
    current_gen: u64,
    is_started: bool,
    crossover: Box<dyn Crossover<T = String> + Send + Sync>,
    selector: Box<dyn SelectIndividual<T = String> + Send + Sync>,
    mutation: Box<dyn Mutate<T = String> + Send + Sync>,
    start_thread: Option<JoinHandle<()>>,
}

pub struct OneMax {
    internal_state: Arc<InternalState>,
}
impl OneMax {
    pub fn new(
        current_gen: u64,
        is_started: bool,
        crossover: Box<dyn Crossover<T = String> + Send + Sync>,
        selector: Box<dyn SelectIndividual<T = String> + Send + Sync>,
        mutation: Box<dyn Mutate<T = String> + Send + Sync>,
        start_thread: Option<JoinHandle<()>>,
    ) -> OneMax {
        let internal_state = Arc::new(InternalState {
            current_gen,
            is_started,
            crossover,
            selector,
            mutation,
            start_thread,
        });

        OneMax { internal_state }
    }
}

impl ProblemSettings for OneMax {
    fn on_start(&mut self, model: &Model) {
        let mut one_max = self.deref().clone();
        let mut pop = init_string_pop(
            model.seed,
            model.population_size,
            Box::new(OneMaxFitnessFunction::default()),
        );
        let steps = model.steps.clone();
        let handler = thread::spawn(move || match steps {
            Step::Inf => loop {
                step_one(pop.borrow_mut(), &mut one_max)
            },
            Step::Steps(num_step) => {
                for _ in 0..num_step {
                    step_one(pop.borrow_mut(), &mut one_max)
                }
            }
        });
        self.internal_state.start_thread = Option::from(handler);
    }

    fn on_pause(&mut self, model: &Model) {
        unimplemented!()
    }

    fn on_stop(&mut self, model: &Model) {
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

fn step_one(pop: &mut Population<String>, one_max: &mut OneMax) {
    pop.crossover(
        one_max.internal_state.crossover.as_mut(),
        one_max.internal_state.selector.as_mut(),
        Box::new(OneMaxFitnessFunction::default()),
    );
    pop.mutate(
        one_max.internal_state.mutation.as_mut(),
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

    for _ in 0..pop_size {
        let new_string_individual = generate_string_individual_one_max(string_len, seed);
        let fitness = one_max_problem.calculate_fitness(&new_string_individual);
        population_list.push(Individual::new(new_string_individual, fitness));
    }

    let population = Population::new(population_list, ProblemType::Max);
    population
}

fn generate_string_individual_one_max(range: u32, seed: [u8; 32]) -> String {
    let mut new_string_individual = String::new();
    let characters = vec!['0', '1'];
    let mut seed_gen: StdRng = SeedableRng::from_seed(seed);
    for _ in 0..range {
        let location = seed_gen.gen_range(0, characters.len()) as usize;
        new_string_individual.push(characters[location]);
    }
    new_string_individual
}

//fn one_step(population: &mut Population<String>) {
//
//    if (isInitializing) {
//        let selection = TournamentSelection::new(7,0.80, *seed);
//        let mut crossover = StringCrossover::new(0.80, 7, *seed);
//        let mut one_max_problem = OneMax::default();
//        let mut selection = TournamentSelection::new(7,0.80, *seed);
//
//    }
//    population.crossover(crossover, *selection, fitness_function);
//    //population.mutate();
//}
