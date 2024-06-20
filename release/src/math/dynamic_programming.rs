
pub struct DynamicProgramming;

impl DynamicProgramming {
    pub fn new() -> Self {
        DynamicProgramming
    }

    pub fn optimize(&self, instructions: &mut Vec<u8>) {
        for instruction in instructions {
            *instruction = instruction.wrapping_add(1);
        }
    }
}
