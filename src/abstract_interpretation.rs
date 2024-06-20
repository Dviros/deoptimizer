
pub struct AbstractInterpretation;

impl AbstractInterpretation {
    pub fn new() -> Self {
        AbstractInterpretation
    }

    pub fn analyze(&self, instructions: &[u8]) -> Vec<String> {
        let mut analysis = Vec::new();
        for instruction in instructions {
            analysis.push(format!("{:x}", instruction));
        }
        analysis
    }
}
