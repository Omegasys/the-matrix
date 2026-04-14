use std::fs;

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<[f32; 3]>,
}

pub fn load_obj(path: &str) -> Result<Model, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    let mut vertices = Vec::new();

    for line in contents.lines() {
        if line.starts_with("v ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let z: f32 = parts[3].parse().unwrap_or(0.0);

                vertices.push([x, y, z]);
            }
        }
    }

    Ok(Model { vertices })
}
