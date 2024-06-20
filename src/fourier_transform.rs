
use rustfft::{FftPlanner, num_complex::Complex};

pub struct FourierTransform;

impl FourierTransform {
    pub fn new() -> Self {
        FourierTransform
    }

    pub fn analyze(&self, instructions: &[u8]) -> Vec<Complex<f64>> {
        let mut planner = FftPlanner::<f64>::new();
        let fft = planner.plan_fft_forward(instructions.len());
        let mut buffer: Vec<Complex<f64>> = instructions.iter().map(|&x| Complex::new(x as f64, 0.0)).collect();
        fft.process(&mut buffer);
        buffer
    }
}
