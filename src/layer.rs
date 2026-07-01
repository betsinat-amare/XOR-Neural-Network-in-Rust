use crate::activation::{sigmoid, sigmoid_derivative_from_output};
use crate::random::Random;

#[derive(Debug, Clone)]
pub struct Layer {
    pub weights: Vec<Vec<f32>>,
    pub biases: Vec<f32>,
    input_cache: Vec<f32>,
    output_cache: Vec<f32>,
    input_size: usize,
    output_size: usize,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize, seed: u64) -> Self {
        let mut rng = Random::new(seed);

        let mut weights = vec![vec![0.0; input_size]; output_size];

        for neuron in 0..output_size {
            for input in 0..input_size {
                weights[neuron][input] = rng.next_weight();
            }
        }

        Self {
            weights,
            biases: vec![0.0; output_size],
            input_cache: vec![0.0; input_size],
            output_cache: vec![0.0; output_size],
            input_size,
            output_size,
        }
    }
    pub fn forward(&mut self, input: &[f32]) -> Vec<f32> {
        assert_eq!(
            input.len(),
            self.input_size,
        );

        self.input_cache = input.to_vec();

        let mut output = vec![0.0; self.output_size];

        for neuron in 0..self.output_size {
            let mut weighted_sum = self.biases[neuron];

            for input_index in 0..self.input_size {
                weighted_sum +=
                    self.weights[neuron][input_index] * input[input_index];
            }

            output[neuron] = sigmoid(weighted_sum);
        }

        self.output_cache = output.clone();

        output
    }
    pub fn backward(
        &mut self,
        output_error: &[f32],
        learning_rate: f32,
    ) -> Vec<f32> {
        assert_eq!(
            output_error.len(),
            self.output_size,
            "Output error length does not match layer output size."
        );

        let mut input_error = vec![0.0; self.input_size];

        for neuron in 0..self.output_size {
            let delta = output_error[neuron]
                * sigmoid_derivative_from_output(self.output_cache[neuron]);

            // Propagate error backward before updating weights.
            for input_index in 0..self.input_size {
                input_error[input_index] +=
                    self.weights[neuron][input_index] * delta;
            }

            // Gradient descent weight update.
            for input_index in 0..self.input_size {
                self.weights[neuron][input_index] -=
                    learning_rate
                    * delta
                    * self.input_cache[input_index];
            }

            // Gradient descent bias update.
            self.biases[neuron] -= learning_rate * delta;
        }

        input_error
    }

}