use crate::crossover::genome_crossover::{get_default_better_individual, Crossover};
use crate::genome::fitness_function::FitnessFunction;
use crate::genome::population::{Individual, ProblemType};
use crate::neural_network::neural_network::NeuralNetwork;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::borrow::Borrow;

pub struct HiddenLayerSwap {
    swap_chance: f64,
    seed: StdRng,
}

impl HiddenLayerSwap {
    pub fn new(swap_chance: f64, seed: [u8; 32]) -> HiddenLayerSwap {
        HiddenLayerSwap {
            swap_chance,
            seed: SeedableRng::from_seed(seed),
        }
    }
}

impl Crossover for HiddenLayerSwap {
    type T = NeuralNetwork;

    fn crossover(
        &mut self,
        _first_individual: &Individual<NeuralNetwork>,
        _second_individual: &Individual<NeuralNetwork>,
        fitness_function: &mut Box<dyn FitnessFunction<T = NeuralNetwork>>,
        problem_type: &ProblemType,
    ) -> Individual<NeuralNetwork> {
        let gen_number = self.seed.gen::<f64>();

        if gen_number < self.swap_chance {
            let mut indv_one_net = _first_individual.retrieve_individual().clone();
            let indv_two_net = _second_individual.retrieve_individual().clone();

            let new_net = indv_one_net.hidden_layer_swap_and_create_new_from(indv_two_net.borrow());

            return match new_net {
                None => get_default_better_individual(
                    _first_individual,
                    _second_individual,
                    problem_type,
                )
                .clone(),
                Some(x) => {
                    let new_fitness = fitness_function.calculate_fitness(x.borrow());
                    Individual::new(x, new_fitness)
                }
            };
        }
        return get_default_better_individual(_first_individual, _second_individual, problem_type)
            .clone();
    }
}

#[cfg(test)]
mod hidden_layer_swap_test {
    use crate::crossover::genome_crossover::Crossover;
    use crate::crossover::neural_crossover::node_swap::HiddenLayerSwap;
    use crate::genome::fitness_function::FitnessFunction;
    use crate::genome::population::{Individual, ProblemType};
    use crate::neural_network::neural_network::NeuralNetwork;
    use sha2::Digest;
    use std::borrow::Borrow;

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
    fn hidden_layer_crossover_test() {
        let mut hidden_layer_crossover = HiddenLayerSwap::new(1.0, *DEFAULT_SEED);

        let xs: [u32; 1] = [5];
        let data = 1;
        let mut net_one = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);
        let mut net_two = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);
        let indv_one = Individual::new(net_one, 1.0);
        let indv_two = Individual::new(net_two, 2.0);
        let mut fitness_function: Box<dyn FitnessFunction<T = NeuralNetwork>> =
            Box::new(TestNeuralNetworkFitnessFunction::default());
        let new_net = hidden_layer_crossover.crossover(
            indv_one.borrow(),
            indv_two.borrow(),
            &mut fitness_function,
            &ProblemType::Max,
        );

        assert_eq!(new_net.retrieve_individual().is_hidden_layer_empty(), false);

        hidden_layer_crossover = HiddenLayerSwap::new(0.0, *DEFAULT_SEED);
        let new_net = hidden_layer_crossover.crossover(
            indv_one.borrow(),
            indv_two.borrow(),
            &mut fitness_function,
            &ProblemType::Max,
        );

        assert_eq!(new_net.retrieve_individual().is_hidden_layer_empty(), false);
    }
}
