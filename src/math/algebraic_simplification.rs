
pub struct AlgebraicSimplification;

impl AlgebraicSimplification {
    pub fn new() -> Self {
        AlgebraicSimplification
    }

    pub fn simplify(&self, instructions: &mut Vec<u8>) {
        for instruction in instructions {
            *instruction = instruction.wrapping_add(1);
        }
    }
}
