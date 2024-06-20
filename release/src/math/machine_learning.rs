
use ndarray::Array1;

pub struct MachineLearning;

impl MachineLearning {
    pub fn new() -> Self {
        MachineLearning
    }

    pub fn predict(&self, instructions: &[u8]) -> Array1<f64> {
        let data = instructions.iter().map(|&x| x as f64).collect::<Vec<f64>>();
        Array1::from(data)
    }
}
