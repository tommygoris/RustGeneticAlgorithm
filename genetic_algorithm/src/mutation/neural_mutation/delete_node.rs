use crate::genome::fitness_function::FitnessFunction;
use crate::genome::population::{Individual, Population};
use crate::mutation::genome_mutation::Mutate;
use crate::neural_network::neural_network::NeuralNetwork;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct DeleteNode {
    delete_node_mutation_rate: f64,
    seed: StdRng,
}

impl DeleteNode {
    fn new(delete_node_mutation_rate: f64, seed: [u8; 32]) -> DeleteNode {
        DeleteNode {
            delete_node_mutation_rate,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

impl Mutate for DeleteNode {
    type T = NeuralNetwork;

    fn mutate(
        &mut self,
        population: &Population<NeuralNetwork>,
        mut fitness_function: Box<dyn FitnessFunction<T = NeuralNetwork>>,
    ) -> Vec<Individual<NeuralNetwork>> {
        let mut new_population = Vec::new();

        for individual in population.list_of_individuals().iter() {
            let gen_number = self.seed.gen::<f64>();
            if gen_number > self.delete_node_mutation_rate {
                new_population.push(individual.clone());
                continue;
            }
            let mut mutated_neural_net = individual.retrieve_individual().clone();
            let node_index = self.seed.gen_range(0, mutated_neural_net.hidden_length());
            mutated_neural_net.remove_hidden_node(node_index);
            let new_fitness = fitness_function.calculate_fitness(&mutated_neural_net);
            let new_individual = Individual::new(mutated_neural_net, new_fitness);
            new_population.push(new_individual);
        }

        new_population
    }
}

#[cfg(test)]
mod delete_node_test {
    use crate::genome::fitness_function::FitnessFunction;
    use crate::genome::population::{Individual, Population, ProblemType};
    use crate::mutation::neural_mutation::delete_node::DeleteNode;
    use crate::neural_network::neural_network::NeuralNetwork;
    use std::convert::AsRef;

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
    fn mutation_delete_test() {
        let xs: [u32; 1] = [1];
        let data = 1;
        let net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);

        assert_eq!(net.hidden_layer_length(0), 1);

        let fitness_function = Box::new(TestNeuralNetworkFitnessFunction::default());
        let new_indv = Individual::new(net, 1.0);
        let mut new_pop = Population::new(vec![new_indv], ProblemType::Max);
        let mut delete_node = DeleteNode::new(0.0, *DEFAULT_SEED);
        new_pop.mutate(&mut delete_node, fitness_function.clone());

        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.is_hidden_layer_empty(), false);

        let mut delete_node = DeleteNode::new(1.0, *DEFAULT_SEED);
        new_pop.mutate(&mut delete_node, fitness_function.clone());

        let net = new_pop.list_of_individuals()[0].retrieve_individual();
        assert_eq!(net.is_hidden_layer_empty(), true);
    }
}
