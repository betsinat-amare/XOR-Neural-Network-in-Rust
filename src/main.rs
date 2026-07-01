mod activation;
mod config;
mod dataset;
mod layer;
mod loss;
mod network;
mod random;

#[cfg(test)]
mod smoke_test;

use config::Config;
use dataset::XorDatasetGenerator;
use network::NeuralNetwork;

fn main() {
    let config = Config::default();

    println!("========================================");
    println!(" XOR Neural Network (Rust)");
    println!("========================================");

    println!("Configuration:");
    println!("  Input Size    : {}", config.input_size);
    println!("  Hidden Size   : {}", config.hidden_size);
    println!("  Dataset Size  : {}", config.dataset_size);
    println!("  Epochs        : {}", config.epochs);
    println!("  Learning Rate : {}", config.learning_rate);
    println!("  Seed          : {}", config.seed);

    println!("\nGenerating dataset...");

    let mut generator = XorDatasetGenerator::new(config.seed);
    let dataset = generator.generate(config.dataset_size);

    println!("Generated {} samples.", dataset.len());

    println!("\nBuilding neural network...");

    let mut network = NeuralNetwork::new(
        config.input_size,
        config.hidden_size,
        config.seed,
    );

    println!("\nTraining...\n");

    network.train(
        &dataset,
        config.epochs,
        config.learning_rate,
    );

    println!("\nEvaluating...\n");

    let accuracy = network.evaluate(&dataset);

    println!("========================================");
    println!("Training Complete");
    println!("========================================");
    println!("Accuracy: {:.2}%", accuracy * 100.0);

    println!("\nSample Predictions:");

    let samples = [
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    for (input, expected) in samples {
        let prediction = network.predict(&input);

        println!(
            "Input: {:?} -> Prediction: {:.4} | Expected: {:.1}",
            input,
            prediction[0],
            expected,
        );
    }

    println!("========================================");
}