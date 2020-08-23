#[derive(PartialEq, Clone, Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 0.0,
    }
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 1.0,
    }
}

fn add(a: Tuple, b: Tuple) -> Tuple {
    Tuple {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w,
    }
}

fn sub(a: Tuple, b: Tuple) -> Tuple {
    Tuple {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w,
    }
}

fn negate(a: Tuple) -> Tuple {
    Tuple {
        x: -a.x,
        y: -a.y,
        z: -a.z,
        w: -a.w,
    }
}

fn mul(a: Tuple, b: f32) -> Tuple {
    Tuple {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
        w: a.w * b,
    }
}

fn div(a: Tuple, b: f32) -> Tuple {
    Tuple {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
        w: a.w / b,
    }
}

fn magnitude(a: Tuple) -> f32 {
    let component_squares = a.x.powf(2.0) + a.y.powf(2.0) + a.z.powf(2.0);
    component_squares.sqrt()
}

fn normalize(a: Tuple) -> Tuple {
    div(a.clone(), magnitude(a.clone()))
}

fn dot(a: Tuple, b: Tuple) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

fn cross(a: Tuple, b: Tuple) -> Tuple {
    vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

#[cfg(test)]
mod tests {
    use crate::tuples::*;

    #[test]
    fn a_vector_has_w_0() {
        assert_eq!(vector(1.0, 1.1, 1.2).w, 0.0)
    }

    #[test]
    fn a_point_has_w_1() {
        assert_eq!(point(1.0, 1.1, 1.2).w, 1.0)
    }

    #[test]
    fn adding_tuples() {
        assert_eq!(
            add(
                Tuple {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    w: 1.0
                },
                Tuple {
                    x: 4.0,
                    y: 5.0,
                    z: 0.0,
                    w: 3.0
                },
            ),
            Tuple {
                x: 5.0,
                y: 7.0,
                z: 3.0,
                w: 4.0
            }
        )
    }

    #[test]
    fn substracting_tuples() {
        assert_eq!(
            sub(
                Tuple {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    w: 1.0
                },
                Tuple {
                    x: 4.0,
                    y: 5.0,
                    z: 0.0,
                    w: 3.0
                },
            ),
            Tuple {
                x: -3.0,
                y: -3.0,
                z: 3.0,
                w: -2.0
            }
        )
    }

    #[test]
    fn negating_tuples() {
        assert_eq!(
            negate(Tuple {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 1.0
            }),
            Tuple {
                x: -1.0,
                y: -2.0,
                z: -3.0,
                w: -1.0
            }
        )
    }

    #[test]
    fn multiplying_tuples_by_scalars() {
        assert_eq!(
            mul(
                Tuple {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    w: 1.0
                },
                2.0
            ),
            Tuple {
                x: 2.0,
                y: 4.0,
                z: 6.0,
                w: 2.0
            }
        )
    }

    #[test]
    fn dividing_tuples_by_scalars() {
        assert_eq!(
            div(
                Tuple {
                    x: 2.0,
                    y: 4.0,
                    z: 6.0,
                    w: 2.0
                },
                2.0
            ),
            Tuple {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 1.0
            }
        )
    }

    #[test]
    fn magnitude_of_a_unit_vector() {
        abs_diff_eq!(magnitude(vector(1.0, 0.0, 0.0)), 1.0);
    }

    #[test]
    fn magnitude_of_a_negative_vector() {
        assert_eq!(magnitude(vector(-1.0, -2.0, -3.0)), 14.0_f32.sqrt());
    }

    #[test]
    fn magnitude_of_a_normalized_vector_is_1() {
        abs_diff_eq!(magnitude(normalize(vector(-1.0, -2.0, -3.0))), 1.0);
    }

    #[test]
    fn dot_product_of_two_tuples() {
        abs_diff_eq!(dot(vector(1.0, 2.0, 3.0), vector(2.0, 3.0, 4.0)), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let cross_vector = cross(vector(1.0, 2.0, 3.0), vector(2.0, 3.0, 4.0));
        abs_diff_eq!(cross_vector.x, -1.0);
        abs_diff_eq!(cross_vector.y, 2.0);
        abs_diff_eq!(cross_vector.z, -1.0);
    }
}
