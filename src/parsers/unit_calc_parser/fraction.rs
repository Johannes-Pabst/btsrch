use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone)]
pub struct Fraction {
    top: i64,
    bottom: u64,
}
impl Add for Fraction {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        let bottom = lcm(self.bottom, rhs.bottom);
        self.top = self.top * bottom as i64 / self.bottom as i64
            + rhs.top * bottom as i64 / rhs.bottom as i64;
        self.reduce();
        self
    }
}
impl Neg for Fraction {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.top *= -1;
        self
    }
}
impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl Mul for Fraction {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.top *= rhs.top;
        self.bottom *= rhs.bottom;
        self.reduce();
        self
    }
}
impl Div for Fraction {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.top *= rhs.bottom as i64*rhs.top.signum();
        self.bottom *= rhs.top.abs() as u64;
        self.reduce();
        self
    }
}
impl Fraction {
    fn reduce(&mut self) {
        let gcd = lcm(self.bottom, self.top as u64);
        self.bottom /= gcd;
        self.top /= gcd as i64;
    }
}
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b)
    }
    a
}
pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
