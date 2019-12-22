use regex::{Captures, Regex};

use crate::types::{error, MalErr, MalRet};
use crate::types::MalErr::ErrString;
use crate::types::MalVal::{Bool, Int, Nil, Str, Sym};

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn next(&mut self) -> Result<String, MalErr> {
        self.pos = self.pos + 1;
        Ok(self.tokens.get(self.pos - 1).ok_or(ErrString("underflow".to_string()))?.to_string())
    }
}

fn tokenize(str: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }

    let mut res = vec![];
    for cap in RE.captures_iter(str) {
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }
    res
}

fn unescape_str(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\\(.)"#).unwrap();
    }
    RE.replace_all(&s, |caps: &Captures| {
        format!("{}", if &caps[1] == "n" { "\n" } else { &caps[1] })
    }).to_string()
}

fn read_atom(rdr: &mut Reader) -> MalRet {
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }
    let token = rdr.next()?;
    match &token[..] {
        "nil" => Ok(Nil),
        "false" => Ok(Bool(false)),
        "true" => Ok(Bool(true)),
        _ => {
            if INT_RE.is_match(&token) {
                Ok(Int(token.parse().unwrap()))
            } else if STR_RE.is_match(&token) {
                Ok(Str(unescape_str(&token[1..token.len() - 1])))
            } else if token.starts_with("\"") {
                error("expected '\"', got EOF")
            } else if token.starts_with(":") {
                Ok(Str(format!("\u{29e}{}", &token[1..])))
            } else {
                Ok(Sym(token.to_string()))
            }
        }
    }
}

pub fn read_str(str: String) -> MalRet {
    let tokens = tokenize(&str);
    println!("tokens: {:?}", tokens);
    if tokens.len() == 0 {
        return error("no input");
    }
    read_atom(&mut Reader {
        pos: 0,
        tokens,
    })
}