
pub struct LinearProgramming;

impl LinearProgramming {
    pub fn new() -> Self {
        LinearProgramming
    }

    pub fn optimize(&self, instructions: &mut Vec<u8>) {
        for instruction in instructions {
            *instruction = instruction.wrapping_add(1);
        }
    }
}
