fn main() {
    let b1 = Body::new(1e10, 0.0);
    let b2 = Body::new(1e10, 1.0);

    let grav = Body::gravity(b1, b2);

    println!("Gravity: {}", grav);
}

struct Body {
    mass: f64,
    x: f64,
    velocity: f64,
    acceleration: f64,
}

const G: f64 = 6.67408e-11;

impl Body {
    fn new(mass: f64, x: f64) -> Body {
        Body {
            mass,
            x,
            velocity: 0.0,
            acceleration: 0.0,
        }
    }

    fn gravity(b1: Body, b2: Body) -> f64 {
        G * b1.mass * b2.mass / (b2.x - b1.x).powi(2)
    }
}
