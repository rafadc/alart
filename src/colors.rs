use approx::AbsDiffEq;

#[derive(PartialEq, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl AbsDiffEq for Color {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs_diff_eq(&self.r, &other.r, epsilon)
            && f32::abs_diff_eq(&self.g, &other.g, epsilon)
            && f32::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

pub fn add(a: Color, b: Color) -> Color {
    Color {
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b,
    }
}

pub fn sub(a: Color, b: Color) -> Color {
    Color {
        r: a.r - b.r,
        g: a.g - b.g,
        b: a.b - b.b,
    }
}

pub fn multiply(a: &Color, b: f32) -> Color {
    Color {
        r: a.r * b,
        g: a.g * b,
        b: a.b * b,
    }
}

pub fn hadamard_product(a: &Color, b: &Color) -> Color {
    Color {
        r: a.r * b.r,
        g: a.g * b.g,
        b: a.b * b.b,
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::*;

    #[test]
    fn adding_colors() {
        let addition = add(
            Color {
                r: 0.9,
                g: 0.6,
                b: 0.75,
            },
            Color {
                r: 0.7,
                g: 0.1,
                b: 0.25,
            },
        );
        abs_diff_eq!(addition.r, 1.6);
        abs_diff_eq!(addition.g, 0.7);
        abs_diff_eq!(addition.b, 1.0);
    }

    #[test]
    fn substracting_colors() {
        let substraction = sub(
            Color {
                r: 0.9,
                g: 0.6,
                b: 0.75,
            },
            Color {
                r: 0.7,
                g: 0.1,
                b: 0.25,
            },
        );
        abs_diff_eq!(substraction.r, 0.2);
        abs_diff_eq!(substraction.g, 0.5);
        abs_diff_eq!(substraction.b, 0.5);
    }

    #[test]
    fn multiply_a_color_by_a_scalar() {
        let multiplication = multiply(
            &Color {
                r: 0.9,
                g: 0.6,
                b: 0.75,
            },
            2.0,
        );
        abs_diff_eq!(multiplication.r, 1.8);
        abs_diff_eq!(multiplication.g, 1.2);
        abs_diff_eq!(multiplication.b, 1.5);
    }

    #[test]
    fn hadamard_product_of_two_colors() {
        let product = hadamard_product(
            &Color {
                r: 1.0,
                g: 0.6,
                b: 0.75,
            },
            &Color {
                r: 0.7,
                g: 0.1,
                b: 0.25,
            },
        );
        abs_diff_eq!(product.r, 0.7);
        abs_diff_eq!(product.g, 0.06);
        abs_diff_eq!(product.b, 0.1875);
    }
}
