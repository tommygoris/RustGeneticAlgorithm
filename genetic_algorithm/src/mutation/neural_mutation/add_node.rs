use crate::genome::fitness_function::FitnessFunction;
use crate::genome::population::{Individual, Population};
use crate::mutation::genome_mutation::Mutate;
use crate::neural_network::neural_network::NeuralNetwork;
use rand::rngs::StdRng;
use rand::Rng;

// A node with a new hidden layer is guaranteed to be added (as long as the mutation rate just for add node is met) if the neural network currently has 0 hidden nodes.
// If the neural network already has a hidden node/layer, then both add a new node and add a new layer have to met to create a new hidden layer with a lone node.
pub struct AddNode {
    add_node_mutation_rate: f64,
    add_layer_mutation_rate: f64,
    seed: StdRng,
}

impl Mutate for AddNode {
    type T = NeuralNetwork;

    fn mutate(
        &mut self,
        population: &Population<NeuralNetwork>,
        mut fitness_function: Box<dyn FitnessFunction<T = NeuralNetwork>>,
    ) -> Vec<Individual<NeuralNetwork>> {
        let mut new_population = Vec::new();

        for individual in population.list_of_individuals().iter() {
            let gen_number = self.seed.gen::<f64>();
            if gen_number > self.add_node_mutation_rate {
                new_population.push(individual.clone());
                continue;
            }
            let mut mutated_neural_net = individual.retrieve_individual().clone();
            let mut add_layer = self.seed.gen::<f64>() < self.add_layer_mutation_rate;
            if !mutated_neural_net.has_hidden_layer() {
                add_layer = true;
            }

            if add_layer {
                mutated_neural_net.add_hidden_node_with_new_layer_at_end();
            } else {
                mutated_neural_net.add_hidden_node(
                    self.seed
                        .gen_range(0, mutated_neural_net.hidden_layer_length()),
                );
            }

            let new_fitness = fitness_function.calculate_fitness(&mutated_neural_net);
            let new_individual = Individual::new(mutated_neural_net, new_fitness);
        }

        new_population
    }
}
