use std::ops::Add;

#[derive(Clone)]
pub struct Fraction{
    top:i64,
    bottom:u64,
}
impl Add for Fraction{
    type Output=Self;

    fn add(self, rhs: Self) -> Self::Output {
        let smallest=todo!();
        todo!()
    }
}
impl Fraction{
    fn reduce(&mut self){
        let gcd=gcd(self.bottom, self.top as u64);
        self.bottom/=gcd;
        self.top/=gcd as i64;
    }
}
pub fn gcd(mut a:u64,mut b:u64)->u64{
    while b>0{
        (a, b)=(b, a%b)
    }
    a
}