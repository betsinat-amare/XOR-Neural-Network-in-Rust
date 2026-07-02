# XOR Neural Network in Rust

A simple feedforward neural network implemented entirely from scratch in Rust.

This project demonstrates the complete implementation of a neural network without using any machine learning libraries. It includes forward propagation, backpropagation, gradient descent, dataset generation, evaluation, and automated testing.

---

# Project Objective

The objective of this project is to build a neural network capable of learning the XOR function.

The XOR problem is a classic machine learning problem because it is **not linearly separable**, meaning a single-layer perceptron cannot solve it. A hidden layer is required, making it an excellent demonstration of how neural networks learn nonlinear relationships.

---

# Features

- Pure Rust implementation
- No external machine learning libraries
- Two-layer feedforward neural network
- Sigmoid activation function
- Mean Squared Error (MSE) loss
- Backpropagation using gradient descent
- Deterministic random number generator
- Automatic XOR dataset generation
- Model evaluation using accuracy
- Smoke test verifying successful learning

---

# Project Structure

```
src/
│
├── activation.rs     # Sigmoid activation function
├── config.rs         # Training configuration
├── dataset.rs        # XOR dataset generator
├── layer.rs          # Dense layer implementation
├── loss.rs           # Mean Squared Error loss
├── network.rs        # Neural network implementation
├── random.rs         # Custom deterministic RNG
├── smoke_test.rs     # End-to-end learning test
└── main.rs           # Application entry point
```

---

# Neural Network Architecture

```
        Input Layer
      x1          x2
         \      /
          \    /
       Hidden Layer
      (8 Sigmoid Neurons)
            |
            |
      Output Layer
      (1 Sigmoid Neuron)
```

Architecture:

- Input Layer: 2 neurons
- Hidden Layer: 8 neurons
- Output Layer: 1 neuron

Activation Function:

- Sigmoid

Loss Function:

- Mean Squared Error (MSE)

Optimizer:

- Gradient Descent

---

# Training Process

Training follows the standard supervised learning pipeline:

1. Generate XOR samples.
2. Perform forward propagation.
3. Compute prediction error using MSE.
4. Compute gradients using backpropagation.
5. Update weights and biases using gradient descent.
6. Repeat for multiple epochs.

---

# Dataset

The dataset is generated programmatically.

Each sample consists of:

```
Input:  [x1, x2]
Target: [x1 XOR x2]
```

Example:

| Input | Target |
|--------|--------|
| 0 0 | 0 |
| 0 1 | 1 |
| 1 0 | 1 |
| 1 1 | 0 |

For training, a large dataset of **50,000 samples** is generated using a deterministic random number generator.

---

# Configuration

Default configuration:

| Parameter | Value |
|------------|------:|
| Input Size | 2 |
| Hidden Size | 8 |
| Dataset Size | 50,000 |
| Epochs | 100 |
| Learning Rate | 0.1 |
| Seed | 42 |

---

# Example Output

```
Training...

[Epoch 1/100] Loss: 0.023112
...
[Epoch 100/100] Loss: 0.000003

Training Complete

Accuracy: 100.00%

Sample Predictions:

Input: [0.0, 0.0] -> Prediction: 0.0013
Input: [0.0, 1.0] -> Prediction: 0.9985
Input: [1.0, 0.0] -> Prediction: 0.9984
Input: [1.0, 1.0] -> Prediction: 0.0020
```

---

# Smoke Test

A smoke test is included to verify that the entire neural network pipeline functions correctly.

The test:

- Generates an XOR dataset
- Trains the neural network
- Evaluates the trained model
- Asserts at least **99% accuracy**

Run the tests with:

```bash
cargo test
```

Expected output:

```
running 1 test
test smoke_test::smoke_test_xor_learning ... ok

test result: ok
```

---

# Running the Project

Clone the repository:

```bash
git clone <repository-url>
```

Navigate to the project:

```bash
cd xor_nn
```

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

Run tests:

```bash
cargo test
```

---

# Learning Concepts Demonstrated

This project demonstrates the implementation of several fundamental machine learning concepts:

- Feedforward Neural Networks
- Dense (Fully Connected) Layers
- Forward Propagation
- Sigmoid Activation Function
- Mean Squared Error (MSE)
- Backpropagation
- Gradient Descent
- Weight Initialization
- Bias Updates
- Dataset Generation
- Model Evaluation
- Deterministic Random Number Generation

---

# Design Decisions

Several design choices were made to improve readability and maintainability:

- Each responsibility is separated into its own module.
- Layers manage their own forward and backward propagation.
- Input and output values are cached for backpropagation.
- Assertions validate input dimensions.
- Deterministic random initialization ensures reproducible training.
- Comprehensive inline documentation explains major components.


---

# Future Improvements

Possible extensions include:

- Additional activation functions (ReLU, Tanh)
- Cross-Entropy loss
- Mini-batch gradient descent
- Multiple hidden layers
- Model serialization
- Generic matrix implementation
- Parallelized training
- Configurable optimizers (Momentum, Adam)

---


Developed as a Rust machine learning project demonstrating a complete implementation of a feedforward neural network trained on the XOR problem without external ML frameworks.
