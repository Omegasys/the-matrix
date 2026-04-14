use std::fs;

pub struct ShaderAsset {
    pub vertex: String,
    pub fragment: String,
}

pub fn load_shader(vertex_path: &str, fragment_path: &str) -> Result<ShaderAsset, String> {
    let vertex = fs::read_to_string(vertex_path)
        .map_err(|e| e.to_string())?;

    let fragment = fs::read_to_string(fragment_path)
        .map_err(|e| e.to_string())?;

    Ok(ShaderAsset { vertex, fragment })
}
