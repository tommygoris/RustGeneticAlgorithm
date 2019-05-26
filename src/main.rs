extern crate azul;
mod crossover;
mod genome;
mod mutation;
mod selection;

use azul::{prelude::*, widgets::{label::Label, button::Button}};
use crate::selection::genome_selection::{TournamentSelection};
use crate::crossover::genome_crossover::{StringCrossover};
use crate::mutation::genome_mutation::StringMutation;
use crate::genome::population::{Individual, Population, ProblemType};
use rand::Rng;
use rand::prelude::*;
use crate::genome::problem::{OneMax, FitnessFunction};
use azul::widgets::text_input::{TextInput, TextInputState};
use azul::dom::NodeType::Text;

struct DataModel {
    counter: usize,
    text_input: TextInputState,
    //label: Label,
}


impl Layout for DataModel {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        Dom::label(String::from("Population"))
//        TextInput::new()
//            .bind(_info.window, &self.text_input, &self)
//            .dom(&self.text_input)
//            .with_id("text_input_1")
    }
}

fn main() {
    let mut app = App::new(DataModel { counter: 0, text_input: TextInputState::default() }, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();



//    let seed: &[u8; 32] = &[1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
//    let mut one_max_problem = OneMax::default();
//    let mut pop= init_string_pop(*seed, &mut one_max_problem);
//
//    //one_step(&mut pop, &mut crossover, &mut selection, &mut one_max_problem);
//
//    let mut selection = TournamentSelection::new(7, 0.80, *seed);
//    let mut crossover = StringCrossover::new(0.80, 7, *seed);
//    let mut mutation = StringMutation::new(0.05, vec!['0', '1'], *seed);
//    let max_fitness = &100.0;
//    loop {
//        pop.crossover(&mut crossover, &mut selection, Box::new(one_max_problem));
//        pop.mutate(&mut mutation, Box::new(one_max_problem));
//        let individual = pop.find_top_individual();
//        println!("{:?}", individual);
//        if individual.fitness() == max_fitness {
//            break;
//        }
//    }
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

fn init_string_pop(seed: [u8; 32], one_max_problem: &mut OneMax) -> Population<String> {
    let population_amount = 100;
    let string_len = 100;

    let mut population_list = Vec::new();

    for _ in 0..population_amount {
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
