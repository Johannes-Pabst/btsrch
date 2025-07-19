use std::vec;

use crate::unit_calc_parser::unit_number_parser::{MetricBaseUnit, UnitExp, UnitNumber};

#[derive(Clone)]
pub enum Token {
    Number(String),
    StringLiteral(String),
    Plus,
    Minus,
    Mult,
    Div,
    Power,
    LowerEq,
    Lower,
    HigherEq,
    Higher,
    Eq,
    NEq,
    OpenBracket,
    CloseBracket,
    In,
    Unit(UnitNumber),
}
impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(s) => format!("{s}").to_string(),
            Token::Unit(s) => s.to_string(),
            Token::StringLiteral(s) => format!("\"{s}\""),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Mult => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Power => "^".to_string(),
            Token::LowerEq => "<=".to_string(),
            Token::Lower => "<".to_string(),
            Token::HigherEq => ">=".to_string(),
            Token::Higher => ">".to_string(),
            Token::Eq => "==".to_string(),
            Token::NEq => "!=".to_string(),
            Token::OpenBracket => "(".to_string(),
            Token::CloseBracket => ")".to_string(),
            Token::In => "in".to_string(),
        }
    }
}
pub fn lex(input: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let operator_chars = "<>!=/*-+^";
    let number_chars = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ°._";
    let mut cur_kind = 0;
    let mut first_id = 0;
    /*
    0: nothing,
    1: operator,
    2: number,
    3: string literal,
    */
    for (i, c) in format!("{input} ").chars().enumerate() {
        match cur_kind {
            0 => {}
            1 => {
                if !operator_chars.contains(c) {
                    cur_kind = 0;
                    let word = input[first_id..i].to_string();
                    match word.as_str() {
                        "+" => {
                            tokens.push(Token::Plus);
                        }
                        "-" => {
                            tokens.push(Token::Minus);
                        }
                        "*" => {
                            tokens.push(Token::Mult);
                        }
                        "/" => {
                            tokens.push(Token::Div);
                        }
                        "^" => {
                            tokens.push(Token::Power);
                        }
                        "<=" => {
                            tokens.push(Token::LowerEq);
                        }
                        "<" => {
                            tokens.push(Token::Lower);
                        }
                        ">=" => {
                            tokens.push(Token::HigherEq);
                        }
                        ">" => {
                            tokens.push(Token::Higher);
                        }
                        "==" => {
                            tokens.push(Token::Eq);
                        }
                        "!=" => {
                            tokens.push(Token::NEq);
                        }
                        "(" => {
                            tokens.push(Token::OpenBracket);
                        }
                        ")" => {
                            tokens.push(Token::CloseBracket);
                        }
                        _ => {
                            return Err(format!("unrecognized token: {word}"));
                        }
                    }
                }
            }
            2 => {
                if !number_chars.contains(c) {
                    cur_kind = 0;
                    let _word = input[first_id..i].to_string();
                }
            }
            3 => {
                if c == '"' {
                    let word = input[first_id + 1..i].to_string();
                    tokens.push(Token::StringLiteral(word));
                    cur_kind = 0;
                    continue;
                }
            }
            _ => todo!(),
        }
        if cur_kind == 0 {
            if operator_chars.contains(c) {
                cur_kind = 1;
            } else if number_chars.contains(c) {
                cur_kind = 2;
            } else if c == '"' {
                cur_kind = 3;
            }
            first_id = i;
        }
    }
    Ok(tokens)
}
pub fn lex2(input: String, units: &Vec<Unit>) -> Option<Vec<Token>> {
    let mut start_id = 0;
    let mut output = Vec::new();
    while start_id < input.len() {
        let mut end_id = input.len();
        let mut sucess = false;
        while end_id > start_id {
            let token = get_token(input[start_id..end_id].to_string(), units);
            if let Some(t) = token {
                output.extend(t);
                start_id = end_id;
                sucess = true;
                break;
            }
            end_id -= 1;
        }
        if !sucess {
            return None;
        }
    }
    Some(output)
}
pub fn get_token(s: String, units: &Vec<Unit>) -> Option<Vec<Token>> {
    let atomic = vec![
        ("+", vec![Token::Plus]),
        ("-", vec![Token::Minus]),
        ("*", vec![Token::Mult]),
        ("/", vec![Token::Div]),
        ("^", vec![Token::Power]),
        ("<=", vec![Token::LowerEq]),
        ("<", vec![Token::Lower]),
        (">=", vec![Token::HigherEq]),
        (">", vec![Token::Higher]),
        ("==", vec![Token::Eq]),
        ("!=", vec![Token::NEq]),
        ("(", vec![Token::OpenBracket]),
        (")", vec![Token::CloseBracket]),
    ];
    if let Some(t) = atomic.iter().find(|a| a.0 == s) {
        return Some(t.1.iter().cloned().collect());
    }
    if s.chars().all(|c| c.is_numeric()) {
        return Some(vec![Token::Number(s)]);
    }
    if s.chars().all(|c| c.is_whitespace()) {
        return Some(vec![]);
    }
    if s.starts_with('"') && s.ends_with('"') {
        let content = s[1..s.len() - 1].to_string();
        if !content.contains('"') {
            return Some(vec![Token::StringLiteral(content)]);
        }
    }
    for u in units {
        for n in u.valid_names.iter() {
            if *n == s {
                return Some(vec![Token::Unit(u.si.clone())]);
            }
        }
    }
    None
}
pub fn get_units() -> Vec<Unit> {
    let mut v = vec![];
    v.extend(
        Unit {
            name: "meter".to_string(),
            plural: "meters".to_string(),
            abbreviation: "m".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );
    v.extend(
        Unit {
            name: "gram".to_string(),
            plural: "grams".to_string(),
            abbreviation: "g".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0, // because base SI unit is kilogram
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Gramm,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "second".to_string(),
            plural: "seconds".to_string(),
            abbreviation: "s".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "ampere".to_string(),
            plural: "amperes".to_string(),
            abbreviation: "A".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Ampere,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "kelvin".to_string(),
            plural: "kelvins".to_string(),
            abbreviation: "K".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Kelvin,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "mole".to_string(),
            plural: "moles".to_string(),
            abbreviation: "mol".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Mole,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "candela".to_string(),
            plural: "candelas".to_string(),
            abbreviation: "cd".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Candela,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "liter".to_string(),
            plural: "liters".to_string(),
            abbreviation: "L".to_string(),
            valid_names: vec!["l".to_string()], // lowercase alias
            si: UnitNumber {
                num: 0.001,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 3,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "byte".to_string(),
            plural: "bytes".to_string(),
            abbreviation: "B".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Byte,
                    exp: 1,
                }],
            },
            priority: 1.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "bit".to_string(),
            plural: "bits".to_string(),
            abbreviation: "b".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.125,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Byte,
                    exp: 1,
                }],
            },
            priority: 0.9,
        }
        .create()
        .add_si_prefixes(),
    );

    v
}
#[derive(Clone)]
pub struct Unit {
    pub name: String,
    pub plural: String,
    pub abbreviation: String,
    pub valid_names: Vec<String>,
    pub si: UnitNumber,
    pub priority: f32,
}
impl Unit {
    pub fn create(mut self) -> Self {
        self.valid_names.push(self.abbreviation.clone());
        self.valid_names.push(self.name.clone());
        self.valid_names.push(self.plural.clone());
        self
    }
    pub fn add_si_prefixes(self) -> Vec<Self> {
        vec![
            Unit {
                name: format!("yocto{}", &self.name),
                plural: format!("yocto{}", &self.plural),
                abbreviation: format!("y{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("y{}", n),
                        _ => format!("yocto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-24,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("zepto{}", &self.name),
                plural: format!("zepto{}", &self.plural),
                abbreviation: format!("z{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("z{}", n),
                        _ => format!("zepto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-21,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("atto{}", &self.name),
                plural: format!("atto{}", &self.plural),
                abbreviation: format!("a{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("a{}", n),
                        _ => format!("atto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-18,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("fempto{}", &self.name),
                plural: format!("fempto{}", &self.plural),
                abbreviation: format!("f{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("f{}", n),
                        _ => format!("fempto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-15,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("pico{}", &self.name),
                plural: format!("pico{}", &self.plural),
                abbreviation: format!("p{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("p{}", n),
                        _ => format!("pico{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-12,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("nano{}", &self.name),
                plural: format!("nano{}", &self.plural),
                abbreviation: format!("n{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("n{}", n),
                        _ => format!("nano{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-9,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("micro{}", &self.name),
                plural: format!("micro{}", &self.plural),
                abbreviation: format!("µ{}", &self.abbreviation),
                valid_names: vec![format!("mu{}", &self.abbreviation)],
                si: UnitNumber {
                    num: self.si.num * 1e-6,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("milli{}", &self.name),
                plural: format!("milli{}", &self.plural),
                abbreviation: format!("m{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("m{}", n),
                        _ => format!("milli{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-3,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("{}", &self.name),
                plural: format!("{}", &self.plural),
                abbreviation: format!("{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("{}", n),
                        _ => format!("{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("kilo{}", &self.name),
                plural: format!("kilo{}", &self.plural),
                abbreviation: format!("k{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("k{}", n),
                        _ => format!("kilo{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e3,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("mega{}", &self.name),
                plural: format!("mega{}", &self.plural),
                abbreviation: format!("M{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("M{}", n),
                        _ => format!("mega{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e6,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("giga{}", &self.name),
                plural: format!("giga{}", &self.plural),
                abbreviation: format!("G{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("G{}", n),
                        _ => format!("giga{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e9,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("tera{}", &self.name),
                plural: format!("tera{}", &self.plural),
                abbreviation: format!("T{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("T{}", n),
                        _ => format!("tera{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e12,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("peta{}", &self.name),
                plural: format!("peta{}", &self.plural),
                abbreviation: format!("P{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("P{}", n),
                        _ => format!("peta{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e15,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("exa{}", &self.name),
                plural: format!("exa{}", &self.plural),
                abbreviation: format!("E{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("E{}", n),
                        _ => format!("exa{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e18,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("zetta{}", &self.name),
                plural: format!("zetta{}", &self.plural),
                abbreviation: format!("Z{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("Z{}", n),
                        _ => format!("zetta{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e21,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("yotta{}", &self.name),
                plural: format!("yotta{}", &self.plural),
                abbreviation: format!("Y{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..2 => format!("Y{}", n),
                        _ => format!("yotta{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e24,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
        ]
    }
}
