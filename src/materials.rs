use crate::colors::*;
use crate::lights::Light;
use crate::tuples::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Material {
    pub color: Color,
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
    point_at: &Tuple,
    eye_vector: &Tuple,
    normal_vector: &Tuple,
) -> Color {
    let effective_color = hadamard_product(&material.color, &light.intensity);
    let ambient_contribution = multiply(&effective_color, material.ambient);

    let direction_to_light_source = &crate::tuples::sub(&light.position, point_at).normalize();

    let light_dot_normal = dot(&direction_to_light_source, normal_vector);
    let diffuse_contribution: Color;
    let specular_contribution: Color;

    if light_dot_normal < 0.0 {
        diffuse_contribution = Color::black();
        specular_contribution = Color::black();
    } else {
        diffuse_contribution = multiply(&multiply(&effective_color, material.diffuse), light_dot_normal);

        let reflect_vector = reflect(&direction_to_light_source.negate(), normal_vector);
        let reflect_dot_eye = dot(&reflect_vector, eye_vector);

        if reflect_dot_eye < 0.0 {
            specular_contribution = Color::black();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular_contribution = multiply(&multiply(&light.intensity, material.specular), factor);
        }
    }

    crate::colors::add(crate::colors::add(ambient_contribution, diffuse_contribution), specular_contribution)
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
        let calculated_color = lighting(material, light, &position, &eye_vector, &normal_vector);
        assert_abs_diff_eq!(
            calculated_color,
            Color {
                r: 1.9,
                g: 1.9,
                b: 1.9
            }
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_changing_angle() {
        let material = Material::new();
        let position = point(0.0, 0.0, 0.0);
        let eye_vector = vector(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = vector(0.0, 0.0, -1.0);
        let light = Light {
            position: point(0.0, 0.0, -10.0),
            intensity: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        };
        let calculated_color = lighting(material, light, &position, &eye_vector, &normal_vector);
        assert_abs_diff_eq!(
            calculated_color,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0
            }
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_changing_angle_of_light() {
        let material = Material::new();
        let position = point(0.0, 0.0, 0.0);
        let eye_vector = vector(0.0, 0.0, -1.0);
        let normal_vector = vector(0.0, 0.0, -1.0);
        let light = Light {
            position: point(0.0, 10.0, -10.0),
            intensity: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        };
        let calculated_color = lighting(material, light, &position, &eye_vector, &normal_vector);
        assert_abs_diff_eq!(
            calculated_color,
            Color {
                r: 0.7363961,
                g: 0.7363961,
                b: 0.7363961
            }
        );
    }

    #[test]
    fn lightning_with_eye_in_the_path_of_the_reflection_vector() {
        let material = Material::new();
        let position = point(0.0, 0.0, 0.0);
        let eye_vector = vector(0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = vector(0.0, 0.0, -1.0);
        let light = Light {
            position: point(0.0, 10.0, -10.0),
            intensity: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        };
        let calculated_color = lighting(material, light, &position, &eye_vector, &normal_vector);
        assert_abs_diff_eq!(
            calculated_color,
            Color {
                r: 1.6363853,
                g: 1.6363853,
                b: 1.6363853
            }
        );
    }

    #[test]
    fn light_behind_surface() {
        let material = Material::new();
        let position = point(0.0, 0.0, 0.0);
        let eye_vector = vector(0.0, 1.0, -1.0);
        let normal_vector = vector(0.0, 0.0, -1.0);
        let light = Light {
            position: point(0.0, 0.0, 10.0),
            intensity: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        };
        let calculated_color = lighting(material, light, &position, &eye_vector, &normal_vector);
        assert_abs_diff_eq!(
            calculated_color,
            Color {
                r: 0.1,
                g: 0.1,
                b: 0.1
            }
        );
    }
}
