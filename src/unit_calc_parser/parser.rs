use crate::unit_calc_parser::{
    lexer::{Token, Unit},
    unit_number_parser::{superscript, UnitNumber},
};
pub enum UnitCalculation {
    Plus(Box<UnitCalculation>, Box<UnitCalculation>),
    Minus(Box<UnitCalculation>, Box<UnitCalculation>),
    Mult(Box<UnitCalculation>, Box<UnitCalculation>),
    ImplMult(Box<UnitCalculation>, Box<UnitCalculation>),
    Div(Box<UnitCalculation>, Box<UnitCalculation>),
    Pow(Box<UnitCalculation>, Box<UnitCalculation>),
    Bracket(Box<UnitCalculation>),
    Number(UnitNumber),
}
pub enum UnitConversion {
    ComplexUnitConversion(UnitCalculation, UnitCalculation),
    PrimitiveUnitConversion(UnitCalculation, Unit),
    Calculation(UnitCalculation),
}
impl ToString for UnitConversion{
    fn to_string(&self) -> String {
        match self {
            Self::Calculation(c)=>{format!("{}",c.to_string())}
            Self::PrimitiveUnitConversion(c,u)=>{format!("{} as {}",c.to_string(),u.plural)}
            Self::ComplexUnitConversion(c,u)=>{format!("{} as {}",c.to_string(),u.to_string())}
        }
    }
}
impl ToString for UnitCalculation{
    fn to_string(&self) -> String {
        match self {
            Self::Plus(a, b)=>{format!("{} + {}", a.to_string(), b.to_string())}
            Self::Minus(a, b) => { format!("{} - {}", a.to_string(), b.to_string()) }
            Self::Mult(a, b) => { format!("{} * {}", a.to_string(), b.to_string()) }
            Self::ImplMult(a, b) => { format!("{}{}", a.to_string(), b.to_string()) }
            Self::Div(a, b) => { format!("{}/{}", a.to_string(), b.to_string()) }
            Self::Pow(a, b) => {
                if let Self::Number(b)=b.as_ref(){
                    return format!("{}{}", a.to_string(), superscript(b.to_string()))
                }
                format!("{}^{}", a.to_string(), b.to_string())
            }
            Self::Bracket(a) => { format!("({})", a.to_string()) }
            Self::Number(n) => { n.to_string() }
        }
    }
}
pub fn parse_unit_conversion(tokens: Vec<Token>) -> Result<UnitConversion, String> {
    match split_at(tokens, vec![Token::Convert]) {
        SplitAtOut::Split(eq1, _, eq2) => {
            if let Some(Token::Unit(_, Some(u))) = eq2.first() {
                if eq2.len()==1{
                    return Ok(UnitConversion::PrimitiveUnitConversion(
                        parse_unit_add_sub(eq1)?,
                        u.clone(),
                    ))
                }
            }
            Ok(UnitConversion::ComplexUnitConversion(parse_unit_add_sub(eq1)?, parse_unit_add_sub(eq2)?))
        }
        SplitAtOut::NoSplit(tokens) => Ok(UnitConversion::Calculation(parse_unit_add_sub(tokens)?)),
    }
}
pub fn parse_unit_add_sub(tokens: Vec<Token>) -> Result<UnitCalculation, String> {
    match split_at(tokens, vec![Token::Plus, Token::Minus]) {
        SplitAtOut::Split(eq1, t, eq2) => match t {
            Token::Plus => Ok(UnitCalculation::Plus(
                Box::new(parse_unit_add_sub(eq1)?),
                Box::new(parse_unit_add_sub(eq2)?),
            )),
            Token::Minus => Ok(UnitCalculation::Minus(
                Box::new(parse_unit_add_sub(eq1)?),
                Box::new(parse_unit_add_sub(eq2)?),
            )),
            _ => todo!(),
        },
        SplitAtOut::NoSplit(tokens) => parse_unit_mult_div_1(tokens),
    }
}
pub fn parse_unit_mult_div_1(tokens: Vec<Token>) -> Result<UnitCalculation, String> {
    /*
        _*_
        _/_
        )(
        1(
        )1
        1 1

        1m
        m1
        g m
        m(
        )m
    */
    let mut open_brackets = 0;
    for t in tokens.iter() {
        match t {
            Token::OpenBracket => {
                open_brackets += 1;
            }
            Token::CloseBracket => {
                open_brackets -= 1;
            }
            _ => {}
        };
    }
    let mut iterator = tokens.iter().enumerate().rev().peekable();
    while let Some((i, t)) = iterator.next() {
        match t {
            Token::OpenBracket => {
                open_brackets += 1;
            }
            Token::CloseBracket => {
                open_brackets -= 1;
            }
            _ => {}
        };
        if open_brackets == 0 {
            match t {
                Token::Mult => {
                    return Ok(UnitCalculation::Mult(
                        Box::new(parse_unit_mult_div_1(tokens[..i].to_vec())?),
                        Box::new(parse_unit_mult_div_1(tokens[i + 1..].to_vec())?),
                    ));
                }
                Token::Div => {
                    return Ok(UnitCalculation::Div(
                        Box::new(parse_unit_mult_div_1(tokens[..i].to_vec())?),
                        Box::new(parse_unit_mult_div_1(tokens[i + 1..].to_vec())?),
                    ));
                }
                Token::Number(_) | Token::OpenBracket => {
                    if let Some((_, p)) = iterator.peek() {
                        match p {
                            Token::Number(_) | Token::CloseBracket => {
                                return Ok(UnitCalculation::ImplMult(
                                    Box::new(parse_unit_mult_div_1(tokens[..i].to_vec())?),
                                    Box::new(parse_unit_mult_div_1(tokens[i..].to_vec())?),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {},
            };
        }
    }
    parse_unit_mult_div_2(tokens)
}
pub fn parse_unit_mult_div_2(tokens: Vec<Token>) -> Result<UnitCalculation, String> {
    let mut open_brackets = 0;
    for t in tokens.iter() {
        match t {
            Token::OpenBracket => {
                open_brackets += 1;
            }
            Token::CloseBracket => {
                open_brackets -= 1;
            }
            _ => {}
        };
    }
    let mut iterator = tokens.iter().enumerate().rev().peekable();
    while let Some((i, t)) = iterator.next() {
        match t {
            Token::OpenBracket => {
                open_brackets += 1;
            }
            Token::CloseBracket => {
                open_brackets -= 1;
            }
            _ => {}
        };
        if open_brackets == 0 {
            match t {
                Token::Unit(_, _) => {
                    if let Some((_, p)) = iterator.peek() {
                        match p {
                            Token::Unit(_, _) | Token::Number(_) | Token::CloseBracket => {
                                return Ok(UnitCalculation::ImplMult(
                                    Box::new(parse_unit_mult_div_2(tokens[..i].to_vec())?),
                                    Box::new(parse_unit_mult_div_2(tokens[i..].to_vec())?),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                Token::Number(_) | Token::OpenBracket => {
                    if let Some((_, p)) = iterator.peek() {
                        match p {
                            Token::Unit(_, _) => {
                                return Ok(UnitCalculation::ImplMult(
                                    Box::new(parse_unit_mult_div_2(tokens[..i].to_vec())?),
                                    Box::new(parse_unit_mult_div_2(tokens[i..].to_vec())?),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {},
            };
        }
    }
    parse_unit_pow(tokens)
}
pub fn parse_unit_pow(tokens: Vec<Token>) -> Result<UnitCalculation, String> {
    match split_at(tokens, vec![Token::Power]) {
        SplitAtOut::Split(eq1, t, eq2) => match t {
            Token::Power => Ok(UnitCalculation::Pow(
                Box::new(parse_unit_pow(eq1)?),
                Box::new(parse_unit_pow(eq2)?),
            )),
            _ => todo!(),
        },
        SplitAtOut::NoSplit(tokens) => parse_unit_bracket(tokens),
    }
}
pub fn parse_unit_bracket(tokens: Vec<Token>) -> Result<UnitCalculation, String> {
    if Some(&Token::OpenBracket) == tokens.first() && Some(&Token::CloseBracket) == tokens.last() {
        Ok(UnitCalculation::Bracket(Box::new(parse_unit_add_sub(tokens[1..tokens.len() - 1].to_vec())?)))
    } else {
        parse_unit_number(tokens)
    }
}
pub fn parse_unit_number(mut tokens: Vec<Token>) -> Result<UnitCalculation, String> {
    match tokens.len() {
        1 => match tokens.drain(..).next().unwrap() {
            Token::Number(n) => n.parse::<f64>().map_err(|e| format!("{:?}", e)).map(|n| {
                UnitCalculation::Number(UnitNumber {
                    num: n,
                    units: Vec::new(),
                })
            }),
            Token::Unit(n, _) => Ok(UnitCalculation::Number(n)),
            t => Err(format!("wrong number token: {}", t.to_string())),
        },
        2 => {
            let mut drain = tokens.drain(..);
            match (
                drain.next().unwrap(),
                drain.next().unwrap(),
            ) {
                (Token::Dot, Token::Number(n)) => {
                    format!(".{}",n).parse::<f64>().map_err(|e| format!("{:?}", e)).map(|n| {
                        UnitCalculation::Number(UnitNumber {
                            num: n,
                            units: Vec::new(),
                        })
                    })
                }
                _ => Err(format!("wrong number format!")),
            }
        }
        3 => {
            let mut drain = tokens.drain(..);
            match (
                drain.next().unwrap(),
                drain.next().unwrap(),
                drain.next().unwrap(),
            ) {
                (Token::Number(n1),Token::Dot, Token::Number(n2)) => {
                    format!("{}.{}",n1,n2).parse::<f64>().map_err(|e| format!("{:?}", e)).map(|n| {
                        UnitCalculation::Number(UnitNumber {
                            num: n,
                            units: Vec::new(),
                        })
                    })
                }
                _ => Err(format!("wrong number format!")),
            }
        }
        _ => Err(format!("wrong number len: {}", tokens.len())),
    }
}
pub enum SplitAtOut {
    Split(Vec<Token>, Token, Vec<Token>),
    NoSplit(Vec<Token>),
}
pub fn split_at(tokens: Vec<Token>, split_at_any: Vec<Token>) -> SplitAtOut {
    let mut open_brackets = 0;
    for t in tokens.iter() {
        match t {
            Token::OpenBracket => {
                open_brackets += 1;
            }
            Token::CloseBracket => {
                open_brackets -= 1;
            }
            _ => {}
        };
    }
    for (i, t) in tokens.iter().enumerate().rev() {
        match t {
            Token::OpenBracket => {
                open_brackets -= 1;
            }
            Token::CloseBracket => {
                open_brackets += 1;
            }
            t => {
                if open_brackets == 0 && split_at_any.contains(t) {
                    return SplitAtOut::Split(
                        tokens[..i].to_vec(),
                        t.clone(),
                        tokens[i + 1..].to_vec(),
                    );
                }
            }
        };
    }
    SplitAtOut::NoSplit(tokens)
}
