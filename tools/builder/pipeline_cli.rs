use std::env;
use crate::pipelines::path1::build_path1;
use crate::pipelines::path2::build_path2;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: pipeline_cli <path1|path2>");
        return;
    }

    let pipeline = match args[1].as_str() {
        "path1" => build_path1(),
        "path2" => build_path2(),
        _ => {
            println!("Unknown pipeline");
            return;
        }
    };

    let data = b"test packet".to_vec();
    let result = pipeline.execute(data);

    println!("Pipeline executed. Output size: {}", result.len());
}
