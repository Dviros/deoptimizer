
#[cfg(test)]
mod tests {
    use super::*;
    use plugin::{PluginManager, TransformationPlugin};
    use visualization::VisualizationTool;
    use reporting::ReportingModule;

    struct TestTransformationPlugin;

    impl TransformationPlugin for TestTransformationPlugin {
        fn name(&self) -> &str {
            "Test Transformation"
        }

        fn transform(&self, instructions: &mut Vec<u8>) {
            for i in 0..instructions.len() {
                instructions[i] = instructions[i].wrapping_add(1);
            }
        }
    }

    #[test]
    fn test_transformation_plugin() {
        let mut instructions = vec![0x90; 10];
        let mut plugin_manager = PluginManager::new();
        plugin_manager.load_transformation_plugin(Box::new(TestTransformationPlugin));
        plugin_manager.apply_transformations(&mut instructions);
        for instruction in instructions {
            assert_eq!(instruction, 0x91);
        }
    }

    #[test]
    fn test_visualization_tool() {
        let instructions = vec![0x90, 0x91, 0x92, 0x93];
        let visualization_tool = VisualizationTool::new();
        assert!(visualization_tool.visualize(&instructions, "test_visualization.txt").is_ok());
    }

    #[test]
    fn test_reporting_module() {
        let transformations = vec!["Test Transformation".to_string()];
        let before = vec![0x90, 0x91, 0x92, 0x93];
        let after = vec![0x91, 0x92, 0x93, 0x94];
        let reporting_module = ReportingModule::new();
        assert!(reporting_module.generate_report(&transformations, &before, &after, "test_report.txt").is_ok());
    }

    #[test]
    fn test_arm_transforms() {
        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        ArmTransform::arithmetic_partitioning(&mut instructions);
        for instruction in &instructions {
            assert_eq!(*instruction, 0x01);
        }

        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        ArmTransform::logical_inverse(&mut instructions);
        for instruction in &instructions {
            assert_eq!(*instruction, !(*instruction));
        }

        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        ArmTransform::offset_mutation(&mut instructions);
        for instruction in &instructions {
            assert_eq!(*instruction, 0xFF);
        }

        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        ArmTransform::register_swap(&mut instructions);
        assert_eq!(instructions, vec![0x01, 0x00, 0x03, 0x02]);
    }

    #[test]
    fn test_riscv_transforms() {
        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        RiscVTransform::arithmetic_partitioning(&mut instructions);
        for instruction in &instructions {
            assert_eq!(*instruction, 0x01);
        }

        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        RiscVTransform::logical_inverse(&mut instructions);
        for instruction in &instructions {
            assert_eq!(*instruction, !(*instruction));
        }

        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        RiscVTransform::offset_mutation(&mut instructions);
        for instruction in &instructions {
            assert_eq!(*instruction, 0xFF);
        }

        let mut instructions = vec![0x00, 0x01, 0x02, 0x03];
        RiscVTransform::register_swap(&mut instructions);
        assert_eq!(instructions, vec![0x01, 0x00, 0x03, 0x02]);
    }
}
