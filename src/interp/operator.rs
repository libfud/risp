//! Operators operating operationally on operands optionally

#[deriving(Show)]
#[deriving(Clone)]
#[deriving(Eq)]
pub enum OperatorType {
    Constructor,
    Car,
    Cdr,
    Print,
    If,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Even,
    Lt,
    LtEq,
    Eq,
    GtEq,
    Gt,
    Null
}

pub fn from_str(s: &str) -> Option<OperatorType> {
    match s {
        "cons"  => Some(Constructor),
        "car"   => Some(Car),
        "cdr"   => Some(Cdr),
        "print" => Some(Print),
        "+"     => Some(Add),
        "-"     => Some(Sub),
        "*"     => Some(Mul),
        "/"     => Some(Div),
        "<"     => Some(Lt),
        "<="    => Some(LtEq),
        "="     => Some(Eq),
        ">="    => Some(GtEq),
        ">"     => Some(Gt),
        "if"    => Some(If),
        _       => None
    }
}
