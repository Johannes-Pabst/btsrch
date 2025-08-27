use crate::unit_calc_parser::{lexer::Unit, parser::{UnitCalculation, UnitConversion}, unit_number_parser::UnitNumber};

impl UnitConversion{
    pub fn execute(&self) -> Result<(UnitNumber, Option<Unit>, Option<(UnitNumber,String)>),String>{
        match self{
            Self::PrimitiveUnitConversion(c, u)=>{
                Ok((c.execute()?, Some(u.clone()),None))
            },
            Self::Calculation(c)=>{
                Ok((c.execute()?, None,None))
            },
            Self::ComplexUnitConversion(a, b)=>{
                Ok((a.execute()?,None,Some((b.execute()?,b.to_string()))))
            }
        }
    }
}
impl UnitCalculation{
    pub fn execute(&self)->Result<UnitNumber,String>{
        match self{
            Self::Plus(a, b)=>{
                a.execute()?+b.execute()?
            },
            Self::Minus(a, b)=>{
                a.execute()?-b.execute()?
            },
            Self::Mult(a, b)=>{
                Ok(a.execute()?*b.execute()?)
            },
            Self::ImplMult(a, b)=>{
                Ok(a.execute()?*b.execute()?)
            },
            Self::Div(a, b)=>{
                Ok(a.execute()?/b.execute()?)
            },
            Self::Pow(a, b)=>{
                Ok(a.execute()?.pow_i64(b.execute()?.to_i64()?))
            },
            Self::Bracket(a)=>{
                a.execute()
            },
            Self::Number(n)=>{
                let mut num = n.clone();
                num.clean();
                Ok(num)
            }
        }
    }
}