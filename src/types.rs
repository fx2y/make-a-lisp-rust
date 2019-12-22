use std::rc::Rc;

use fnv::FnvHashMap;
use itertools::Itertools;

use crate::types::MalErr::ErrString;
use crate::types::MalVal::{Bool, Hash, Int, List, Nil, Str, Sym, Vector};

#[derive(Clone)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Sym(String),
    List(Rc<Vec<MalVal>>, Rc<MalVal>),
    Vector(Rc<Vec<MalVal>>, Rc<MalVal>),
    Hash(Rc<FnvHashMap<String, MalVal>>, Rc<MalVal>),
}

pub enum MalErr {
    ErrString(String),
}

pub type MalArgs = Vec<MalVal>;
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

impl PartialEq for MalVal {
    fn eq(&self, other: &MalVal) -> bool {
        match (self, other) {
            (Nil, Nil) => true,
            (Bool(ref a), Bool(ref b)) => a == b,
            (Int(ref a), Int(ref b)) => a == b,
            (Str(ref a), Str(ref b)) => a == b,
            (Sym(ref a), Sym(ref b)) => a == b,
            (List(ref a, _), List(ref b, _))
            | (Vector(ref a, _), Vector(ref b, _))
            | (List(ref a, _), Vector(ref b, _))
            | (Vector(ref a, _), List(ref b, _)) => a == b,
            (Hash(ref a, _), Hash(ref b, _)) => a == b,
            _ => false,
        }
    }
}

fn _assoc(mut hm: FnvHashMap<String, MalVal>, kvs: MalArgs) -> MalRet {
    if kvs.len() % 2 != 0 {
        return error("odd number of elements");
    }
    for (k, v) in kvs.iter().tuples() {
        match k {
            Str(s) => {
                hm.insert(s.to_string(), v.clone());
            }
            _ => return error("key is not string"),
        }
    }
    Ok(Hash(Rc::new(hm), Rc::new(Nil)))
}

pub fn error(s: &str) -> MalRet {
    Err(ErrString(s.to_string()))
}

pub fn format_error(e: MalErr) -> String {
    match e {
        ErrString(s) => s.clone(),
    }
}

pub fn hash_map(kvs: MalArgs) -> MalRet {
    let hm: FnvHashMap<String, MalVal> = FnvHashMap::default();
    _assoc(hm, kvs)
}