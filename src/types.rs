use std::rc::Rc;

use crate::types::MalErr::ErrString;

pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Sym(String),
    List(Rc<Vec<MalVal>>, Rc<MalVal>),
    Vector(Rc<Vec<MalVal>>, Rc<MalVal>),
}

pub enum MalErr {
    ErrString(String),
}

pub type MalRet = Result<MalVal, MalErr>;

macro_rules! list {
    ($seq:expr) => {{
       List(Rc::new($seq), Rc::new(Nil))
    }};
    [$($args:expr), *] => {{
        let v: Vec<MalVal> = vec![$($args), *];
        List(Rc::new(v), Rc::new(Nil))
    }}
}

macro_rules! vector {
    ($seq:expr) => {{
        Vector(Rc::new($seq), Rc::new(Nil))
    }};
    [$($args:expr), *] => {{
        let v: Vec<MalVal> = vec![$($args), *];
        Vector(Rc::new(v), Rc::new(Nil))
    }}
}

pub fn error(s: &str) -> MalRet {
    Err(ErrString(s.to_string()))
}

pub fn format_error(e: MalErr) -> String {
    match e {
        ErrString(s) => s.clone(),
    }
}