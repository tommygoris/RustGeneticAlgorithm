#![feature(wait_until)]
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

mod one_max;
mod problem_settings;

use log::{info, warn, LevelFilter};

use glib::GString;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    BoxExt, Button, ButtonExt, CellLayoutExt, ComboBoxExt, ComboBoxTextExt, Container,
    ContainerExt, EditableSignals, EntryBuilder, EntryExt, GtkListStoreExtManual, Inhibit, Label,
    LabelExt, RadioButtonExt, ScrolledWindowExt, ToggleButtonExt, TreeViewColumnExt, TreeViewExt,
    WidgetExt, Window, WindowType,
};

use relm::{Channel, EventStream, Relm, Update, Widget, WidgetTest};
use sha2::{Digest, Sha256};

use crate::gtk::ListBoxExt;
use crate::gtk::ScrollableExt;
use crate::gtk::StaticType;
use crate::gtk::TextTagTableExt;
use crate::gtk::TextViewExt;
use crate::one_max::one_max::OneMax;
use crate::problem_settings::ProblemSettings;
use crossbeam_utils::thread::scope;
use genetic_algorithm::crossover::genome_crossover::{Crossover, StringCrossover};
use genetic_algorithm::genome::fitness_function::FitnessFunction;
use genetic_algorithm::genome::population::{Individual, Population, ProblemType};
use genetic_algorithm::mutation::genome_mutation::StringMutation;
use genetic_algorithm::selection::genome_selection::{SelectIndividual, TournamentSelection};
use gio::SocketConnectableExt;
use plotters::prelude::*;
use rand::prelude::*;
use rand::Rng;
use std::borrow::{Borrow, BorrowMut};
use std::convert::{TryFrom, TryInto};
use std::env::args;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
#[cfg(debug_assertions)]
use std::time::Duration;

const DEFAULT_POPULATION: u64 = 1;
const DEFAULT_CROSSOVER_RATE: f64 = 0.80;
const DEFAULT_MUTATION_RATE: f64 = 0.05;
const DEFAULT_PROBLEM: &str = "One Max";
const DEFAULT_PROBLEM_TYPE: ProblemType = ProblemType::Max;
const DEFAULT_K_VALUE: u32 = 7;
const DEFAULT_ELITIST_VALUE: f64 = 0.85;
const DEFAULT_SEED: &[u8; 32] = &[
    1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4,
];
const DEFAULT_CROSSOVER_POINTS: u32 = 5;
const DEFAULT_SELECTION_TYPE: &str = "Tournament Selection";

#[derive(Copy, Clone, Debug)]
enum Step {
    Inf,
    Steps(u64),
}

#[derive(Clone, Debug)]
enum SelectionTypes {
    Tournament,
    Random,
    FitnessProportionate,
}

pub struct Model {
    population_size: u64,
    crossover_rate: f64,
    mutation_rate: f64,
    problem_type: ProblemType,
    seed: [u8; 32],
    selection_type: String,
    problem_to_solve: String,
    steps: Step,
    current_problem: Option<Box<dyn ProblemSettings + Send + Sync>>,
    sender: Option<Sender<()>>,
    mutex_cond: Option<Arc<(Mutex<bool>, Condvar)>>,
    stream: EventStream<Msg>,
    // selector: Option<Box<SelectIndividual<T = T>>>
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
    SeedChanged,
    StartGA,
    ResumeGA,
    PauseGA,
    CurrentGen(u64, Vec<Individual<String>>),
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
    problem_combobox: gtk::ComboBoxText,
    population_entry: gtk::Entry,
    crossover_rate_entry: gtk::Entry,
    mutation_rate_entry: gtk::Entry,
    selection_type_combobox: gtk::ComboBoxText,
    radio_buttons: Vec<gtk::RadioButton>,
    seed_entry: gtk::Entry,
    population_list_box: gtk::TreeView,
    current_gen_label: gtk::Label,
}

fn main() {
    gtk::init().expect("gtk::init failed");
    simple_logging::log_to_stderr(LevelFilter::Info);
    info!("Starting");
    let main_stream = EventStream::new();

    fn create_button_box(
        label_text: &str,
        stream: &EventStream<Msg>,
        msg: Msg,
    ) -> (gtk::Box, gtk::Button) {
        let problem_inner_vbox = gtk::Box::new(Horizontal, 10);
        let button = gtk::Button::new_with_label(label_text);

        problem_inner_vbox.pack_start(&button, false, true, 5);

        let stream = stream.clone();
        button.connect_clicked(move |_| {
            stream.emit(msg.clone());
        });

        (problem_inner_vbox, button)
    }
    fn create_listbox_box(label_text: &str) -> (gtk::TreeView) {
        let problem_inner_vbox = gtk::Box::new(Horizontal, 10);
        let problem_type_label = gtk::Label::new(Some(label_text));

        let tree = create_and_setup_view();
        //tree.set
        //problem_inner_vbox.pack_start(&problem_type_label, false, true, 5);
        //problem_inner_vbox.pack_start(&tree, false, true, 5);

        (tree)
    }
    fn create_label_entry_box(
        label_text: &str,
        stream: &EventStream<Msg>,
        msg: Msg,
        initial_entry_val: &str,
    ) -> (gtk::Box, gtk::Entry) {
        let problem_inner_vbox = gtk::Box::new(Horizontal, 10);

        let problem_type_label = gtk::Label::new(Some(label_text));
        problem_type_label.set_width_chars(20);
        let problem_type_entry = gtk::Entry::new();

        problem_inner_vbox.pack_start(&problem_type_label, false, true, 5);
        problem_inner_vbox.pack_start(&problem_type_entry, false, true, 5);

        let stream = stream.clone();
        problem_type_entry.connect_changed(move |_| {
            stream.emit(msg.clone());
        });

        problem_type_entry.set_text(initial_entry_val);
        (problem_inner_vbox, problem_type_entry)
    }

    fn create_label_combobox_box(
        label_text: &str,
        text_entries: &Vec<&str>,
        stream: &EventStream<Msg>,
        msg: Msg,
    ) -> (gtk::Box, gtk::ComboBoxText) {
        let problem_inner_vbox = gtk::Box::new(Horizontal, 10);

        let problem_type_label = gtk::Label::new(Some(label_text));
        problem_type_label.set_width_chars(20);
        let problem_type_combobox = gtk::ComboBoxText::new();

        problem_inner_vbox.pack_start(&problem_type_label, false, true, 5);
        problem_inner_vbox.pack_start(&problem_type_combobox, false, true, 5);

        let stream = stream.clone();
        problem_type_combobox.connect_changed(move |_| {
            stream.emit(msg.clone());
        });

        if text_entries.len() > 0 {
            for (index, item) in text_entries.iter().enumerate() {
                problem_type_combobox.append(Some(item), item);
            }

            problem_type_combobox.set_active_id(Some(text_entries[0]));
        }
        (problem_inner_vbox, problem_type_combobox)
    }

    fn create_problem_type_radio_group(
        radio_text: Vec<&str>,
        stream: &EventStream<Msg>,
        msg: Msg,
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
                let msg = msg.clone();
                let stream = stream.clone();
                radio_button.connect_clicked(move |_| {
                    stream.emit(msg.clone());
                });
                radio_vector.push(radio_button.clone());
            }
            first_radio_button.set_active(true);
        }
        (problem_inner_vbox, radio_vector)
    }

    fn create_and_setup_view() -> gtk::TreeView {
        // Creating the tree view.
        let tree = gtk::TreeView::new();

        tree.set_headers_visible(false);
        // Creating the two columns inside the view.
        append_column(&tree, 0);
        append_column(&tree, 1);
        tree
    }

    fn create_and_fill_model(data: &Vec<String>) -> gtk::ListStore {
        // Creation of a model with two rows.
        let model = gtk::ListStore::new(&[u32::static_type(), String::static_type()]);

        // Filling up the tree view.
        //let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
        for (i, entry) in data.iter().enumerate() {
            model.insert_with_values(None, &[0, 1], &[&(i as u32 + 1), entry]);
        }
        model
    }

    fn append_column(tree: &gtk::TreeView, id: i32) {
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        // Association of the view's column with the model's `id` column.
        column.add_attribute(&cell, "text", id);
        tree.append_column(&column);
    }

    fn create_label_on_label_box(
        left_label_text: &str,
        right_label_text: &str,
    ) -> (gtk::Box, gtk::Label, gtk::Label) {
        let label_vbox = gtk::Box::new(Horizontal, 10);

        let left_label = gtk::Label::new(Some(left_label_text));

        let right_label = gtk::Label::new(Some(right_label_text));

        label_vbox.pack_start(&left_label, false, true, 5);
        label_vbox.pack_start(&right_label, false, true, 5);

        (label_vbox, left_label, right_label)
    }

    let radio_widgets = create_problem_type_radio_group(
        vec!["Max", "Min"],
        &main_stream,
        Msg::ProblemRadioChangedToggleChanged,
    );
    let population_entry = create_label_entry_box(
        "Population Size",
        &main_stream,
        Msg::PopulationChanged,
        DEFAULT_POPULATION.to_string().as_str(),
    );
    let crossover_entry = create_label_entry_box(
        "Crossover Rate",
        &main_stream,
        Msg::CrossoverRateChanged,
        DEFAULT_CROSSOVER_RATE.to_string().as_str(),
    );
    let mutation_entry = create_label_entry_box(
        "Mutation Rate",
        &main_stream,
        Msg::MutationRateChanged,
        DEFAULT_MUTATION_RATE.to_string().as_str(),
    );
    let problem_combobox = create_label_combobox_box(
        "Problem",
        &mut vec!["One Max"],
        &main_stream,
        Msg::ProblemChanged,
    );
    let selection_type_combobox = create_label_combobox_box(
        "Selection Type",
        &mut vec!["Tournament Selection"],
        &main_stream,
        Msg::SelectionChanged,
    );
    let seed_entry = create_label_entry_box(
        "Seed",
        &main_stream,
        Msg::SeedChanged,
        std::str::from_utf8(DEFAULT_SEED).unwrap(),
    );

    let start_button = create_button_box("Start", &main_stream, Msg::StartGA);
    let resume_button = create_button_box("Stop", &main_stream, Msg::PauseGA);
    let stop_button = create_button_box("Resume", &main_stream, Msg::ResumeGA);

    start_button.0.pack_start(&resume_button.0, false, false, 5);
    start_button.0.pack_start(&stop_button.0, false, false, 5);

    let left_outer_vbox = gtk::Box::new(Vertical, 0);
    left_outer_vbox.pack_start(&problem_combobox.0, false, false, 5);
    left_outer_vbox.pack_start(&population_entry.0, false, false, 5);
    left_outer_vbox.pack_start(&crossover_entry.0, false, false, 5);
    left_outer_vbox.pack_start(&mutation_entry.0, false, false, 5);
    left_outer_vbox.pack_start(&selection_type_combobox.0, false, false, 5);
    left_outer_vbox.pack_start(&radio_widgets.0, false, false, 5);
    left_outer_vbox.pack_start(&radio_widgets.0, false, false, 5);
    left_outer_vbox.pack_start(&seed_entry.0, false, false, 5);
    left_outer_vbox.pack_start(&start_button.0, false, false, 5);

    let top_level_box = gtk::Box::new(Horizontal, 0);
    let current_gen_labels = create_label_on_label_box("Current Gen:", "0");

    let population_list_box = create_listbox_box("Population");
    let inner_level_box = gtk::Box::new(Vertical, 0);
    let horizontal_adjustment = gtk::Adjustment::new(50.0, 0.0, 50.0, 1.0, 1.0, 500.0);
    let vertical_adjustment = gtk::Adjustment::new(50.0, 0.0, 50.0, 1.0, 1.0, 500.0);

    //    population_list_box
    //        .1
    //        .set_hadjustment(Some(&horizontal_adjustment));
    //    population_list_box
    //        .1
    //        .set_vadjustment(Some(&vertical_adjustment));
    let scrolled_window_pop_list =
        gtk::ScrolledWindow::new(Some(&horizontal_adjustment), Some(&vertical_adjustment));
    scrolled_window_pop_list.add(&population_list_box);

    //scrolled_window_pop_list.set
    //    population_list_box
    //        .0
    //        .pack_start(&scrolled_window_pop_list, false, false, 5);

    //    population_list_box
    //        .1
    //        .set_adjustment(Some(&vertical_adjustment));
    inner_level_box.pack_start(&current_gen_labels.0, false, false, 5);
    inner_level_box.add(&scrolled_window_pop_list);

    let window = Window::new(WindowType::Toplevel);
    window.set
    top_level_box.pack_start(&left_outer_vbox, false, false, 5);
    top_level_box.pack_start(&inner_level_box, false, false, 5);

    window.add(&top_level_box);
    window.show_all();

    let widgets = Widgets {
        problem_combobox: problem_combobox.1,
        population_entry: population_entry.1,
        crossover_rate_entry: crossover_entry.1,
        mutation_rate_entry: mutation_entry.1,
        selection_type_combobox: selection_type_combobox.1,
        radio_buttons: radio_widgets.1,
        seed_entry: seed_entry.1,
        population_list_box,
        current_gen_label: current_gen_labels.2,
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
        seed: *DEFAULT_SEED,
        selection_type: DEFAULT_SELECTION_TYPE.to_string(),
        problem_to_solve: DEFAULT_PROBLEM.to_string(),
        steps: Step::Inf,
        current_problem: None,
        sender: None,
        mutex_cond: None,
        stream: main_stream.clone(),
    };

    fn update(event: Msg, model: &mut Model, widgets: &Widgets) {
        println!("{:?}", event);
        match event {
            Msg::Quit => {
                info!("Quitting");
                gtk::main_quit()
            }
            Msg::ProblemRadioChangedToggleChanged => {
                info!("ProblemRadioChanged");
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
            Msg::ProblemChanged => {
                info!("ProblemChanged");

                let combobox_widget = &widgets.problem_combobox;
                let combobox_current_text_option = combobox_widget.get_active_text();

                if let Some(current_active_text) = combobox_current_text_option {
                    model.problem_to_solve = current_active_text.to_string();
                }
            }
            Msg::PopulationChanged => {
                let population_option = widgets.population_entry.get_text();

                if let Some(population_string) = population_option {
                    let population_string = population_string.to_string();
                    let population_size = population_string.parse::<u64>();

                    match population_size {
                        Ok(pop_size) => model.population_size = pop_size,
                        Err(e) => {
                            info!("Number too large to parse {:?}", e);
                            model.population_size = std::u64::MAX;
                            widgets
                                .population_entry
                                .set_text(std::u64::MAX.to_string().as_str());
                        }
                    }

                    info!("New population is: {:?}", model.population_size);

                    info!("PopulationChanged");
                }
            }
            Msg::CrossoverRateChanged => {
                let crossoverrate_option = widgets.crossover_rate_entry.get_text();

                if let Some(crossover_string) = crossoverrate_option {
                    let crossover_string = crossover_string.to_string();
                    let crossover_rate = crossover_string.parse::<f64>();

                    match crossover_rate {
                        Ok(cross_rate) => model.crossover_rate = cross_rate,
                        Err(e) => {
                            info!("Number couldn't be parsed {:?}", e);
                            let lower_limit_rate = 0.0;
                            model.crossover_rate = lower_limit_rate;
                            widgets
                                .crossover_rate_entry
                                .set_text(lower_limit_rate.to_string().as_str());
                        }
                    }

                    info!("New crossrate rate is: {:?}", model.crossover_rate);
                }

                info!("CrossoverRateChanged");
            }
            Msg::MutationRateChanged => {
                let mutation_option = widgets.mutation_rate_entry.get_text();

                if let Some(mutation_string) = mutation_option {
                    let mutation_string = mutation_string.to_string();
                    let mutation_rate = mutation_string.parse::<f64>();

                    match mutation_rate {
                        Ok(mut_rate) => model.mutation_rate = mut_rate,
                        Err(e) => {
                            info!("Number couldn't be parsed {:?}", e);
                            model.mutation_rate = std::f64::MAX;
                            widgets
                                .mutation_rate_entry
                                .set_text(std::f64::MAX.to_string().as_str());
                        }
                    }

                    info!("New mutation rate is: {:?}", model.mutation_rate);
                    info!("MutationRateChanged");
                }
            }
            Msg::SelectionChanged => {
                info!("SelectionChanged");
                let combobox_widget = &widgets.selection_type_combobox;
                let combobox_current_text_option = combobox_widget.get_active_text();

                if let Some(current_active_text) = combobox_current_text_option {
                    model.selection_type = current_active_text.to_string();
                }
            }
            Msg::SeedChanged => {
                info!("SeedChanged");
                let seed_string_option = widgets.seed_entry.get_text();

                if let Some(seed_string) = seed_string_option {
                    let seed_string = seed_string.to_string();
                    let hash = create_hash(seed_string.as_str());
                    let new_seed = array_ref!(hash.as_bytes(), 0, 32);
                    model.seed = *new_seed;

                    info!("New seed is: {:?}", new_seed);
                } else {
                    info!("Failed to convert the seed string option to a string");
                }
            }
            Msg::StartGA => {
                info!("GA started");
                let selection_combobox_option =
                    &widgets.selection_type_combobox.get_active_text().unwrap();
                let problem_combobox_option = &widgets.problem_combobox.get_active_text().unwrap();

                let mut selector =
                    TournamentSelection::new(DEFAULT_K_VALUE, DEFAULT_ELITIST_VALUE, model.seed);
                let mut crossover = StringCrossover::new(
                    model.crossover_rate,
                    DEFAULT_CROSSOVER_POINTS,
                    model.seed,
                );
                info!("Starting with tournament Selection");
                let mut mutation =
                    StringMutation::new(model.mutation_rate, vec!['0', '1'], model.seed);

                let mut one_max = one_max::one_max::OneMax::new(
                    0,
                    false,
                    Box::new(crossover),
                    Box::new(selector),
                    Box::new(mutation),
                    &model.seed,
                    model.population_size,
                );

                let steps = model.steps.clone();

                //let (tx, rx) = channel();
                //model.receiver = Some(rx);
                let pair = Arc::new((Mutex::new(true), Condvar::new()));
                let pair2 = pair.clone();
                let stream = model.stream.clone();

                let (channel, sender) =
                    Channel::new(move |data: (u64, Vec<Individual<String>>)| {
                        // This closure is executed whenever a message is received from the sender.
                        // We send a message to the current widget.
                        stream.emit(Msg::CurrentGen(data.0, data.1));
                    });

                model.mutex_cond = Some(pair);
                let handler = thread::spawn(move || {
                    let &(ref lock, ref cvar) = &*pair2;
                    match steps {
                        Step::Inf => loop {
                            one_max.on_start();
                            let list_of_indvs = one_max
                                .population()
                                .as_ref()
                                .unwrap()
                                .list_of_individuals()
                                .clone();
                            let current_gen = one_max.current_gen();
                            sender
                                .send((current_gen, list_of_indvs.to_vec()))
                                .expect("send message");
                            let _guard = cvar
                                .wait_until(lock.lock().unwrap(), |started| *started)
                                .unwrap();
                        },
                        Step::Steps(num_step) => {
                            for _ in 0..num_step {
                                one_max.on_start();
                                let _guard = cvar
                                    .wait_until(lock.lock().unwrap(), |started| *started)
                                    .unwrap();
                            }
                        }
                    }
                });
            }
            Msg::ResumeGA => {
                let mut started = model.mutex_cond.as_ref().unwrap().0.lock().unwrap();
                *started = true;
                model.mutex_cond.as_ref().unwrap().1.notify_one();
            }
            Msg::PauseGA => {
                let mut started = model.mutex_cond.as_ref().unwrap().0.lock().unwrap();
                *started = false;
                model.mutex_cond.as_ref().unwrap().1.notify_one();
            }
            Msg::CurrentGen(num, list_of_individuals) => {
                let label = &widgets.current_gen_label;
                label.set_text(num.to_string().as_str());
                let list_widget = &widgets.population_list_box;
                let mut tree_pop_vec = Vec::new();
                //                //let text_buffer = gtk::TextBuffer::new();
                //                for row in list_widget.get_children() {
                //                    list_widget.remove(&row);
                //                }
                //
                for indv in &list_of_individuals {
                    let idnv_clone = indv.clone();
                    tree_pop_vec.push(idnv_clone.to_string());
                }

                list_widget.set_model(Some(&create_and_fill_model(&tree_pop_vec)));
                //list_widget.show_all();
            }
        }
    }

    main_stream.set_callback(move |msg| {
        update(msg, &mut model, &widgets);
    });
    gtk::main();
}

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

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}
