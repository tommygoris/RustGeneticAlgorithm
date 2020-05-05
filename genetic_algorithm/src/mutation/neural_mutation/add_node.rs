use crate::genome::fitness_function::FitnessFunction;
use crate::genome::population::{Individual, Population};
use crate::mutation::genome_mutation::Mutate;
use crate::neural_network::neural_network::NeuralNetwork;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// A node with a new hidden layer is guaranteed to be added (as long as the mutation rate just for add node is met) if the neural network currently has 0 hidden nodes.
// If the neural network already has a hidden node/layer, then both add a new node and add a new layer have to met to create a new hidden layer with a lone node.
pub struct AddNode {
    add_node_mutation_rate: f64,
    add_layer_mutation_rate: f64,
    seed: StdRng,
}

impl AddNode {
    fn new(add_node_mutation_rate: f64, add_layer_mutation_rate: f64, seed: [u8; 32]) -> AddNode {
        AddNode {
            add_node_mutation_rate,
            add_layer_mutation_rate,
            seed: SeedableRng::from_seed(seed),
        }
    }
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
            if mutated_neural_net.is_hidden_layer_empty() {
                add_layer = true;
            }

            if add_layer {
                mutated_neural_net.push_hidden_node_with_new_layer_at_end();
            } else {
                mutated_neural_net
                    .push_hidden_node(self.seed.gen_range(0, mutated_neural_net.hidden_length()));
            }

            let new_fitness = fitness_function.calculate_fitness(&mutated_neural_net);
            let new_individual = Individual::new(mutated_neural_net, new_fitness);
            new_population.push(new_individual);
        }

        new_population
    }
}

#[cfg(test)]
mod add_node_test {
    use crate::genome::fitness_function::FitnessFunction;
    use crate::genome::population::{Individual, Population, ProblemType};
    use crate::mutation::neural_mutation::add_node::AddNode;
    use crate::neural_network::neural_network::NeuralNetwork;

    const DEFAULT_SEED: &[u8; 32] = &[
        1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
        3, 4,
    ];
    #[derive(Default, Copy, Clone, Debug)]
    struct TestNeuralNetworkFitnessFunction;
    impl FitnessFunction for TestNeuralNetworkFitnessFunction {
        type T = NeuralNetwork;

        fn calculate_fitness(&mut self, individual: &NeuralNetwork) -> f64 {
            1.0
        }
    }
    #[test]
    fn mutation_add_test() {
        let xs: [u32; 1] = [1];
        let data = 1;
        let net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);

        assert_eq!(net.hidden_layer_length(0), 1);

        let fitness_function = Box::new(TestNeuralNetworkFitnessFunction::default());
        let new_indv = Individual::new(net, 1.0);
        let mut new_pop = Population::new(vec![new_indv], ProblemType::Max);
        let mut add_node = AddNode::new(1.0, 0.0, *DEFAULT_SEED);
        new_pop.mutate(&mut add_node, fitness_function.clone());

        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.hidden_layer_length(0), 2);

        new_pop.mutate(&mut add_node, fitness_function.clone());
        new_pop.mutate(&mut add_node, fitness_function.clone());
        new_pop.mutate(&mut add_node, fitness_function.clone());
        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.hidden_layer_length(0), 5);

        let mut add_node = AddNode::new(1.0, 1.0, *DEFAULT_SEED);
        new_pop.mutate(&mut add_node, fitness_function.clone());
        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.hidden_layer_length(0), 5);
        assert_eq!(net.hidden_layer_length(1), 1);

        new_pop.mutate(&mut add_node, fitness_function.clone());
        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.hidden_layer_length(0), 5);
        assert_eq!(net.hidden_layer_length(1), 1);
        assert_eq!(net.hidden_layer_length(2), 1);

        let mut add_node = AddNode::new(0.0, 0.0, *DEFAULT_SEED);
        new_pop.mutate(&mut add_node, fitness_function);
        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.hidden_layer_length(0), 5);
        assert_eq!(net.hidden_layer_length(1), 1);
        assert_eq!(net.hidden_layer_length(2), 1);
    }

    #[test]
    fn mutation_add_test_no_hidden_nodes() {
        let xs: [u32; 0] = [];
        let data = 1;
        let net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);
        let fitness_function = Box::new(TestNeuralNetworkFitnessFunction::default());
        let new_indv = Individual::new(net, 1.0);
        let mut new_pop = Population::new(vec![new_indv], ProblemType::Max);
        let mut add_node = AddNode::new(1.0, 0.0, *DEFAULT_SEED);
        new_pop.mutate(&mut add_node, fitness_function.clone());
        let net = new_pop.list_of_individuals()[0].retrieve_individual();

        assert_eq!(net.hidden_layer_length(0), 1);
        new_pop.mutate(&mut add_node, fitness_function.clone());
        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.hidden_layer_length(0), 2);
    }
}
