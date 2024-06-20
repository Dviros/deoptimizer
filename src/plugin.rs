
pub trait TransformationPlugin {
    fn name(&self) -> &str;
    fn transform(&self, instructions: &mut Vec<u8>);
}

pub trait AnalysisPlugin {
    fn name(&self) -> &str;
    fn analyze(&self, binary: &[u8]) -> Result<(), String>;
}

pub struct PluginManager {
    transformations: Vec<Box<dyn TransformationPlugin>>,
    analyses: Vec<Box<dyn AnalysisPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            transformations: Vec::new(),
            analyses: Vec::new(),
        }
    }

    pub fn load_transformation_plugin(&mut self, plugin: Box<dyn TransformationPlugin>) {
        self.transformations.push(plugin);
    }

    pub fn load_analysis_plugin(&mut self, plugin: Box<dyn AnalysisPlugin>) {
        self.analyses.push(plugin);
    }

    pub fn apply_transformations(&self, instructions: &mut Vec<u8>) {
        for plugin in &self.transformations {
            plugin.transform(instructions);
        }
    }

    pub fn run_analyses(&self, binary: &[u8]) -> Result<(), String> {
        for plugin in &self.analyses {
            plugin.analyze(binary)?;
        }
        Ok(())
    }
}
