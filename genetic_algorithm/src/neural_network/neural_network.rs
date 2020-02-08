use nalgebra::DMatrix;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::borrow::Borrow;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct NeuralNetwork {
    inputs: Vec<NeuralNode>,
    hidden: Vec<Vec<NeuralNode>>,
    outputs: Vec<NeuralNode>,
    bias: Vec<Vec<NeuralNode>>,
    num_inputs: u32,
    seed: StdRng,
}

#[derive(Debug)]
struct NeuralNode {
    node_val: f64,
    connection_weights: Vec<f64>,
}
impl NeuralNode {
    pub fn new(node_val: f64, connection_weights: Vec<f64>) -> NeuralNode {
        NeuralNode {
            node_val,
            connection_weights,
        }
    }
}

impl NeuralNetwork {
    pub fn new(
        num_inputs: u32,
        hidden: &[u32],
        outputs_data: &[f64],
        seed: [u8; 32],
    ) -> NeuralNetwork {
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        let mut input_layer = Vec::new();

        // Get the number of connections the input layer has to connect to. If we have a hidden layer defined, connect inputs to first layer of hidden.
        // else connect inputs to outputs.
        let num_input_connections = if !hidden.is_empty() {
            hidden[0]
        } else {
            u32::try_from(outputs_data.len()).unwrap()
        };

        for _ in 0..num_inputs {
            let mut weights = Vec::new();
            for _ in 0..num_input_connections {
                weights.push(rng.gen::<f64>())
            }
            input_layer.push(NeuralNode::new(0.0, weights));
        }

        let mut hidden_layer: Vec<Vec<NeuralNode>> = Vec::new();
        if !hidden.is_empty() {
            for current_layer in 0..hidden.len() - 1 {
                let mut inter_hidden_layer = Vec::new();
                for _ in 0..hidden[current_layer] {
                    let mut weights = Vec::new();
                    for _ in 0..hidden[current_layer + 1] {
                        weights.push(rng.gen::<f64>())
                    }
                    inter_hidden_layer.push(NeuralNode::new(rng.gen::<f64>(), weights))
                }
                hidden_layer.push(inter_hidden_layer);
            }
            let mut inter_hidden_layer = Vec::new();
            for _ in 0..hidden[hidden.len() - 1] {
                let mut weights = Vec::new();
                for _ in outputs_data.into_iter() {
                    weights.push(rng.gen::<f64>())
                }
                inter_hidden_layer.push(NeuralNode::new(rng.gen::<f64>(), weights));
            }
            hidden_layer.push(inter_hidden_layer);
        }
        let mut output_layer = Vec::new();
        for output_node_val in outputs_data.into_iter() {
            output_layer.push(NeuralNode::new(*output_node_val, Vec::new()));
        }

        let mut bias_layer = Vec::new();

        for current_layer in 0..hidden.len() {
            let mut inter_bias_layer = Vec::new();
            // Create 1 bias node per hidden layer
            for _ in 0..1 {
                let mut weights = Vec::new();
                for _ in 0..hidden[current_layer] {
                    weights.push(rng.gen::<f64>())
                }
                inter_bias_layer.push(NeuralNode::new(rng.gen::<f64>(), weights));
            }
            bias_layer.push(inter_bias_layer);
        }

        NeuralNetwork {
            inputs: input_layer,
            hidden: hidden_layer,
            outputs: output_layer,
            bias: bias_layer,
            num_inputs,
            seed: rng,
        }
    }
    pub fn feedforward(&self, inputs: &[f64]) -> Vec<f64> {
        let mut layer_output: Vec<f64> = vec![0.0; self.inputs[0].connection_weights.len()];
        for input_node in self.inputs.iter() {
            for (index, weights) in input_node.connection_weights.iter().enumerate() {
                layer_output[index] += inputs[index] * weights;
            }
        }

        if !self.hidden.is_empty() {
            for (hidden_layer_index, hidden_layer) in self.hidden.iter().enumerate() {
                for bias_node in self.bias[hidden_layer_index].iter() {
                    for (weight_index, weights) in bias_node.connection_weights.iter().enumerate() {
                        layer_output[weight_index] += weights * bias_node.node_val;
                    }
                }
                for index in 0..layer_output.len() {
                    layer_output[index] = sigmoid(layer_output[index].borrow());
                }
                let mut next_layer_output: Vec<f64> =
                    vec![0.0; hidden_layer[hidden_layer_index].connection_weights.len()];
                for (hidden_node_index, hidden_node) in hidden_layer.iter().enumerate() {
                    for (weight_index, weights) in hidden_node.connection_weights.iter().enumerate()
                    {
                        next_layer_output[weight_index] += layer_output[hidden_node_index] * weights
                    }
                }
                layer_output = next_layer_output;
            }
            //            for bias_node in self.bias[index].iter() {
            //                for (index, weights) in bias_node.connection_weights.iter().enumerate() {
            //                    layer_output[index] += weights * bias_node.node_val;
            //                }
            //            }
        }
        for index in 0..layer_output.len() {
            layer_output[index] = sigmoid(layer_output[index].borrow());
        }
        layer_output.to_vec()
    }

    //    fn calculate_layer() -> Vec<f64> {}
    //    pub fn feedforward(&mut self, inputs: &[f64]) -> Vec<f64> {
    //        let dm1 = DMatrix::from_vec(
    //            4,
    //            3,
    //            vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
    //        );
    //        let mut input_vec: Vec<f64> = Vec::new();
    //        for node in self.inputs.iter_mut() {
    //            input_vec.append(&mut node.connection_weights);
    //        }
    //
    //        let next_layer_len = if !self.hidden.is_empty() {
    //            self.hidden[0].len()
    //        } else {
    //            self.outputs.len()
    //        };
    //        // rows, cols
    //        let input_weight_matrix = DMatrix::from_vec(self.inputs.len(), next_layer_len, input_vec);
    //        // assume 1 row of training data for now
    //        let input_data_matrix = DMatrix::from_vec(1, self.inputs.len(), inputs.to_vec());
    //        //println!("{:?}", input_weight_matrix);
    //        // Matrix multiply input data with input layer weights
    //        let mut output_matrix = (input_weight_matrix * input_data_matrix);
    //
    //        if !self.hidden.is_empty() {
    //            let input_bias_matrix = DMatrix::from_vec(next_layer_len, self.);
    //            for hidden_layer in self.hidden.iter_mut() {
    //                let mut hidden_layer_data = Vec::new();
    //                for node in hidden_layer.into_iter() {
    //                    hidden_layer_data.append(&mut node.connection_weights);
    //                }
    //                let hidden_layer_matrix = DMatrix::from_vec(
    //                    hidden_layer.len(),
    //                    hidden_layer[0].connection_weights.len(),
    //                    hidden_layer_data,
    //                );
    //                output_matrix = hidden_layer_matrix * output_matrix;
    //            }
    //        }
    //        let mut output_vec = Vec::new();
    //        for val in output_matrix.into_iter() {
    //            output_vec.push(val.clone());
    //        }
    //        output_vec
    //    }
}

impl std::fmt::Display for NeuralNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Input Layer: {:#?}", self.inputs);
        write!(f, "Hidden Layer: {:#?}", self.hidden);
        write!(f, "Output Layer: {:#?}", self.outputs);
        write!(f, "Bias Layer: {:#?}", self.bias);
        write!(f, "Seed: {:#?}", self.seed)
    }
}

impl std::fmt::Display for NeuralNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Node val: {}, Node Weights: {:#?}",
            self.node_val, self.connection_weights
        )
    }
}

fn sigmoid(a: &f64) -> f64 {
    1.0 / (1.0 + (-a).exp())
}

#[cfg(test)]
mod neural_network_test {
    use crate::neural_network::neural_network::NeuralNetwork;
    use std::borrow::BorrowMut;
    const DEFAULT_SEED: &[u8; 32] = &[
        1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
        3, 4,
    ];
    #[test]
    fn network_creation_test() {
        let xs: [u32; 5] = [1, 2, 3, 4, 5];
        let data: [f64; 1] = [1.0];
        let net = NeuralNetwork::new(5, xs.as_ref(), data.as_ref(), *DEFAULT_SEED);

        assert_eq!(net.num_inputs, 5);
        assert_eq!(net.outputs.len(), 1);
        assert_eq!(net.bias.len(), 5);
        assert_eq!(net.bias[0].len(), 1);
        assert_eq!(net.bias[1].len(), 1);
        assert_eq!(net.bias[2].len(), 1);
        assert_eq!(net.bias[3].len(), 1);
        assert_eq!(net.bias[4].len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 2);
        assert_eq!(net.bias[2][0].connection_weights.len(), 3);
        assert_eq!(net.bias[3][0].connection_weights.len(), 4);
        assert_eq!(net.bias[4][0].connection_weights.len(), 5);
        assert_eq!(net.hidden.len(), 5);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[1].len(), 2);
        assert_eq!(net.hidden[2].len(), 3);
        assert_eq!(net.hidden[3].len(), 4);
        assert_eq!(net.hidden[4].len(), 5);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 2);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[2][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[3][0].connection_weights.len(), 5);
        assert_eq!(net.hidden[4][0].connection_weights.len(), 1);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.inputs[1].connection_weights.len(), 1);
        assert_eq!(net.inputs[2].connection_weights.len(), 1);
        assert_eq!(net.inputs[3].connection_weights.len(), 1);
        assert_eq!(net.inputs[4].connection_weights.len(), 1);
        let xs: [u32; 0] = [];
        let data: [f64; 2] = [1.0, 1.0];
        let net = NeuralNetwork::new(5, xs.as_ref(), data.as_ref(), *DEFAULT_SEED);
        assert_eq!(net.num_inputs, 5);
        assert_eq!(net.outputs.len(), 2);
        assert_eq!(net.bias.len(), 0);
        assert_eq!(net.hidden.len(), 0);
        assert_eq!(net.inputs[0].connection_weights.len(), 2);
        assert_eq!(net.inputs[1].connection_weights.len(), 2);
        assert_eq!(net.inputs[2].connection_weights.len(), 2);
        assert_eq!(net.inputs[3].connection_weights.len(), 2);
        assert_eq!(net.inputs[4].connection_weights.len(), 2);
    }

    #[test]
    fn network_feedforward_test() {
        let xs: [u32; 1] = [1];
        let data: [f64; 1] = [1.0];
        let mut net = NeuralNetwork::new(1, xs.as_ref(), data.as_ref(), *DEFAULT_SEED);
        let inputs = [1.0];

        let mut output = net.feedforward(inputs.as_ref());

        output = net.feedforward(inputs.as_ref());
        assert_eq!(output[0], 0.6035386944499268);
    }
}
