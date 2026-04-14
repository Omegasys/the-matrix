use crate::pipelines::builder::Pipeline;
use crate::pipelines::path2::build_path2;

#[test]
fn test_pipeline_execution() {
    let pipeline = build_path2();
    let input = b"test data".to_vec();

    let output = pipeline.execute(input.clone());

    assert_eq!(output, input);
}
