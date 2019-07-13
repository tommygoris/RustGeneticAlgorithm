extern crate gtk;
extern crate sha2;
extern crate simple_logging;
#[macro_use]
extern crate arrayref;

use gio::prelude::*;
use gtk::prelude::*;
use log::{info, warn, LevelFilter};

use gtk::{ApplicationWindow, Builder, Button, ComboBox, ComboBoxText, MessageDialog};
use sha2::{Digest, Sha256};

use genetic_algorithm::crossover::genome_crossover::StringCrossover;
use genetic_algorithm::genome::population::{Individual, Population, ProblemType};
use genetic_algorithm::genome::problem::{FitnessFunction, OneMax};
use genetic_algorithm::mutation::genome_mutation::StringMutation;
use genetic_algorithm::selection::genome_selection::{SelectIndividual, TournamentSelection};
use plotters::prelude::*;
use rand::prelude::*;
use rand::Rng;
use std::env::args;
use std::path::PathBuf;
#[cfg(debug_assertions)]
use std::time::Duration;

const DEFAULT_POPULATION: u64 = 1000;
const DEFAULT_CROSSOVER_RATE: f64 = 0.80;
const DEFAULT_MUTATION_RATE: f64 = 0.05;
const DEFAULT_PROBLEM: &str = "One Max";
const DEFAULT_PROBLEM_TYPE: ProblemType = ProblemType::Max;
const DEFAULT_K_VALUE: u32 = 7;
const DEFAULT_ELITIST_VALUE: f64 = 0.85;
const DEFAULT_SEED: &[u8; 32] = &[
    1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4,
];
const DEFAULT_SELECTOR: Option<Box<SelectIndividual<T = String>>> = None;

struct Model<T> {
    population_size: u64,
    crossover_rate: f64,
    mutation_rate: f64,
    problem_type: ProblemType,
    selector: Option<Box<SelectIndividual<T = T>>>,
}

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

fn selection_combobox_changed_cb() {
    info!("selection selector changed");
}

fn on_problem_combobox_changed(builder: &Builder) {
    info!("problem changed");

    let model = Box::new(Model {
        population_size: DEFAULT_POPULATION,
        crossover_rate: DEFAULT_CROSSOVER_RATE,
        mutation_rate: DEFAULT_MUTATION_RATE,
        problem_type: DEFAULT_PROBLEM_TYPE,
        selector: DEFAULT_SELECTOR,
    });

    let selection_combobox: ComboBoxText = builder
        .get_object("selection_combobox")
        .expect("Couldn't get the selection combo box");

    selection_combobox.append(
        Some(String::from("Tournament Selection").as_str()),
        String::from("Tournament Selection").as_str(),
    );

    selection_combobox.connect_changed(move |model| selection_combobox_changed_cb());

    selection_combobox.set_active_id(Some(String::from("Tournament Selection").as_str()));
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("window.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));

    let problem_combobox: ComboBoxText = builder
        .get_object("problem_combobox")
        .expect("Couldn't get the problem combobox");

    problem_combobox.append(Some(DEFAULT_PROBLEM), DEFAULT_PROBLEM);

    problem_combobox.connect_changed(move |_| on_problem_combobox_changed(&builder));

    problem_combobox.set_active_id(Some(DEFAULT_PROBLEM));
    window.show_all();
}

fn main() {
    simple_logging::log_to_stderr(LevelFilter::Info);
    info!("Starting");
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
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
fn create_chart() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
    root.fill(&White)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &Red,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &Red));

    chart
        .configure_series_labels()
        .background_style(&White.mix(0.8))
        .border_style(&Black)
        .draw()?;

    Ok(())
}
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
