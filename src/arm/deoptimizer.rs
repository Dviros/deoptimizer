
use log::{error, info};
use std::collections::HashMap;

mod analysis;
mod transforms;
mod encoding;
use analysis::ArmAnalyzer;
use transforms::ArmTransform;
use encoding::ArmEncoder;

#[derive(Debug)]
pub enum ArmDeoptimizerError {
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

pub struct ArmDeoptimizer {
    pub transforms: AvailableTransforms,
    pub analyzer: ArmAnalyzer,
}

impl ArmDeoptimizer {
    pub fn new() -> Self {
        ArmDeoptimizer {
            transforms: AvailableTransforms::All,
            analyzer: ArmAnalyzer::new(),
        }
    }

    pub fn set_transform_gadgets(&mut self, gadgets: AvailableTransforms) {
        self.transforms = gadgets;
    }

    pub fn analyze(&mut self, binary: &[u8]) -> Result<(), ArmDeoptimizerError> {
        // Use ARM analyzer to analyze the binary
        info!("Analyzing ARM binary...");
        self.analyzer.analyze(binary).map_err(|e| {
            error!("{}", e);
            ArmDeoptimizerError::InvalidInstruction
        })
    }

    pub fn transform(&self, instructions: &mut Vec<u8>) -> Result<(), ArmDeoptimizerError> {
        // Apply transformations based on available gadgets
        info!("Transforming ARM instructions...");
        if self.transforms.contains(AvailableTransforms::ArithmeticPartitioning) {
            ArmTransform::arithmetic_partitioning(instructions);
        }
        if self.transforms.contains(AvailableTransforms::LogicalInverse) {
            ArmTransform::logical_inverse(instructions);
        }
        if self.transforms.contains(AvailableTransforms::LogicalPartitioning) {
            ArmTransform::logical_partitioning(instructions);
        }
        if self.transforms.contains(AvailableTransforms::OffsetMutation) {
            ArmTransform::offset_mutation(instructions);
        }
        if self.transforms.contains(AvailableTransforms::RegisterSwap) {
            ArmTransform::register_swap(instructions);
        }
        Ok(())
    }

    pub fn encode(&self, instructions: &Vec<u8>) -> Result<Vec<u8>, ArmDeoptimizerError> {
        // Use ARM encoder to encode transformed instructions back to binary
        info!("Encoding ARM instructions...");
        ArmEncoder::encode(instructions).map_err(|e| {
            error!("{}", e);
            ArmDeoptimizerError::EncodingFail
        })
    }
}
