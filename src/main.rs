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
#[cfg(debug_assertions)]
use std::time::Duration;
use std::path::PathBuf;
use azul::window::FakeWindow;

macro_rules! css_path {() => { concat!(env!("CARGO_MANIFEST_DIR"), "/src/app.css")};}
//macro_rules! FONT_PATH {() => { concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/fonts/KoHo-Light.ttf")};}

#[cfg(debug_assertions)]
struct DataModel {
    counter: usize,
    text_input: TextInputState,
    //label: Label,
}


impl Layout for DataModel {
    fn layout(&self, mut _info: LayoutInfo<Self>) -> Dom<Self> {

        fn add_population_text_box(window: &mut FakeWindow<DataModel>, text_input: &TextInputState, data_model: &DataModel, label_text: String) -> Dom<DataModel> {

            let text_input = TextInput::new()
                .bind(window, &text_input, &data_model)
                .dom(&text_input)
                .with_class("test");

            let label2 = Label::new(label_text).dom().with_class("left_align_label_text");

            let label_div = Dom::div()
                .with_class("label_with_textbox")
                .with_child(text_input)
                .with_child(label2);

            Dom::div()
                .with_child(label_div)
        }
        //add_population_text_box(_info, &self.text_input, &self)
        Dom::div()
            .with_class("orange")
            .with_child(add_population_text_box(_info.window, &self.text_input, &self, String::from("Population:")))
            .with_child(add_population_text_box(_info.window, &self.text_input, &self, String::from("Crossover Rate:")))
            .with_child(add_population_text_box(_info.window, &self.text_input, &self, String::from("Mutation Rate:")))
            .with_child(add_population_text_box(_info.window, &self.text_input, &self, String::from("Selection Rate:")))
            .with_child(add_population_text_box(_info.window, &self.text_input, &self, String::from("Solution Fitness:")))
            //.with_child(TextInput::new().dom(&self.text_input))
            //.with_child(Dom::label(String::from("hello2")))
            //.with_child(Dom::label(String::from("hello3")))
//        TextInput::new()
//            .bind(_info.window, &self.text_input, &self)
//            .dom(&self.text_input)
//            .with_class("test")
    }
}

fn main() {
    let mut file_path = PathBuf::new();
    file_path.push("C:\\Users\\goris\\CLionProjects\\genetic_algorithm\\src\\app.css");
    let mut css = css::hot_reload_override_native(file_path, Duration::from_millis(100));
    let mut app = App::new(DataModel { counter: 0, text_input: TextInputState::default() }, AppConfig::default()).unwrap();
    let window = app.create_hot_reload_window(WindowCreateOptions::default(), css).unwrap();
    app.run(window).unwrap();
    let x = 1;



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
