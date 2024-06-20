
use std::collections::HashMap;

pub struct MarkovChains;

impl MarkovChains {
    pub fn new() -> Self {
        MarkovChains
    }

    pub fn analyze(&self, instructions: &[u8]) -> HashMap<u8, u8> {
        let mut analysis = HashMap::new();
        for window in instructions.windows(2) {
            if let [first, second] = window {
                *analysis.entry(*first).or_insert(0) += 1;
                *analysis.entry(*second).or_insert(0) += 1;
            }
        }
        analysis
    }
}
