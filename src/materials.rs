use crate::colors::Color;

#[derive(PartialEq, Clone, Debug)]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {}
