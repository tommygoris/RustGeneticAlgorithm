extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{Relm, Update, Widget};

use self::Msg::*;

use genetic_algorithm::crossover::genome_crossover::StringCrossover;
use genetic_algorithm::genome::population::{Individual, Population, ProblemType};
use genetic_algorithm::genome::problem::{FitnessFunction, OneMax};
use genetic_algorithm::mutation::genome_mutation::StringMutation;
use genetic_algorithm::selection::genome_selection::TournamentSelection;
use plotters::prelude::*;
use rand::prelude::*;
use rand::Rng;
use std::env::args;
use std::path::PathBuf;
#[cfg(debug_assertions)]
use std::time::Duration;

#[derive(Msg)]
enum Msg {
    // …
    Quit,
}

struct Model {
    // …
}

struct Win {
    // …
    model: Model,
    window: Window,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            window: window,
        }
    }
}

fn main() {
    //create_chart();
    Win::run(()).expect("Win::run failed");
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
