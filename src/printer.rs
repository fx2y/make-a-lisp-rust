use crate::types::MalVal;
use crate::types::MalVal::{Bool, Int, Nil, Str, Sym};

fn escape_str(s: &str) -> String {
    s.chars().map(|c| match c {
        '"' => "\\\"".to_string(),
        '\n' => "\\n".to_string(),
        '\\' => "\\\\".to_string(),
        _ => c.to_string(),
    }).collect::<Vec<String>>().join("")
}

impl MalVal {
    pub fn pr_str(&self, print_readably: bool) -> String {
        match self {
            Nil => String::from("nil"),
            Bool(true) => String::from("true"),
            Bool(false) => String::from("false"),
            Int(i) => format!("{}", i),
            Str(s) => {
                if s.starts_with("\u{29e}") {
                    format!(":{}", &s[2..])
                } else if print_readably {
                    format!("\"{}\"", escape_str(s))
                } else {
                    s.clone()
                }
            }
            Sym(s) => s.clone(),
        }
    }
}