/// σ(x) = 1 / (1 + e^-x)
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// σ'(x) = y * (1 - y)
/// where y = σ(x)
pub fn sigmoid_derivative_from_output(output: f32) -> f32 {
    output * (1.0 - output)
}

