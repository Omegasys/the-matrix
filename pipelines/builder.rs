pub trait PipelineStage: Send + Sync {
    fn process(&self, data: Vec<u8>) -> Vec<u8>;
}

pub struct Pipeline {
    stages: Vec<Box<dyn PipelineStage>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn add_stage<T: PipelineStage + 'static>(&mut self, stage: T) {
        self.stages.push(Box::new(stage));
    }

    pub fn execute(&self, mut data: Vec<u8>) -> Vec<u8> {
        for stage in &self.stages {
            data = stage.process(data);
        }
        data
    }
}
