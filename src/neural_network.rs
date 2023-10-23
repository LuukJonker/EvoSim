extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

use ndarray::{Array, Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use rand::Rng;

pub struct NeuralNetworkFactory {
    sizes: Vec<usize>,
    learning_rate: f64,
    merging_strategy: fn(f64, f64) -> f64,
}

impl NeuralNetworkFactory {
    pub fn new(sizes: &[usize], learning_rate: f64) -> Self {
        NeuralNetworkFactory {
            sizes: sizes.to_vec(),
            learning_rate,
            merging_strategy: |a, b| 0.5 * a + 0.5 * b,
        }
    }

    pub fn create_random(&self) -> NeuralNetwork {
        let weights: Vec<Array2<f64>> = self
            .sizes
            .iter()
            .zip(&self.sizes[1..])
            .map(|(n1, n2)| Array::random((*n2, *n1), Uniform::new(-1.0, 1.0)))
            .collect();

        let biases: Vec<Array1<f64>> = self
            .sizes
            .iter()
            .skip(1)
            .map(|n| Array::random(*n, Uniform::new(-1.0, 1.0)))
            .collect();

        NeuralNetwork {
            weights,
            biases,
            learning_rate: self.learning_rate,
        }
    }

    pub fn create_offspring(&self, net1: &NeuralNetwork, net2: &NeuralNetwork) -> NeuralNetwork {
        assert_eq!(net1.weights.len(), net2.weights.len());
        assert_eq!(net1.biases.len(), net2.biases.len());

        let weights: Vec<Array2<f64>> = net1
            .weights
            .iter()
            .zip(&net2.weights)
            .map(|(w1, w2)| w1.mapv(|x| (self.merging_strategy)(x, *w2.first().unwrap())))
            .collect();

        let biases: Vec<Array1<f64>> = net1
            .biases
            .iter()
            .zip(&net2.biases)
            .map(|(b1, b2)| b1.mapv(|x| (self.merging_strategy)(x, *b2.first().unwrap())))
            .collect();

        NeuralNetwork {
            weights,
            biases,
            learning_rate: (net1.learning_rate + net2.learning_rate) / 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    weights: Vec<Array2<f64>>,
    biases: Vec<Array1<f64>>,
    learning_rate: f64,
}

impl NeuralNetwork {
    pub fn forward(&self, input: Array1<f64>) -> Array1<f64> {
        let mut current_activation = input;

        for (w, b) in self.weights.iter().zip(&self.biases) {
            current_activation = (w.dot(&current_activation) + b).mapv(|z| sigmoid(z));
        }

        current_activation
    }

    // Add random mutations to the network
    pub fn mutate(&self, mutation_rate: f64) -> NeuralNetwork {
        let mut rng = rand::thread_rng();

        let weights: Vec<Array2<f64>> = self
            .weights
            .iter()
            .map(|w| {
                w.mapv(|x| {
                    if rng.gen::<f64>() < mutation_rate {
                        x + rng.gen_range(-mutation_rate..mutation_rate)
                    } else {
                        x
                    }
                })
            })
            .collect();

        let biases: Vec<Array1<f64>> = self
            .biases
            .iter()
            .map(|b| {
                b.mapv(|x| {
                    if rng.gen::<f64>() < mutation_rate {
                        x + rng.gen_range(-mutation_rate..mutation_rate)
                    } else {
                        x
                    }
                })
            })
            .collect();

        NeuralNetwork {
            weights,
            biases,
            learning_rate: self.learning_rate,
        }
    }
}

// Helper function for the sigmoid activation function
fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}
