use super::super::{SExpr, Cons, Nil, Data, DataType, car, cdr};
use super::super::{Operator, Literal, Variable};
use super::super::{Environment, lookup};
use super::super::basictype::BasicType;
use super::super::basictype::{Boolean, Character, Number, String, Symbol, Pair, List, Procedure};
use super::super::basictype::{NumericType, Floating, Integer};
use super::super::operator::{OperatorType, Add, Mul};

pub fn eval(terms: &Vec<DataType>, mut environment: &Environment) -> Result<BasicType, StrBuf> {
    if terms.len() == 0 {
        return Err("No terms!".to_strbuf())
    } else if terms.len() == 1 {
        match terms.get(0) {
            &Operator(x)    => return Ok(Symbol(x.to_str().to_strbuf())),
            &Literal(ref x)     => return Ok(x.clone()),
            &Variable(ref x)    => match try!(lookup(x.as_slice(), environment)) {
                    Data(ty) => match ty {
                        Literal(value ) => return Ok(value),
                        _               => return Err("I can't handle this!".to_strbuf())
                    },
                    _   => return Err("I give up".to_strbuf())
            }
        }
    }

    let operator = match terms.get(0) {
        &Operator(x) => x,
        _   => return Err("idgi yet".to_strbuf())
    };

    let mut operands: Vec<BasicType> = Vec::new();
    for term in terms.tail().iter() {
        match *term {
            Operator(_) => return Err("Operating on an operator? Heresy!".to_strbuf()),
            Literal(ref x)  => operands.push(x.clone()),
            Variable(ref x) => {
                let var = try!(lookup(x.as_slice(), environment));
                let val = match var {
                    Data(x) => try!(eval(&vec!(x.clone()), environment)),
                    _       => return Err("I can't handle this!".to_strbuf()),
                };
                operands.push(val);
            }
        }
    }

    match operator {
        Add => add(&operands),
        Mul => mul(&operands),
        _   => Err("idgi yet".to_strbuf())
    }
}


pub fn add(terms: &Vec<BasicType>) -> Result<BasicType, StrBuf> {
    let mut number_vec: Vec<NumericType> = Vec::new();
    for term in terms.iter() {
        match term {
            &Number(ref ty) => number_vec.push(ty.clone()),
            _           => return Err("foo".to_strbuf())
        }
    }

    //floating point will supercede integers
    let mut float_flag = false;
    for term in number_vec.iter() {
        match term {
            &Floating(_)    => float_flag = true,
            &Integer(_)     => { }
            _   => return Err("qux".to_strbuf())
        }
    }

    if float_flag == true {
        let mut answer = 0.0;
        for term in number_vec.iter() {
            let augend = match *term {
                Floating(x) => x,
                Integer(x)  => x as f64,
                _   => fail!("Unexpected argument!")
            };
            answer += augend;
        }
        return Ok(Number(Floating(answer)))
    }

    let mut answer = 0;
    for term in number_vec.iter() {
        match *term {
            Integer(x)  => answer += x,
            _   => fail!("Unexpected argument!")
        }
    }

    Ok(Number(Integer(answer)))
}

pub fn mul(terms: &Vec<BasicType>) -> Result<BasicType, StrBuf> {
    let mut number_vec: Vec<NumericType> = Vec::new();
    for term in terms.iter() {
        match term {
            &Number(ref ty) => number_vec.push(ty.clone()),
            _           => return Err("foo".to_strbuf())
        }
    }

    //floating point will supercede integers
    let mut float_flag = false;
    for term in number_vec.iter() {
        match term {
            &Floating(_)    => float_flag = true,
            &Integer(_)     => { }
            _   => return Err("qux".to_strbuf())
        }
    }

    if float_flag == true {
        let mut answer = 1.0;
        for term in number_vec.iter() {
            let augend = match *term {
                Floating(x) => x,
                Integer(x)  => x as f64,
                _   => fail!("Unexpected argument!")
            };
            answer *= augend;
        }
        return Ok(Number(Floating(answer)))
    }

    let mut answer = 1;
    for term in number_vec.iter() {
        match *term {
            Integer(x)  => answer *= x,
            _   => fail!("Unexpected argument!")
        }
    }

    Ok(Number(Integer(answer)))
}
