pub fn mse(predicted: &[f32], target: &[f32]) -> f32 {
    assert_eq!(
        predicted.len(),
        target.len(),
    );

    let sum: f32 = predicted
        .iter()
        .zip(target.iter())
        .map(|(prediction, expected)| {
            let error = prediction - expected;
            error * error
        })
        .sum();

    sum / predicted.len() as f32
}

pub fn mse_derivative(
    predicted: &[f32],
    target: &[f32],
) -> Vec<f32> {

    assert_eq!(
        predicted.len(),
        target.len(),
    );

    let scale = 2.0 / predicted.len() as f32;

    predicted
        .iter()
        .zip(target.iter())
        .map(|(prediction, expected)| {
            scale * (prediction - expected)
        })
        .collect()
}

