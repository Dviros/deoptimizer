
pub struct ConvexOptimization;

impl ConvexOptimization {
    pub fn new() -> Self {
        ConvexOptimization
    }

    pub fn optimize(&self, instructions: &mut Vec<u8>) {
        for instruction in instructions {
            *instruction = instruction.wrapping_add(1);
        }
    }
}
