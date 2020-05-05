use genetic_algorithm::neural_network::neural_network::NeuralNetwork;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;

const DEFAULT_SEED: &[u8; 32] = &[
    1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 3, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
    3, 4,
];

fn main() {
    let input_label_list = read_input_labels_from_file();
    let input_images_list = read_input_images_from_file();
    println!("{:?}", input_label_list);
    println!("{:?}", input_images_list);
    let net = NeuralNetwork::new(1, vec![1], 9, *DEFAULT_SEED);
}

fn read_input_labels_from_file() -> Vec<u8> {
    let mut input_label_list = Vec::new();
    let mut labels =
        File::open("examples/genetic_algorithm_gui/neural_network/train-labels.idx1-ubyte")
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

fn read_input_images_from_file() -> Vec<[f64]> {
    let mut input_images_list: Vec<[f64]> = Vec::new();
    let mut images =
        File::open("examples/genetic_algorithm_gui/neural_network/train-images.idx3-ubyte")
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
    for _ in number_of_images {
        let mut input_images_array: Vec<f64> = Vec::new();
        for _ in number_of_columns {
            for _ in number_of_rows {
                images.read(&mut pixel_buf).unwrap();
                input_images_array.push(f64::from(pixel_buf));
            }
        }
        input_images_list.push(*input_images_array.as_slice());
    }
    input_images_list
}
