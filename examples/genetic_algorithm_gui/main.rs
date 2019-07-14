extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
#[cfg_attr(test, macro_use)]
extern crate sha2;
extern crate simple_logging;
#[macro_use]
extern crate arrayref;
extern crate glib;

use log::{info, warn, LevelFilter};

use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    BoxExt, Button, ButtonExt, Container, ContainerExt, EntryBuilder, Inhibit, Label, LabelExt,
    RadioButtonExt, ToggleButtonExt, WidgetExt, Window, WindowType,
};
use relm::{EventStream, Relm, Update, Widget, WidgetTest};
use sha2::{Digest, Sha256};

use genetic_algorithm::crossover::genome_crossover::StringCrossover;
use genetic_algorithm::genome::population::{Individual, Population, ProblemType};
use genetic_algorithm::genome::problem::{FitnessFunction, OneMax};
use genetic_algorithm::mutation::genome_mutation::StringMutation;
use genetic_algorithm::selection::genome_selection::{SelectIndividual, TournamentSelection};
use plotters::prelude::*;
use rand::prelude::*;
use rand::Rng;
use std::convert::TryFrom;
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

//struct Model<T> {
//    population_size: u64,
//    crossover_rate: f64,
//    mutation_rate: f64,
//    problem_type: ProblemType,
//    selector: Option<Box<SelectIndividual<T = T>>>,
//}
//
//impl<T> Model<T> {
//    fn get_population_size(&mut self) -> &mut u64 {
//        &mut self.population_size
//    }
//
//    fn set_population_size(&mut self, new_val: u64) {
//        self.population_size = new_val;
//    }
//}
//
//fn create_hash(text: &str) -> String {
//    let mut hasher = Sha256::default();
//    hasher.input(text.as_bytes());
//    format!("{:x}", hasher.result())
//}
//
//fn selection_combobox_changed_cb() {
//    info!("selection selector changed");
//}
//
//fn set_selection_combobox<T>(builder: &Builder, model: &Box<Model<T>>) {
//    let selection_combobox: ComboBoxText = builder
//        .get_object("selection_combobox")
//        .expect("Couldn't get the selection combo box");
//
//    selection_combobox.append(
//        Some(String::from("Tournament Selection").as_str()),
//        String::from("Tournament Selection").as_str(),
//    );
//
//    selection_combobox.connect_changed(move |_| selection_combobox_changed_cb());
//
//    selection_combobox.set_active_id(Some(String::from("Tournament Selection").as_str()));
//}
//
//fn set_population_entry<T>(builder: &Builder, model: &'static mut Box<Model<T>>) {
//    let population_entry: Entry = builder
//        .get_object("population_entry")
//        .expect("Couldn't get the population entry");
//
//    population_entry.connect_changed(move |_| model.set_population(10));
//
//    set_entry(&population_entry, DEFAULT_POPULATION.to_string().as_str());
//}
//
//fn set_crossover_rate_entry<T>(builder: &Builder, model: &Box<Model<T>>) {
//    let crossover_entry: Entry = builder
//        .get_object("crossover_entry")
//        .expect("Couldn't get the population entry");
//
//    set_entry(
//        &crossover_entry,
//        DEFAULT_CROSSOVER_RATE.to_string().as_str(),
//    );
//}
//
//fn set_mutation_rate_entry<T>(builder: &Builder, model: &Box<Model<T>>) {
//    let mutation_entry: Entry = builder
//        .get_object("mutation_entry")
//        .expect("Couldn't get the population entry");
//
//    set_entry(&mutation_entry, DEFAULT_MUTATION_RATE.to_string().as_str());
//}
//
//fn set_entry(entry: &Entry, text: &str) {
//    entry.set_text(text);
//}
//
//fn on_problem_combobox_changed(builder: &Builder) {
//    info!("problem changed");
//
//    let model = Box::new(Model {
//        population_size: DEFAULT_POPULATION,
//        crossover_rate: DEFAULT_CROSSOVER_RATE,
//        mutation_rate: DEFAULT_MUTATION_RATE,
//        problem_type: DEFAULT_PROBLEM_TYPE,
//        selector: DEFAULT_SELECTOR,
//    });
//
//    set_selection_combobox(builder, &model);
//    set_population_entry(builder, &mut model);
//    set_crossover_rate_entry(builder, &model);
//    set_mutation_rate_entry(builder, &model)
//}
//
//fn build_ui(application: &gtk::Application) {
//    let glade_src = include_str!("window.glade");
//    let builder = Builder::new_from_string(glade_src);
//
//    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
//    window.set_application(Some(application));
//
//    let problem_combobox: ComboBoxText = builder
//        .get_object("problem_combobox")
//        .expect("Couldn't get the problem combobox");
//
//    problem_combobox.append(Some(DEFAULT_PROBLEM), DEFAULT_PROBLEM);
//
//    problem_combobox.connect_changed(move |_| on_problem_combobox_changed(&builder));
//
//    problem_combobox.set_active_id(Some(DEFAULT_PROBLEM));
//    window.show_all();
//}

struct Model {
    population_size: u64,
    crossover_rate: f64,
    mutation_rate: f64,
    problem_type: ProblemType,
    // selector: Option<Box<SelectIndividual<T = T>>>,
}

#[derive(Clone, Debug)]
enum Msg {
    Quit,
    ProblemChanged,
    PopulationChanged,
    CrossoverRateChanged,
    MutationRateChanged,
    SelectionChanged,
    ProblemRadioChangedToggleChanged,
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
    problem_combobox: gtk::ComboBox,
    population_entry: gtk::Entry,
    crossover_rate_entry: gtk::Entry,
    mutation_rate_entry: gtk::Entry,
    selection_type_combobox: gtk::ComboBox,
    radio_buttons: Vec<gtk::RadioButton>,
}

fn main() {
    gtk::init().expect("gtk::init failed");
    simple_logging::log_to_stderr(LevelFilter::Info);
    info!("Starting");
    let main_stream = EventStream::new();

    fn create_label_entry_box(label_text: &str) -> (gtk::Box, gtk::Entry) {
        let problem_inner_vbox = gtk::Box::new(Horizontal, 10);

        let problem_type_label = gtk::Label::new(Some(label_text));
        problem_type_label.set_width_chars(20);
        let problem_type_entry = gtk::Entry::new();

        problem_inner_vbox.pack_start(&problem_type_label, false, true, 5);
        problem_inner_vbox.pack_start(&problem_type_entry, false, true, 5);

        (problem_inner_vbox, problem_type_entry)
    }

    fn create_label_combobox_box(label_text: &str) -> (gtk::Box, gtk::ComboBox) {
        let problem_inner_vbox = gtk::Box::new(Horizontal, 10);

        let problem_type_label = gtk::Label::new(Some(label_text));
        problem_type_label.set_width_chars(20);
        let problem_type_combobox = gtk::ComboBox::new();

        problem_inner_vbox.pack_start(&problem_type_label, false, true, 5);
        problem_inner_vbox.pack_start(&problem_type_combobox, false, true, 5);

        (problem_inner_vbox, problem_type_combobox)
    }

    fn create_problem_type_radio_group(
        radio_text: Vec<&str>,
        stream: &EventStream<Msg>,
    ) -> (gtk::Box, Vec<gtk::RadioButton>) {
        let problem_inner_vbox = gtk::Box::new(Horizontal, 10);
        let mut radio_vector = vec![];
        if radio_text.len() > 0 {
            let first_radio_button = gtk::RadioButton::new_with_label(radio_text[0]);
            radio_vector.push(first_radio_button.clone());
            problem_inner_vbox.pack_start(&first_radio_button, false, false, 5);

            for str in radio_text.iter().skip(1) {
                let radio_button = gtk::RadioButton::new_with_label(str);

                first_radio_button.join_group(Some(&radio_button));

                problem_inner_vbox.pack_start(&radio_button, false, false, 5);

                let stream = stream.clone();
                radio_button.connect_clicked(move |_| {
                    stream.emit(Msg::ProblemRadioChangedToggleChanged);
                });
                radio_vector.push(radio_button.clone());
            }
            first_radio_button.set_active(true);
        }
        (problem_inner_vbox, radio_vector)
    }
    // Create the view using the normal GTK+ method calls.
    let outer_vbox = gtk::Box::new(Vertical, 0);

    let radio_widgets = create_problem_type_radio_group(vec!["Max", "Min"], &main_stream);
    let population_entry = create_label_entry_box("Population Size");
    let crossover_entry = create_label_entry_box("Crossover Rate");
    let mutation_entry = create_label_entry_box("Mutation Rate");
    let problem_combobox = create_label_combobox_box("Problem");
    let selection_type_combobox = create_label_combobox_box("Selection Type");

    outer_vbox.pack_start(&problem_combobox.0, false, false, 5);
    outer_vbox.pack_start(&population_entry.0, false, false, 5);
    outer_vbox.pack_start(&crossover_entry.0, false, false, 5);
    outer_vbox.pack_start(&mutation_entry.0, false, false, 5);
    outer_vbox.pack_start(&selection_type_combobox.0, false, false, 5);
    outer_vbox.pack_start(&radio_widgets.0, false, false, 5);

    let ch = outer_vbox.get_children();

    let window = Window::new(WindowType::Toplevel);

    window.add(&outer_vbox);

    window.show_all();

    let widgets = Widgets {
        problem_combobox: problem_combobox.1,
        population_entry: population_entry.1,
        crossover_rate_entry: crossover_entry.1,
        mutation_rate_entry: mutation_entry.1,
        selection_type_combobox: selection_type_combobox.1,
        radio_buttons: radio_widgets.1,
    };

    main_stream.observe(move |event: &Msg| {
        println!("Event: {:?}", event);
        // echo_stream.emit(event.clone());
    });

    let stream = main_stream.clone();
    window.connect_delete_event(move |_, _| {
        stream.emit(Msg::Quit);
        Inhibit(false)
    });

    let mut model = Model {
        population_size: DEFAULT_POPULATION,
        crossover_rate: DEFAULT_CROSSOVER_RATE,
        mutation_rate: DEFAULT_MUTATION_RATE,
        problem_type: DEFAULT_PROBLEM_TYPE,
    };

    fn update(event: Msg, model: &mut Model, widgets: &Widgets) {
        println!("{:?}", event);
        match event {
            Msg::Quit => {
                info!("Quitting");
                gtk::main_quit()
            }
            Msg::ProblemRadioChangedToggleChanged => {
                info!("Changed");
                let radio_widgets = &widgets.radio_buttons;

                let first_radio_button = &radio_widgets[0];
                let first_radio_label_condition = first_radio_button
                    .get_label()
                    .unwrap()
                    .to_string()
                    .eq("Max");

                if first_radio_button.get_active() && first_radio_label_condition {
                    info!("Max");
                    model.problem_type = ProblemType::Max;
                } else {
                    info!("Min");
                    model.problem_type = ProblemType::Min;
                }

                //model.problem_type =
            }
        }
    }

    main_stream.set_callback(move |msg| {
        update(msg, &mut model, &widgets);
    });

    gtk::main();
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
