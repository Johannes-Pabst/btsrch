use crate::parsers::unit_calc_parser::{
    lexer::Unit,
    parser::{UnitCalculation, UnitConversion},
    unit_number_parser::UnitNumber,
};

impl UnitConversion {
    pub fn execute(&self) -> Result<(Value, Option<Unit>, Option<(UnitNumber, String)>), String> {
        match self {
            Self::PrimitiveUnitConversion(c, u) => Ok((c.execute()?, Some(u.clone()), None)),
            Self::Calculation(c) => Ok((c.execute()?, None, None)),
            Self::ComplexUnitConversion(a, b) => match b.execute()? {
                Value::UnitNumber(b2) => Ok((a.execute()?, None, Some((b2, b.to_string())))),
                Value::Touple(_) => Err("a touple cannot be given as a target unit!".to_string()),
            },
        }
    }
}
#[derive(Clone)]
pub enum Value {
    UnitNumber(UnitNumber),
    Touple(Vec<Value>),
}
impl Value {
    pub fn log(&self, un: &UnitNumber) -> Option<i64> {
        let f = self.flatten();
        f.iter()
            .map(|a| a.log(un))
            .reduce(|a, b| if a == b { a } else { None }).flatten()
    }
    pub fn flatten(&self) -> Vec<UnitNumber> {
        match self {
            Self::Touple(v) => v.iter().map(|v| v.flatten()).flatten().collect(),
            Self::UnitNumber(u) => vec![u.clone()],
        }
    }
    pub fn map<F>(&self, f:&F)->Result<Self,String> where F: Fn(UnitNumber)->Result<UnitNumber,String>{
        match self {
            Self::Touple(v)=>{
                let mut v2=Vec::new();
                for n in v{
                    v2.push(n.map(f)?);
                }
                Ok(Self::Touple(v2))
            }
            Self::UnitNumber(n)=>{
                Ok(Self::UnitNumber(f(n.clone())?))
            }
        }
    }
}
impl ToString for Value{
    fn to_string(&self) -> String {
        match self {
            Self::Touple(v)=>{
                format!("({})", v.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
            }
            Self::UnitNumber(un)=>{
                un.to_string()
            }
        }
    }
}
impl UnitCalculation {
    pub fn execute(&self) -> Result<Value, String> {
        match self {
            Self::Plus(a, b) => match (a.execute()?, b.execute()?) {
                (Value::UnitNumber(a), Value::UnitNumber(b)) => Ok(Value::UnitNumber((a + b)?)),
                _ => Err("cannot calculate using touples yet!".to_string()),
            },
            Self::Minus(a, b) => match (a.execute()?, b.execute()?) {
                (Value::UnitNumber(a), Value::UnitNumber(b)) => Ok(Value::UnitNumber((a - b)?)),
                _ => Err("cannot calculate using touples yet!".to_string()),
            },
            Self::Mult(a, b) => match (a.execute()?, b.execute()?) {
                (Value::UnitNumber(a), Value::UnitNumber(b)) => Ok(Value::UnitNumber(a * b)),
                _ => Err("cannot calculate using touples yet!".to_string()),
            },
            Self::ImplMult(a, b) => match (a.execute()?, b.execute()?) {
                (Value::UnitNumber(a), Value::UnitNumber(b)) => Ok(Value::UnitNumber(a * b)),
                _ => Err("cannot calculate using touples yet!".to_string()),
            },
            Self::Div(a, b) => match (a.execute()?, b.execute()?) {
                (Value::UnitNumber(a), Value::UnitNumber(b)) => Ok(Value::UnitNumber(a / b)),
                _ => Err("cannot calculate using touples yet!".to_string()),
            },
            Self::Pow(a, b) => match (a.execute()?, b.execute()?) {
                (Value::UnitNumber(a), Value::UnitNumber(b)) => {
                    Ok(Value::UnitNumber(a.pow_i64(b.to_i64()?)))
                }
                _ => Err("cannot calculate using touples yet!".to_string()),
            },
            Self::Bracket(a) => a.execute(),
            Self::Number(n) => {
                let mut num = n.clone();
                num.clean();
                Ok(Value::UnitNumber(num))
            }
            Self::Function(s, a) => {
                let v = a.execute()?;
                match s.as_str() {
                    "sqrt" => Ok(v.map(&|n| n.pow_one_over_i(2))?),
                    _ => Err(format!("unknown function: {}", s)),
                }
            }
            Self::Touple(v) => Ok(Value::Touple(
                v.iter()
                    .map(|a| a.execute())
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Self::Value(v)=>{
                Ok(v.clone())
            }
        }
    }
}
pub fn get_function_names() -> Vec<String> {
    vec!["sqrt".to_string()]
}