use crate::dataset::Sample;
use crate::layer::Layer;
use crate::loss::{mse, mse_derivative};

pub struct NeuralNetwork {
    hidden: Layer,
    output: Layer,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, seed: u64) -> Self {
        Self {
            hidden: Layer::new(input_size, hidden_size, seed),
            output: Layer::new(hidden_size, 1, seed + 1),
        }
    }

    pub fn forward(&mut self, input: &[f32]) -> Vec<f32> {
        let hidden_output = self.hidden.forward(input);
        self.output.forward(&hidden_output)
    }

    pub fn predict(&mut self, input: &[f32]) -> Vec<f32> {
        self.forward(input)
    }

    pub fn train(
        &mut self,
        dataset: &[Sample],
        epochs: usize,
        learning_rate: f32,
    ) {
        assert!(
            !dataset.is_empty(),
            "Training dataset cannot be empty."
        );

        for epoch in 0..epochs {
            let mut total_loss = 0.0;

            for sample in dataset {
                let prediction = self.forward(&sample.input);

                total_loss += mse(&prediction, &sample.target);

                let output_error =
                    mse_derivative(&prediction, &sample.target);

                let hidden_error =
                    self.output.backward(&output_error, learning_rate);

                self.hidden.backward(&hidden_error, learning_rate);
            }

            let average_loss = total_loss / dataset.len() as f32;

            println!(
                "[Epoch {}/{}] Loss: {:.6}",
                epoch + 1,
                epochs,
                average_loss
            );
        }
    }
    pub fn evaluate(&mut self, dataset: &[Sample]) -> f32 {
        assert!(
            !dataset.is_empty(),
            "Evaluation dataset cannot be empty."
        );

        let mut correct = 0;

        for sample in dataset {
            let prediction = self.forward(&sample.input);

            let predicted_label = if prediction[0] >= 0.5 {
                1.0
            } else {
                0.0
            };

            if (predicted_label - sample.target[0]).abs() < f32::EPSILON {
                correct += 1;
            }
        }

        correct as f32 / dataset.len() as f32
    }
}