#[derive(Clone)]
pub enum LightType {
    Directional,
    Point,
}

#[derive(Clone)]
pub struct Light {
    pub light_type: LightType,
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub intensity: f32,
}

impl Light {
    pub fn directional(direction: [f32; 3], intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional,
            position: [0.0, 0.0, 0.0],
            direction,
            intensity,
        }
    }

    pub fn point(position: [f32; 3], intensity: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            direction: [0.0, 0.0, 0.0],
            intensity,
        }
    }
}
