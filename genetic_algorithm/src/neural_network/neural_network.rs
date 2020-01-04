use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct NeuralNetwork {
    inputs: Vec<Vec<f64>>,
    hidden: Vec<Vec<Vec<f64>>>,
    outputs: Vec<f64>,
    num_inputs: u32,
    seed: StdRng,
}

impl NeuralNetwork {
//    pub fn new(num_inputs: u32, hidden: &[u32],  outputs_data: &[u32], seed: [u8; 32]) -> NeuralNetwork {
//        let rng = SeedableRng::from_seed(seed);
//        for _ in 0..num_inputs {
//            let inputs = Vec::new();
//            for _ in 0..hidden.len() {
//
//
//            }
//        }
//        NeuralNetwork (
//
//        )
//    }
//
//    pub fn set_inputs(inputs: &[f64]) {
//
//    }
}