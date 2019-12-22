use std::cell::RefCell;
use std::rc::Rc;

use fnv::FnvHashMap;

use crate::types::{error, MalRet, MalVal};
use crate::types::MalErr::ErrString;
use crate::types::MalVal::Sym;

pub struct EnvStruct {
    data: RefCell<FnvHashMap<String, MalVal>>,
    pub outer: Option<Env>,
}

pub type Env = Rc<EnvStruct>;

pub fn env_new(outer: Option<Env>) -> Env {
    Rc::new(EnvStruct {
        data: RefCell::new(FnvHashMap::default()),
        outer,
    })
}

pub fn env_find(env: &Env, key: &str) -> Option<Env> {
    match (env.data.borrow().contains_key(key), env.outer.clone()) {
        (true, _) => Some(env.clone()),
        (false, Some(o)) => env_find(&o, key),
        _ => None,
    }
}

pub fn env_get(env: &Env, key: &MalVal) -> MalRet {
    match key {
        Sym(ref s) => match env_find(env, s) {
            Some(e) => Ok(e.data.borrow().get(s).ok_or(ErrString(format!("'{}' not found", s)))?.clone()),
            _ => error(&format!("'{}' not found", s)),
        },
        _ => error("Env.get called with non-Str"),
    }
}

pub fn env_sets(env: &Env, key: &str, val: MalVal) {
    env.data.borrow_mut().insert(key.to_string(), val);
}