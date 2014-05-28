//! Operators operating operationally on operands optionally

#[deriving(Show, Clone, Eq)]
pub enum OperatorType {
    Constructor,
    Car,
    Cdr,
    List,
    Print,
    Lambda,
    Set,
    Quote,
    If,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    IsEven,
    Lt,
    LtEq,
    Eq,
    NEq,
    GtEq,
    Gt,
    And,
    Or,
    Null
}

pub fn from_str(s: &str) -> Option<OperatorType> {
    match s {
        "cons"  => Some(Constructor),
        "car"   => Some(Car),
        "cdr"   => Some(Cdr),
        "list"  => Some(List),
        "print" => Some(Print),
        "lambda"=> Some(Lambda),
        "set"   => Some(Set),
        "even?" => Some(IsEven),
        "+"     => Some(Add),
        "-"     => Some(Sub),
        "*"     => Some(Mul),
        "/"     => Some(Div),
        "<"     => Some(Lt),
        "<="    => Some(LtEq),
        "="     => Some(Eq),
        "!="    => Some(NEq),
        ">="    => Some(GtEq),
        ">"     => Some(Gt),
        "if"    => Some(If),
        "and"   => Some(And),
        "or"    => Some(Or),
        "quote" => Some(Quote),
        _       => None
    }
}
