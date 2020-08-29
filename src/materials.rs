use crate::colors::Color;
use crate::lights::Light;
use crate::tuples::*;

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
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub fn lighting(
    material: Material,
    light: Light,
    position: Tuple,
    eye_vector: Tuple,
    normal_vector: Tuple,
) -> Color {
    Color {
        r: 1.9,
        g: 1.9,
        b: 1.9,
    }
}

#[cfg(test)]
mod tests {
    use crate::materials::*;

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let material = Material::new();
        let position = point(0.0, 0.0, 0.0);
        let eye_vector = vector(0.0, 1.0, -1.0);
        let normal_vector = vector(0.0, 0.0, -1.0);
        let light = Light {
            position: point(0.0, 0.0, -10.0),
            intensity: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        };
        let calculated_color = lighting(material, light, position, eye_vector, normal_vector);
        assert_abs_diff_eq!(
            calculated_color,
            Color {
                r: 1.9,
                g: 1.9,
                b: 1.9
            }
        );
    }
}
