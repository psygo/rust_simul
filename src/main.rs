fn main() {
    let mut b1 = Body::new(1e5, 0.0);
    let mut b2 = Body::new(1e5, 1.0);

    let mut counter = 1;

    loop {
        (b1, b2) = Body::gravity(b1, b2);

        print!("----------------------");
        print!("\n");

        print!("B1 X: {}\n", b1.x);
        print!("B2 X: {}\n", b2.x);
        print!("\n");

        b1 = apply_forces(b1);
        b2 = apply_forces(b2);

        print!("B1 X: {}\n", b1.x);
        print!("B2 X: {}\n", b2.x);
        print!("\n");
        
        print!("----------------------");

        counter = counter + 1;
        if counter > 10 {
            break;
        }
    }
}

// Delay of 1s.
fn apply_forces(b: Body) -> Body {
    let new_a = b.f / b.m;
    let new_v = b.v + new_a;
    let new_x = b.x + new_v;

    Body {
        m: b.m,
        f: b.f,
        v: new_v,
        x: new_x,
    }
}

struct Body {
    m: f64,
    f: f64,
    v: f64,
    x: f64,
}

const G: f64 = 6.67408e-11;

impl Body {
    fn new(m: f64, x: f64) -> Body {
        Body {
            m,
            x,
            f: 0.0,
            v: 0.0,
        }
    }

    fn gravity(b1: Body, b2: Body) -> (Body, Body) {
        let fg = G * b1.m * b2.m / (b2.x - b1.x).powi(2);
        let signed_fg = if b2.x - b1.x > 0.0 { fg } else { -fg };

        (
            Body {
                m: b1.m,
                x: b1.x,
                f: b1.f + signed_fg,
                v: b1.v,
            },
            Body {
                m: b2.m,
                x: b2.x,
                f: b2.f - signed_fg,
                v: b2.v,
            },
        )
    }
}
