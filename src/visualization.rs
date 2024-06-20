
use std::fs::File;
use std::io::{self, Write};

pub struct VisualizationTool;

impl VisualizationTool {
    pub fn new() -> Self {
        VisualizationTool
    }

    pub fn visualize(&self, instructions: &[u8], filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "Detailed Transformation Visualization")?;
        writeln!(file, "=================================")?;
        for (i, instruction) in instructions.iter().enumerate() {
            writeln!(file, "Instruction {}: {:#04X} (Binary: {:08b})", i, instruction, instruction)?;
        }
        Ok(())
    }
}
