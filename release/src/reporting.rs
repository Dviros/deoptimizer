
use std::fs::File;
use std::io::{self, Write};

pub struct ReportingModule;

impl ReportingModule {
    pub fn new() -> Self {
        ReportingModule
    }

    pub fn generate_report(
        &self,
        transformations: &[String],
        before: &[u8],
        after: &[u8],
        filename: &str,
    ) -> io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "Transformation Report")?;
        writeln!(file, "====================")?;
        for transformation in transformations {
            writeln!(file, "Applied Transformation: {}", transformation)?;
        }
        writeln!(file, "
Before Transformation:")?;
        for (i, instruction) in before.iter().enumerate() {
            writeln!(file, "Instruction {}: {:#04X} (Binary: {:08b})", i, instruction, instruction)?;
        }
        writeln!(file, "
After Transformation:")?;
        for (i, instruction) in after.iter().enumerate() {
            writeln!(file, "Instruction {}: {:#04X} (Binary: {:08b})", i, instruction, instruction)?;
        }
        Ok(())
    }
}
