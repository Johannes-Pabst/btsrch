#[derive(Clone)]
pub struct UnitNumber{
    pub num:f64,
    pub units:Vec<UnitExp>,
}
impl ToString for UnitNumber{
    fn to_string(&self) -> String {
        format!("{} {}", self.num, self.units.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(""))
    }
}
#[derive(Clone)]
pub struct UnitExp{
    pub exp:i64,
    pub unit:MetricBaseUnit,
}
impl ToString for UnitExp{
    fn to_string(&self) -> String {
        format!("{}^{}", self.unit.to_string(),self.exp)
    }
}
#[derive(Clone)]
pub enum MetricBaseUnit{
    Meter,
    Gramm,
    Second,
    Ampere,
    Volt,
    Kelvin,
    Mole,
    Candela,
    Byte,
}
impl ToString for MetricBaseUnit{
    fn to_string(&self) -> String {
        match self {
            MetricBaseUnit::Meter=>"m",
            MetricBaseUnit::Gramm=>"g",
            MetricBaseUnit::Second=>"s",
            MetricBaseUnit::Ampere=>"A",
            MetricBaseUnit::Volt=>"V",
            MetricBaseUnit::Kelvin=>"°K",
            MetricBaseUnit::Mole=>"mol",
            MetricBaseUnit::Candela=>"cd",
            MetricBaseUnit::Byte=>"B",
        }.to_string()
    }
}
pub fn parse_unit_number(input:String)->Result<UnitNumber,String>{
    let all_digits="0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut last_num_id=0;
    for (i,c) in input.chars().enumerate(){
        if "0123456789".contains(c){
            last_num_id=i;
        }
    }
    let mut num:String=input[0..last_num_id+1].to_string();
    let units:String=input[last_num_id+1..].to_string();
    let mut base:u64=match &num[0..2]{
        "0x"=>16,
        "0b"=>2,
        "0o"=>8,
        _=>10,
    };
    if base!=10{
        num=num[2..].to_string();
    }
    if num.contains("z")&&base==10{
        let id = num.find("z").unwrap();
        let base_str=num[0..id].to_string();
        num=num[id+1..].to_string();
        base=base_str.parse().map_err(|e| format!("{:?}",e))?;
    }
    let mut exp: i64=0;
    if num.contains("e"){
        let id = num.find("e").unwrap();
        let exp_str=num[id+1..].to_string();
        num=num[0..id].to_string();
        exp=exp_str.parse().map_err(|e| format!("{:?}",e))?;
    }
    let dot_id=num.find(".").or(Some(num.len())).unwrap() as i64+exp;
    let mut i=0;
    let digits=all_digits[0..(base as usize)].to_string();
    let mut number:f64=0.0;
    for c in num.chars(){
        if digits.contains(c){
            number+=(digits.find(c).unwrap()*base.pow((dot_id-i) as u32) as usize) as f64;
            i+=1;
        }
    }

    Err("TODO!".to_string())
}