
use log::{error, info};
use std::collections::HashMap;

mod analysis;
mod transforms;
mod encoding;
use analysis::RiscVAnalyzer;
use transforms::RiscVTransform;
use encoding::RiscVEncoder;

#[derive(Debug)]
pub enum RiscVDeoptimizerError {
    InvalidInstruction,
    EncodingFail,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AvailableTransforms: u8 {
        const None = 0;
        const ArithmeticPartitioning = 1;
        const LogicalInverse = 1 << 1;
        const LogicalPartitioning = 1 << 2;
        const OffsetMutation = 1 << 3;
        const RegisterSwap = 1 << 4;
        const All = u8::MAX;
    }
}

pub struct RiscVDeoptimizer {
    pub transforms: AvailableTransforms,
    pub analyzer: RiscVAnalyzer,
}

impl RiscVDeoptimizer {
    pub fn new() -> Self {
        RiscVDeoptimizer {
            transforms: AvailableTransforms::All,
            analyzer: RiscVAnalyzer::new(),
        }
    }

    pub fn set_transform_gadgets(&mut self, gadgets: AvailableTransforms) {
        self.transforms = gadgets;
    }

    pub fn analyze(&mut self, binary: &[u8]) -> Result<(), RiscVDeoptimizerError> {
        // Use RISC-V analyzer to analyze the binary
        info!("Analyzing RISC-V binary...");
        self.analyzer.analyze(binary).map_err(|e| {
            error!("{}", e);
            RiscVDeoptimizerError::InvalidInstruction
        })
    }

    pub fn transform(&self, instructions: &mut Vec<u8>) -> Result<(), RiscVDeoptimizerError> {
        // Apply transformations based on available gadgets
        info!("Transforming RISC-V instructions...");
        if self.transforms.contains(AvailableTransforms::ArithmeticPartitioning) {
            RiscVTransform::arithmetic_partitioning(instructions);
        }
        if self.transforms.contains(AvailableTransforms::LogicalInverse) {
            RiscVTransform::logical_inverse(instructions);
        }
        if self.transforms.contains(AvailableTransforms::LogicalPartitioning) {
            RiscVTransform::logical_partitioning(instructions);
        }
        if self.transforms.contains(AvailableTransforms::OffsetMutation) {
            RiscVTransform::offset_mutation(instructions);
        }
        if self.transforms.contains(AvailableTransforms::RegisterSwap) {
            RiscVTransform::register_swap(instructions);
        }
        Ok(())
    }

    pub fn encode(&self, instructions: &Vec<u8>) -> Result<Vec<u8>, RiscVDeoptimizerError> {
        // Use RISC-V encoder to encode transformed instructions back to binary
        info!("Encoding RISC-V instructions...");
        RiscVEncoder::encode(instructions).map_err(|e| {
            error!("{}", e);
            RiscVDeoptimizerError::EncodingFail
        })
    }
}
