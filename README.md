# XOR Neural Network in Rust

A simple feedforward neural network implemented from scratch in Rust without using any machine learning libraries.

This project demonstrates the core concepts of neural networks, including:

- Forward propagation
- Backpropagation
- Gradient descent
- Sigmoid activation
- Mean Squared Error (MSE) loss
- Random weight initialization
- Deterministic dataset generation
- Smoke testing

---

## Project Structure

```
src/
├── activation.rs      # Sigmoid activation function
├── config.rs          # Training configuration
├── dataset.rs         # XOR dataset generator
├── layer.rs           # Dense neural network layer
├── loss.rs            # Mean Squared Error loss
├── network.rs         # Neural network implementation
├── random.rs          # Custom pseudo-random number generator
├── smoke_test.rs      # End-to-end learning test
└── main.rs            # Program entry point
```

---

## Network Architecture

```
Input (2)
      │
      ▼
Hidden Layer (8 neurons)
      │
      ▼
Output (1)
```

The network is trained to learn the XOR function.

---

## XOR Truth Table

| Input A | Input B | Output |
|---------:|---------:|-------:|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

---

## Features

- Written entirely in Rust
- No external machine learning libraries
- Custom deterministic random number generator
- Fully connected dense layers
- Forward propagation
- Backpropagation
- Sigmoid activation
- Mean Squared Error loss
- Configurable training parameters
- Smoke test for end-to-end verification

---

## Building

```bash
cargo build
```

---

## Running

```bash
cargo run
```

Example output:

```
Training...

[Epoch 1/100] Loss: ...
...

Training Complete

Accuracy: 100.00%

Sample Predictions:
Input: [0, 0] -> 0.001
Input: [0, 1] -> 0.998
Input: [1, 0] -> 0.998
Input: [1, 1] -> 0.002
```

---

## Running Tests

Run all tests:

```bash
cargo test
```

Run the smoke test with training output:

```bash
cargo test -- --nocapture
```

---

## Learning Process

The network learns by repeating the following steps:

1. Generate an XOR sample.
2. Perform a forward pass.
3. Compute the Mean Squared Error.
4. Compute the loss gradient.
5. Perform backpropagation.
6. Update weights and biases using gradient descent.
7. Repeat for every epoch until the loss converges.

---

## Technologies

- Rust
- Cargo

---

## Author

Betsinat Amare

Developed as a neural network implementation project in Rust.