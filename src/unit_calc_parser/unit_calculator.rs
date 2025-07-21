use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::unit_calc_parser::unit_number_parser::{UnitExp, UnitNumber};

impl UnitNumber {
    pub fn clean(&mut self) {
        let mut nu: Vec<UnitExp> = Vec::new();
        for u in self.units.drain(..) {
            if let Some(u2) = nu.iter_mut().find(|a| a.unit == u.unit) {
                u2.exp += u.exp;
            } else {
                nu.push(u);
            }
        }
        self.units = nu;
        self.units.retain(|a| a.exp != 0);
        self.units.sort_by(|a, b| a.unit.cmp(&b.unit));
    }
    pub fn cleaned(&self) -> Self {
        let mut s = self.clone();
        s.clean();
        s
    }
    pub fn pow_i64(mut self, exp: i64) -> Self {
        self.units = self
            .units
            .iter()
            .map(|x| UnitExp {
                exp: x.exp * exp,
                unit: x.unit.clone(),
            })
            .collect::<Vec<UnitExp>>();
        self.num = self.num.powi(exp as i32);
        self
    }
    pub fn to_i64(&self) -> Result<i64, String> {
        if self.units.len() == 0 && (self.num.round() - self.num).abs() < 1e-10 {
            Ok(self.num.round() as i64)
        } else {
            Err("only integer exponents without numbers are allowed!".to_string())
        }
    }
    pub fn addable(&self, other: Self) -> bool {
        self.units.len() == other.units.len()
            && self.units.iter().zip(other.units).all(|(a, b)| *a == b)
    }
    pub fn log(&self, smaller: &Self) -> Option<i64> {
        if self.units.len() == smaller.units.len() {
            if self.units.len() > 0 {
                let log=self.units[0].exp/smaller.units[0].exp;
                if self.units.iter().zip(smaller.units.clone()).all(|(a,b)| a.unit==b.unit&&b.exp*log==a.exp){
                    return Some(log);
                }
            }
        }
        None
    }
}
impl Add for UnitNumber {
    type Output = Result<Self, String>;
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.units.len() == rhs.units.len()
            && self.units.iter().zip(rhs.units).all(|(a, b)| *a == b)
        {
            self.num += rhs.num;
            Ok(self)
        } else {
            Err("non-matching units cannot be added!".to_string())
        }
    }
}
impl Neg for UnitNumber {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.num = -self.num;
        self
    }
}
impl Sub for UnitNumber {
    type Output = Result<Self, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl Mul for UnitNumber {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.units.extend(rhs.units);
        self.num *= rhs.num;
        self.clean();
        self
    }
}
impl Div for UnitNumber {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.pow_i64(-1)
    }
}
