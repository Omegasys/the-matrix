use crate::pipelines::builder::{Pipeline, PipelineStage};

struct DummyStage(&'static str);

impl PipelineStage for DummyStage {
    fn process(&self, data: Vec<u8>) -> Vec<u8> {
        println!("[PIPELINE] Passing through {}", self.0);
        data
    }
}

pub fn build_path1() -> Pipeline {
    let mut pipeline = Pipeline::new();

    pipeline.add_stage(DummyStage("Shadowsocks"));
    pipeline.add_stage(DummyStage("Firewall"));
    pipeline.add_stage(DummyStage("VPN"));
    pipeline.add_stage(DummyStage("Soft Layer"));
    pipeline.add_stage(DummyStage("Tor"));
    pipeline.add_stage(DummyStage("OPC"));
    pipeline.add_stage(DummyStage("Freenet"));
    pipeline.add_stage(DummyStage("QUIC"));
    pipeline.add_stage(DummyStage("Lokinet"));
    pipeline.add_stage(DummyStage("V2Ray"));
    pipeline.add_stage(DummyStage("Zeronet"));
    pipeline.add_stage(DummyStage("Meek"));
    pipeline.add_stage(DummyStage("Firewall"));
    pipeline.add_stage(DummyStage("VPN"));
    pipeline.add_stage(DummyStage("DNS/DSCP"));

    pipeline
}
