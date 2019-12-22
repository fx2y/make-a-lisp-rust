use crate::types::MalErr::ErrString;

pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Sym(String),
}

pub enum MalErr {
    ErrString(String),
}

pub type MalRet = Result<MalVal, MalErr>;

pub fn error(s: &str) -> MalRet {
    Err(ErrString(s.to_string()))
}

pub fn format_error(e: MalErr) -> String {
    match e {
        ErrString(s) => s.clone(),
    }
}