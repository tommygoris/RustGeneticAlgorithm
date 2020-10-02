use genetic_algorithm::crossover::neural_crossover::node_swap::HiddenLayerSwap;
use genetic_algorithm::genome::fitness_function::FitnessFunction;
use genetic_algorithm::genome::population::{Individual, Population, ProblemType};
use genetic_algorithm::mutation::neural_mutation::add_node::AddNode;
use genetic_algorithm::mutation::neural_mutation::delete_node::DeleteNode;
use genetic_algorithm::neural_network::neural_network::NeuralNetwork;
use genetic_algorithm::selection::genome_selection::TournamentSelection;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;

const DEFAULT_SEED: &[u8; 32] = &[
    1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4,
];
// const DEFAULT_SEED2: &[u8; 32] = &[
//     1, 1, 1, 1, 2, 3, 4, 5, 6, 1, 3, 34, 13, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3,
//     4,
// ];

const MAX_NUM: u32 = 9;
const NUMBER_OF_NETWORKS_TO_MAKE: u32 = 100;
const K_VALUE: u32 = 100;
const SELECTION_WIN_CHANCE: f64 = 0.95;
const SWAP_CHANCE: f64 = 0.90;
const ADD_NODE_MUTATION_RATE: f64 = 0.95;
const ADD_LAYER_MUTATION_RATE: f64 = 0.95;
const DELETE_NODE_MUTATION_RATE: f64 = 0.95;

#[derive(Default, Clone, Debug)]
struct NetFitness {
    image_list: Vec<Vec<f64>>,
    actual_list: Vec<u8>,
}

impl NetFitness {
    fn new(image_list: Vec<Vec<f64>>, actual_list: Vec<u8>) -> NetFitness {
        NetFitness {
            image_list,
            actual_list,
        }
    }
    fn get_accuracy(&mut self, net: &NeuralNetwork) -> f64 {
        let mut correct = 0.0;
        let mut output = vec![];
        for (index, image_list) in self.image_list.iter().enumerate() {
            output = net.feedforward(image_list.as_slice());
            let actual_val = self.actual_list[index];

            for number in 1..MAX_NUM + 1 {
                if actual_val == number as u8 {
                    if output[number as usize - 1] > 0.5 {
                        correct += 1.0
                    }
                }
            }
        }
        println!(
            "Correct: {:?}, Image_List: {:?}",
            correct,
            self.image_list.len()
        );
        correct / (self.image_list.len() as f64)
    }
}

impl FitnessFunction for NetFitness {
    type T = NeuralNetwork;

    fn calculate_fitness(&mut self, net: &NeuralNetwork) -> f64 {
        let mut fitness = 0.0;
        let mut output = vec![];

        for (index, image_list) in self.image_list.iter().enumerate() {
            output = net.feedforward(image_list.as_slice());
            let actual_val = self.actual_list[index];

            for number in 1..MAX_NUM + 1 {
                if actual_val == number as u8 {
                    if output[number as usize - 1] > 0.5 {
                        fitness += 100000.0;
                    } else {
                        fitness -= 10000.0;
                    }
                } else {
                    if output[number as usize - 1] > 0.5 {
                        fitness -= 10000.0;
                    }
                }
            }
        }
        return fitness;
    }
}

fn main() {
    let input_label_list = read_input_labels_from_file();
    let input_images_list = read_input_images_from_file();
    let mut rng: StdRng = SeedableRng::from_seed(*DEFAULT_SEED);

    let mut network_list = Vec::new();
    let mut net_fitness_function = NetFitness::new(input_images_list, input_label_list);
    for _ in 0..NUMBER_OF_NETWORKS_TO_MAKE {
        // let new_seed = generate_seed(rng);
        let mut new_seed = [0; 32];

        let num_of_values = 32;
        for i in 0..num_of_values {
            new_seed[i] = rng.gen::<u8>();
        }
        // new_seed
        let net = NeuralNetwork::new(28 * 28, &[], 9, new_seed);
        let fitness = net_fitness_function.calculate_fitness(&net);
        network_list.push(Individual::new(net, fitness));
    }

    let mut net_pop = Population::new(network_list, ProblemType::Max);

    let mut node_swap = HiddenLayerSwap::new(SWAP_CHANCE, *DEFAULT_SEED);
    let mut tournament_selection: TournamentSelection =
        TournamentSelection::new(K_VALUE, SELECTION_WIN_CHANCE, *DEFAULT_SEED);

    let mut add_node = AddNode::new(
        ADD_NODE_MUTATION_RATE,
        ADD_LAYER_MUTATION_RATE,
        *DEFAULT_SEED,
    );

    let mut delete_node = DeleteNode::new(DELETE_NODE_MUTATION_RATE, *DEFAULT_SEED);
    let mut gen = 0;
    loop {
        // net_pop.crossover(
        //     &mut node_swap,
        //     &mut tournament_selection,
        //     Box::new(net_fitness_function.clone()),
        // );
        net_pop.crossover(
            &mut node_swap,
            &mut tournament_selection,
            Box::new(net_fitness_function.clone()),
        );
        net_pop.mutate(&mut delete_node, Box::new(net_fitness_function.clone()));
        net_pop.mutate(&mut add_node, Box::new(net_fitness_function.clone()));
        //println!("{:?}", net_pop.find_top_individual().fitness());
        gen += 1;
        let indv = net_pop.find_top_individual();
        println!(
            "Generation: {:?}, Best fitness: {:?} Accuracy: {:?}",
            gen,
            indv,
            net_fitness_function.get_accuracy(indv.retrieve_individual())
        );
    }
}

// pub fn generate_seed(mut rng: StdRng) -> [u8; 32] {
//     let mut new_seed = [0; 32];
//
//     let num_of_values = 32;
//     for i in 0..num_of_values {
//         new_seed[i] = rng.gen::<u8>();
//     }
//     new_seed
// }

fn read_input_labels_from_file() -> Vec<u8> {
    let mut input_label_list = Vec::new();
    let mut labels = File::open(
        "examples/genetic_algorithm_gui/neural_network/images_net/train-labels.idx1-ubyte",
    )
    .unwrap();
    let mut buf = [0u8; 4];
    labels.read(&mut buf).unwrap();
    let magic_number = u32::from_be_bytes(buf);
    labels.read(&mut buf).unwrap();
    let number_of_items = u32::from_be_bytes(buf);
    let mut buf = [0u8; 1];
    for _ in 0..number_of_items {
        labels.read(&mut buf).unwrap();
        input_label_list.push(buf[0]);
    }
    input_label_list
}

fn read_input_images_from_file() -> Vec<Vec<f64>> {
    let mut input_images_list: Vec<Vec<f64>> = Vec::new();
    let mut images = File::open(
        "examples/genetic_algorithm_gui/neural_network/images_net/train-images.idx3-ubyte",
    )
    .unwrap();
    let mut buf = [0u8; 4];
    images.read(&mut buf).unwrap();
    let magic_number = u32::from_be_bytes(buf);
    images.read(&mut buf).unwrap();
    let number_of_images = u32::from_be_bytes(buf);
    images.read(&mut buf).unwrap();
    let number_of_rows = u32::from_be_bytes(buf);
    images.read(&mut buf).unwrap();
    let number_of_columns = u32::from_be_bytes(buf);

    let mut pixel_buf = [0u8; 1];
    for i in 0..number_of_images / 1000 {
        let mut input_images_array: Vec<f64> = Vec::new();
        for j in 0..number_of_columns {
            for k in 0..number_of_rows {
                images.read(&mut pixel_buf).unwrap();
                input_images_array.push(f64::from(pixel_buf[0]));
            }
        }
        input_images_list.push(input_images_array);
    }
    input_images_list
}
