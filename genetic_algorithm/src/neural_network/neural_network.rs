use nalgebra::DMatrix;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::borrow::{Borrow, BorrowMut};
use std::cmp;
use std::convert::TryFrom;

const BIAS_VALUE: f64 = 1.0;

#[derive(Clone, Debug)]
pub struct NeuralNetwork {
    inputs: Vec<NeuralNode>,
    hidden: Vec<Vec<NeuralNode>>,
    outputs: Vec<NeuralNode>,
    bias: Vec<Vec<NeuralNode>>,
    seed: StdRng,
}

#[derive(Clone, Debug)]
struct NeuralNode {
    connection_weights: Vec<f64>,
}
impl NeuralNode {
    pub fn new(connection_weights: Vec<f64>) -> NeuralNode {
        NeuralNode { connection_weights }
    }
    pub fn retrieve_connection_weights(&self) -> &Vec<f64> {
        &self.connection_weights
    }
}

impl NeuralNetwork {
    pub fn hidden_layer_swap_and_create_new_from(
        &mut self,
        net_to_exchange: &NeuralNetwork,
    ) -> Option<NeuralNetwork> {
        // TODO: Swap inner nodes in a layer

        if self.is_hidden_layer_empty() && net_to_exchange.is_hidden_layer_empty() {
            return None;
        }

        // Randomly pick inputs from the first or second network
        let gen_val = self.seed.gen::<f64>();
        let new_inputs = if self.seed.gen_bool(gen_val) {
            self.inputs.clone()
        } else {
            net_to_exchange.inputs.clone()
        };

        // TODO: Refactor to it's own method to use for hidden and bias logic.
        let new_hidden: Vec<Vec<NeuralNode>> = {
            let mut new_hidden_layer: Vec<Vec<NeuralNode>> = Vec::new();

            // randomly choose between the first net or the second.
            let gen_val = self.seed.gen::<f64>();
            let mut toggle_value = self.seed.gen_bool(gen_val);
            let min_length = cmp::min(self.hidden.len(), net_to_exchange.hidden.len());
            for layer_index in 0..min_length {
                if toggle_value {
                    new_hidden_layer.push(self.hidden[layer_index].clone())
                } else {
                    new_hidden_layer.push(net_to_exchange.hidden[layer_index].clone())
                }
                toggle_value ^= true;
            }

            if self.hidden.len() > net_to_exchange.hidden.len() {
                let start = (self.hidden.len() - net_to_exchange.hidden.len()) - 1;
                for layer_index in start..self.hidden.len() {
                    new_hidden_layer.push(self.hidden[layer_index].clone())
                }
            } else if self.hidden.len() < net_to_exchange.hidden.len() {
                let start = (net_to_exchange.hidden.len() - self.hidden.len()) - 1;
                for layer_index in start..net_to_exchange.hidden.len() {
                    new_hidden_layer.push(net_to_exchange.hidden[layer_index].clone())
                }
            }

            new_hidden_layer
        };

        let new_bias: Vec<Vec<NeuralNode>> = {
            let mut new_bias_layer: Vec<Vec<NeuralNode>> = Vec::new();
            // randomly choose between the first net or the second.
            let gen_val = self.seed.gen::<f64>();
            let mut toggle_value = self.seed.gen_bool(gen_val);
            let min_length = cmp::min(self.bias.len(), net_to_exchange.bias.len());
            for layer_index in 0..min_length {
                if toggle_value {
                    new_bias_layer.push(self.bias[layer_index].clone())
                } else {
                    new_bias_layer.push(net_to_exchange.bias[layer_index].clone())
                }
                toggle_value ^= true;
            }

            if self.bias.len() > net_to_exchange.bias.len() {
                let start = (self.bias.len() - net_to_exchange.bias.len()) - 1;
                for layer_index in start..self.bias.len() {
                    new_bias_layer.push(self.bias[layer_index].clone())
                }
            } else if self.bias.len() < net_to_exchange.bias.len() {
                let start = (net_to_exchange.bias.len() - self.bias.len()) - 1;
                for layer_index in start..net_to_exchange.bias.len() {
                    new_bias_layer.push(net_to_exchange.bias[layer_index].clone())
                }
            }
            new_bias_layer
        };

        let mut new_net = NeuralNetwork {
            inputs: new_inputs,
            hidden: new_hidden,
            outputs: self.outputs.clone(),
            bias: new_bias,
            seed: self.seed.clone(),
        };

        new_net.fix_weights();

        Some(new_net)
    }
    fn fix_weights(&mut self) {
        // TODO: Refactor methods
        if self.is_hidden_layer_empty() {
            for node in self.inputs.iter_mut() {
                if self.outputs.len() > node.connection_weights.len() {
                    let weights_to_add = self.outputs.len() - node.connection_weights.len();
                    for _ in 0..weights_to_add {
                        node.connection_weights.push(self.seed.gen::<f64>());
                    }
                } else if self.outputs.len() < node.connection_weights.len() {
                    let weights_to_remove = node.connection_weights.len() - self.outputs.len();
                    for _ in 0..weights_to_remove {
                        node.connection_weights
                            .remove(node.connection_weights.len() - 1);
                    }
                }
            }
            return;
        }

        for node in self.inputs.iter_mut() {
            if self.hidden[0].len() > node.connection_weights.len() {
                let weights_to_add = self.hidden[0].len() - node.connection_weights.len();
                for _ in 0..weights_to_add {
                    node.connection_weights.push(self.seed.gen::<f64>());
                }
            } else if self.hidden[0].len() < node.connection_weights.len() {
                let weights_to_remove = node.connection_weights.len() - self.hidden[0].len();
                for _ in 0..weights_to_remove {
                    node.connection_weights
                        .remove(node.connection_weights.len() - 1);
                }
            }
        }

        for hidden_layer in 0..self.hidden.len() - 1 {
            let next_hidden_layer_len = self.hidden[hidden_layer + 1].len();
            for node in self.hidden[hidden_layer].iter_mut() {
                if next_hidden_layer_len > node.connection_weights.len() {
                    let weights_to_add = next_hidden_layer_len - node.connection_weights.len();
                    for _ in 0..weights_to_add {
                        node.connection_weights.push(self.seed.gen::<f64>());
                    }
                } else if next_hidden_layer_len < node.connection_weights.len() {
                    let weights_to_remove = node.connection_weights.len() - next_hidden_layer_len;
                    for _ in 0..weights_to_remove {
                        node.connection_weights
                            .remove(node.connection_weights.len() - 1);
                    }
                }
            }
        }
        let last_index = self.hidden.len() - 1;

        for node in self.hidden[last_index].iter_mut() {
            if self.outputs.len() > node.connection_weights.len() {
                let weights_to_add = self.outputs.len() - node.connection_weights.len();
                for _ in 0..weights_to_add {
                    node.connection_weights.push(self.seed.gen::<f64>());
                }
            } else if self.outputs.len() < node.connection_weights.len() {
                let weights_to_remove = node.connection_weights.len() - self.outputs.len();
                for _ in 0..weights_to_remove {
                    node.connection_weights
                        .remove(node.connection_weights.len() - 1);
                }
            }
        }

        for bias_layer_index in 0..self.bias.len() {
            for node in self.bias[bias_layer_index].iter_mut() {
                if self.hidden[bias_layer_index].len() > node.connection_weights.len() {
                    let weights_to_add =
                        self.hidden[bias_layer_index].len() - node.connection_weights.len();
                    for _ in 0..weights_to_add {
                        node.connection_weights.push(self.seed.gen::<f64>());
                    }
                } else if self.hidden[bias_layer_index].len() < node.connection_weights.len() {
                    let weights_to_remove =
                        node.connection_weights.len() - self.hidden[bias_layer_index].len();
                    for _ in 0..weights_to_remove {
                        node.connection_weights
                            .remove(node.connection_weights.len() - 1);
                    }
                }
            }
        }
    }
    pub fn hidden_layer_length(&self, index: usize) -> usize {
        self.hidden[index].len()
    }
    pub fn hidden_length(&self) -> usize {
        self.hidden.len()
    }
    pub fn is_hidden_layer_empty(&self) -> bool {
        self.hidden.is_empty()
    }

    // Will delete hidden node layer if the current hidden node layer has 1 node.
    pub fn remove_hidden_node(&mut self, layer_index: usize) {
        let mut deleted_layer = false;
        if !self.is_hidden_layer_empty() && self.hidden[layer_index].len() == 1 {
            self.hidden.remove(layer_index);
            self.bias.remove(layer_index);
            deleted_layer = true;
        } else {
            let last_element_index = self.hidden[layer_index].len() - 1;
            self.hidden[layer_index].remove(last_element_index);
        }

        self.fix_weights();
    }

    pub fn push_hidden_node(&mut self, layer_index: usize) {
        let number_of_conn_weights = if self.hidden.len() - 1 == layer_index {
            self.outputs.len()
        } else {
            self.hidden[layer_index + 1].len()
        };

        let mut weights = Vec::new();

        for _ in 0..number_of_conn_weights {
            weights.push(self.seed.gen::<f64>());
        }

        self.hidden[layer_index].push(NeuralNode::new(weights));
        self.fix_weights();
    }

    pub fn push_hidden_node_with_new_layer_at_end(&mut self) {
        let mut new_layer = Vec::new();

        let mut weights = Vec::new();
        for _ in 0..self.outputs.len() {
            weights.push(self.seed.gen::<f64>());
        }
        new_layer.push(NeuralNode::new(weights));

        self.hidden.push(new_layer);
        if self.hidden.len() > 1 {
            let layer_before_index = self.hidden.len() - 2;
            for nodes in self.hidden[layer_before_index].iter_mut() {
                nodes.connection_weights.push(self.seed.gen::<f64>());
            }
        } else {
            for nodes in self.inputs.iter_mut() {
                nodes.connection_weights.push(self.seed.gen::<f64>());
            }
        }
        let mut new_bias_layer = Vec::new();
        new_bias_layer.push(NeuralNode::new(vec![self.seed.gen::<f64>()]));
        self.bias.push(new_bias_layer);
    }
    pub fn new(num_inputs: u32, hidden: &[u32], num_outputs: u32, seed: [u8; 32]) -> NeuralNetwork {
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        let mut input_layer = Vec::new();

        // Get the number of connections the input layer has to connect to. If we have a hidden layer defined, connect inputs to first layer of hidden.
        // else connect inputs to outputs.
        let num_input_connections = if !hidden.is_empty() {
            hidden[0]
        } else {
            u32::try_from(num_outputs).unwrap()
        };

        for _ in 0..num_inputs {
            let mut weights = Vec::new();
            for _ in 0..num_input_connections {
                weights.push(rng.gen::<f64>())
            }
            input_layer.push(NeuralNode::new(weights));
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
                    inter_hidden_layer.push(NeuralNode::new(weights))
                }
                hidden_layer.push(inter_hidden_layer);
            }
            let mut inter_hidden_layer = Vec::new();
            for _ in 0..hidden[hidden.len() - 1] {
                let mut weights = Vec::new();
                for _ in 0..num_outputs {
                    weights.push(rng.gen::<f64>())
                }
                inter_hidden_layer.push(NeuralNode::new(weights));
            }
            hidden_layer.push(inter_hidden_layer);
        }
        let mut output_layer = Vec::new();
        for _ in 0..num_outputs {
            output_layer.push(NeuralNode::new(Vec::new()));
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
                inter_bias_layer.push(NeuralNode::new(weights));
            }
            bias_layer.push(inter_bias_layer);
        }

        NeuralNetwork {
            inputs: input_layer,
            hidden: hidden_layer,
            outputs: output_layer,
            bias: bias_layer,
            seed: rng,
        }
    }
    pub fn feedforward(&self, inputs: &[f64]) -> Vec<f64> {
        let mut layer_output: Vec<f64> = vec![0.0; self.inputs[0].connection_weights.len()];
        for (input_index, input_node) in self.inputs.iter().enumerate() {
            for (index, weights) in input_node.connection_weights.iter().enumerate() {
                layer_output[index] += inputs[input_index] * weights;
            }
        }

        if !self.hidden.is_empty() {
            for (hidden_layer_index, hidden_layer) in self.hidden.iter().enumerate() {
                for bias_node in self.bias[hidden_layer_index].iter() {
                    for (weight_index, weights) in bias_node.connection_weights.iter().enumerate() {
                        layer_output[weight_index] += weights * BIAS_VALUE;
                    }
                }
                for index in 0..layer_output.len() {
                    layer_output[index] = sigmoid(layer_output[index].borrow());
                }

                let mut next_layer_output: Vec<f64> =
                    vec![0.0; hidden_layer[0].connection_weights.len()];
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
        write!(f, "Node Weights: {:#?}", self.connection_weights)
    }
}

fn sigmoid(a: &f64) -> f64 {
    1.0 / (1.0 + (-a).exp())
}

#[cfg(test)]
mod neural_network_test {
    use crate::neural_network::neural_network::NeuralNetwork;
    use std::borrow::{Borrow, BorrowMut};
    const DEFAULT_SEED: &[u8; 32] = &[
        1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
        3, 4,
    ];
    #[test]
    fn network_creation_test() {
        let xs: [u32; 5] = [1, 2, 3, 4, 5];
        let data = 1;
        let net = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);

        assert_eq!(net.inputs.len(), 5);
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
        let data = 2;
        let net = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);
        assert_eq!(net.inputs.len(), 5);
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
        let data = 1;
        let mut net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);
        let inputs = [1.0];

        let mut output = net.feedforward(inputs.as_ref());

        output = net.feedforward(inputs.as_ref());
        assert_eq!(output[0], 0.6126923921759762);

        let xs: [u32; 5] = [10, 10, 10, 10, 10];
        let data = 1;
        let net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);
        let inputs = [0.10];

        let mut output = net.feedforward(inputs.as_ref());

        output = net.feedforward(inputs.as_ref());
        assert_eq!(output[0], 0.9970523923651378);
    }

    #[test]
    fn network_add_node() {
        let xs: [u32; 1] = [1];
        let data = 1;
        let mut net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);

        assert_eq!(net.hidden.len(), 1);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);

        net.push_hidden_node(0);

        assert_eq!(net.hidden[0].len(), 2);
        assert_eq!(net.inputs[0].connection_weights.len(), 2);
        assert_eq!(net.hidden[0][1].connection_weights.len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 2);

        net.push_hidden_node(0);

        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][2].connection_weights.len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);

        let xs: [u32; 2] = [2, 2];
        let data = 1;
        let mut net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);

        assert_eq!(net.hidden[0].len(), 2);
        assert_eq!(net.inputs[0].connection_weights.len(), 2);
        assert_eq!(net.hidden[0][1].connection_weights.len(), 2);
        assert_eq!(net.bias[0][0].connection_weights.len(), 2);

        net.push_hidden_node(1);

        assert_eq!(net.hidden[0].len(), 2);
        assert_eq!(net.hidden[1].len(), 3);
        assert_eq!(net.hidden[0][1].connection_weights.len(), 3);
        assert_eq!(net.hidden[1][1].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 3);
    }

    #[test]
    fn network_add_node_with_new_layer() {
        let xs: [u32; 0] = [];
        let data = 1;
        let mut net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);

        assert_eq!(net.hidden.len(), 0);
        assert_eq!(net.bias.len(), 0);

        net.push_hidden_node_with_new_layer_at_end();

        assert_eq!(net.hidden.len(), 1);
        assert_eq!(net.bias.len(), 1);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 1);
        net.push_hidden_node_with_new_layer_at_end();

        assert_eq!(net.hidden.len(), 2);
        assert_eq!(net.bias.len(), 2);
        assert_eq!(net.hidden[1].len(), 1);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 1);
    }

    #[test]
    fn network_remove_node() {
        let xs: [u32; 5] = [1, 2, 3, 4, 5];
        let data = 2;
        let mut net = NeuralNetwork::new(1, xs.as_ref(), data, *DEFAULT_SEED);

        assert_eq!(net.hidden.len(), 5);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[1].len(), 2);
        assert_eq!(net.hidden[2].len(), 3);
        assert_eq!(net.hidden[3].len(), 4);
        assert_eq!(net.hidden[4].len(), 5);
        assert_eq!(net.bias.len(), 5);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 2);
        assert_eq!(net.bias[2][0].connection_weights.len(), 3);
        assert_eq!(net.bias[3][0].connection_weights.len(), 4);
        assert_eq!(net.bias[4][0].connection_weights.len(), 5);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 2);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[2][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[3][0].connection_weights.len(), 5);
        net.remove_hidden_node(0);

        assert_eq!(net.hidden.len(), 4);
        assert_eq!(net.hidden[0].len(), 2);
        assert_eq!(net.hidden[1].len(), 3);
        assert_eq!(net.hidden[2].len(), 4);
        assert_eq!(net.hidden[3].len(), 5);
        assert_eq!(net.bias.len(), 4);
        assert_eq!(net.bias[0][0].connection_weights.len(), 2);
        assert_eq!(net.bias[1][0].connection_weights.len(), 3);
        assert_eq!(net.bias[2][0].connection_weights.len(), 4);
        assert_eq!(net.bias[3][0].connection_weights.len(), 5);
        assert_eq!(net.inputs[0].connection_weights.len(), 2);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[2][0].connection_weights.len(), 5);
        net.remove_hidden_node(0);

        assert_eq!(net.hidden.len(), 4);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[1].len(), 3);
        assert_eq!(net.hidden[2].len(), 4);
        assert_eq!(net.hidden[3].len(), 5);
        assert_eq!(net.bias.len(), 4);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 3);
        assert_eq!(net.bias[2][0].connection_weights.len(), 4);
        assert_eq!(net.bias[3][0].connection_weights.len(), 5);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[2][0].connection_weights.len(), 5);

        net.remove_hidden_node(0);
        assert_eq!(net.hidden.len(), 3);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 4);
        assert_eq!(net.hidden[2].len(), 5);
        assert_eq!(net.bias.len(), 3);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 4);
        assert_eq!(net.bias[2][0].connection_weights.len(), 5);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 5);

        net.remove_hidden_node(2);
        assert_eq!(net.hidden.len(), 3);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 4);
        assert_eq!(net.hidden[2].len(), 4);
        assert_eq!(net.bias.len(), 3);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 4);
        assert_eq!(net.bias[2][0].connection_weights.len(), 4);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 4);

        net.remove_hidden_node(2);
        assert_eq!(net.hidden.len(), 3);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 4);
        assert_eq!(net.hidden[2].len(), 3);
        assert_eq!(net.bias.len(), 3);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 4);
        assert_eq!(net.bias[2][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 3);

        net.remove_hidden_node(2);
        assert_eq!(net.hidden.len(), 3);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 4);
        assert_eq!(net.hidden[2].len(), 2);
        assert_eq!(net.bias.len(), 3);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 4);
        assert_eq!(net.bias[2][0].connection_weights.len(), 2);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 2);

        net.remove_hidden_node(2);
        assert_eq!(net.hidden.len(), 3);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 4);
        assert_eq!(net.hidden[2].len(), 1);
        assert_eq!(net.bias.len(), 3);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 4);
        assert_eq!(net.bias[2][0].connection_weights.len(), 1);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 1);

        net.remove_hidden_node(2);
        assert_eq!(net.hidden.len(), 2);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 4);
        assert_eq!(net.bias.len(), 2);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 4);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 4);
        assert_eq!(net.hidden[1][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 2);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 3);
        assert_eq!(net.bias.len(), 2);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[1][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 2);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 2);
        assert_eq!(net.bias.len(), 2);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 2);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 2);
        assert_eq!(net.hidden[1][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 2);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.hidden[1].len(), 1);
        assert_eq!(net.bias.len(), 2);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.bias[1][0].connection_weights.len(), 1);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 1);
        assert_eq!(net.hidden[1][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 1);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.bias.len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(0);
        assert_eq!(net.hidden.len(), 1);
        assert_eq!(net.hidden[0].len(), 2);
        assert_eq!(net.bias.len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 2);
        assert_eq!(net.inputs[0].connection_weights.len(), 2);
        assert_eq!(net.hidden[0][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(0);
        assert_eq!(net.hidden.len(), 1);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.bias.len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(0);
        assert_eq!(net.hidden.len(), 0);
        assert_eq!(net.bias.len(), 0);
        assert_eq!(net.inputs[0].connection_weights.len(), data as usize);

        let xs: [u32; 6] = [1, 1, 1, 1, 1, 3];
        let data = 1;
        let mut net = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);
        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 5);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[1].len(), 1);
        assert_eq!(net.bias.len(), 5);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 1);
        assert_eq!(net.bias[2][0].connection_weights.len(), 1);
        assert_eq!(net.bias[3][0].connection_weights.len(), 1);
        assert_eq!(net.bias[4][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 1);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 1);
        assert_eq!(net.hidden[2][0].connection_weights.len(), 1);
        assert_eq!(net.hidden[3][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[4][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 4);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[1].len(), 1);
        assert_eq!(net.bias.len(), 4);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 1);
        assert_eq!(net.bias[2][0].connection_weights.len(), 1);
        assert_eq!(net.bias[3][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 1);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 1);
        assert_eq!(net.hidden[2][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[3][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 3);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[1].len(), 1);
        assert_eq!(net.bias.len(), 3);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 1);
        assert_eq!(net.bias[2][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 1);
        assert_eq!(net.hidden[1][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[2][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(1);
        assert_eq!(net.hidden.len(), 2);
        assert_eq!(net.hidden[0].len(), 1);
        assert_eq!(net.hidden[1].len(), 3);
        assert_eq!(net.bias.len(), 2);
        assert_eq!(net.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net.bias[1][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 1);
        assert_eq!(net.hidden[0][0].connection_weights.len(), 3);
        assert_eq!(net.hidden[1][0].connection_weights.len(), data as usize);

        net.remove_hidden_node(0);
        assert_eq!(net.hidden.len(), 1);
        assert_eq!(net.hidden[0].len(), 3);
        assert_eq!(net.bias.len(), 1);
        assert_eq!(net.bias[0][0].connection_weights.len(), 3);
        assert_eq!(net.inputs[0].connection_weights.len(), 3);
        assert_eq!(net.hidden[0][0].connection_weights.len(), data as usize);
    }
    #[test]
    fn network_hidden_layer_swap_and_create_new_from_test() {
        let xs: [u32; 0] = [];
        let data = 1;
        let mut net_one = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);

        let mut net_two = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);

        let net_three = net_one.hidden_layer_swap_and_create_new_from(net_two.borrow());

        assert_eq!(net_three.is_none(), true);
        let xs: [u32; 1] = [1];
        net_two = NeuralNetwork::new(5, xs.as_ref(), data, *DEFAULT_SEED);

        let net_three = net_one.hidden_layer_swap_and_create_new_from(net_two.borrow());

        let net_three_val = net_three.unwrap();
        assert_eq!(net_three_val.hidden.len(), 1);
        assert_eq!(net_three_val.bias.len(), 1);
        assert_eq!(net_three_val.inputs.len(), 5);
        for node in net_three_val.inputs.iter() {
            assert_eq!(node.connection_weights.len(), 1);
        }
        assert_eq!(net_three_val.bias.len(), 1);
        assert_eq!(net_three_val.bias[0][0].connection_weights.len(), 1);
        assert_eq!(net_three_val.hidden[0][0].connection_weights.len(), 1);

        let xs: [u32; 5] = [1, 2, 3, 4, 5];
        let data = 5;
        let mut net_one = NeuralNetwork::new(10, xs.as_ref(), data, *DEFAULT_SEED);
        let xs: [u32; 2] = [8, 8];
        let data = 5;
        let mut net_two = NeuralNetwork::new(8, xs.as_ref(), data, *DEFAULT_SEED);

        let net_three = net_one.hidden_layer_swap_and_create_new_from(net_two.borrow());

        let net_three_val = net_three.unwrap();

        assert_eq!(net_three_val.hidden.len(), 5);
        assert_eq!(net_three_val.bias.len(), 5);
        assert_eq!(net_three_val.inputs.len(), 8);
        for node in net_three_val.inputs.iter() {
            assert_eq!(node.connection_weights.len(), 8);
        }
        assert_eq!(net_three_val.bias.len(), 5);
        assert_eq!(net_three_val.bias[0][0].connection_weights.len(), 8);
        assert_eq!(net_three_val.hidden[0].len(), 8);
        for index in 0..net_three_val.hidden[0].len() {
            assert_eq!(net_three_val.hidden[0][index].connection_weights.len(), 2);
        }
        assert_eq!(net_three_val.hidden[1].len(), 2);
        for index in 0..net_three_val.hidden[1].len() {
            assert_eq!(net_three_val.hidden[1][index].connection_weights.len(), 3);
        }
        assert_eq!(net_three_val.bias[1][0].connection_weights.len(), 2);
        assert_eq!(
            net_three_val.hidden[4][0].connection_weights.len(),
            data as usize
        );
    }
}
