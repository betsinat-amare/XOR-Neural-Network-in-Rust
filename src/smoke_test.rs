use crate::dataset::XorDatasetGenerator;
use crate::network::NeuralNetwork;

#[test]
fn smoke_test_xor_learning() {
    const SEED: u64 = 42;
    const DATASET_SIZE: usize = 10_000;

    let mut generator = XorDatasetGenerator::new(SEED);
    let dataset = generator.generate(DATASET_SIZE);

    let mut network = NeuralNetwork::new(
        2,
        8,
        SEED,
    );

    network.train(
        &dataset,
        100,
        0.1,
    );

    let accuracy = network.evaluate(&dataset);

    assert!(
        accuracy >= 0.99,
        "Expected accuracy >= 99%, got {:.2}%",
        accuracy * 100.0
    );
}