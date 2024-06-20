
use plugin::TransformationPlugin;

struct IncrementTransformationPlugin;

impl TransformationPlugin for IncrementTransformationPlugin {
    fn name(&self) -> &str {
        "Increment Transformation"
    }

    fn transform(&self, instructions: &mut Vec<u8>) {
        for i in 0..instructions.len() {
            instructions[i] = instructions[i].wrapping_add(1);
        }
    }
}

struct DecrementTransformationPlugin;

impl TransformationPlugin for DecrementTransformationPlugin {
    fn name(&self) -> &str {
        "Decrement Transformation"
    }

    fn transform(&self, instructions: &mut Vec<u8>) {
        for i in 0..instructions.len() {
            instructions[i] = instructions[i].wrapping_sub(1);
        }
    }
}

struct XORTransformationPlugin;

impl TransformationPlugin for XORTransformationPlugin {
    fn name(&self) -> &str {
        "XOR Transformation"
    }

    fn transform(&self, instructions: &mut Vec<u8>) {
        for i in 0..instructions.len() {
            instructions[i] = instructions[i] ^ 0xAA;
        }
    }
}

pub fn load_example_plugins(manager: &mut plugin::PluginManager) {
    manager.load_transformation_plugin(Box::new(IncrementTransformationPlugin));
    manager.load_transformation_plugin(Box::new(DecrementTransformationPlugin));
    manager.load_transformation_plugin(Box::new(XORTransformationPlugin));
}
