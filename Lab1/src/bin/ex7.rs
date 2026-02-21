use std::fmt;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
struct Complex {
    x: f32,
    y: f32,
}

impl Complex {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn abs_val(&self) -> f32 {
        self.x.hypot(self.y)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

fn main() {
    let x = Complex::new(1.0, 2.0);
    let y = Complex::new(4.0, 0.0);

    println!("|x| = {}", x.abs_val());

    let z = x * y;
    println!("x * y = {}", z);
}
