use image::GenericImageView;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn load_texture(path: &str) -> Result<Texture, String> {
    let img = image::open(path)
        .map_err(|e| e.to_string())?;

    let rgba = img.to_rgba8();

    Ok(Texture {
        width: img.width(),
        height: img.height(),
        data: rgba.into_raw(),
    })
}
