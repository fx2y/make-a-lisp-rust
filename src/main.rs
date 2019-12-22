extern crate fnv;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rustyline;

use std::rc::Rc;

use fnv::FnvHashMap;
use rustyline::Editor;
use rustyline::error::ReadlineError;

use crate::env::{Env, env_get, env_new, env_set, env_sets};
use crate::types::{error, format_error, func, MalArgs, MalErr, MalRet, MalVal};
use crate::types::MalVal::{Hash, Int, List, Nil, Sym, Vector};

#[macro_use]
mod types;
mod env;
mod printer;
mod reader;

fn read(str: &str) -> MalRet {
    reader::read_str(str.to_string())
}

fn eval_ast(ast: &MalVal, env: &Env) -> MalRet {
    match ast {
        Sym(_) => Ok(env_get(&env, &ast)?),
        List(v, _) => {
            let mut lst: MalArgs = vec![];
            for a in v.iter() {
                lst.push(eval(a.clone(), env.clone())?)
            }
            Ok(list!(lst))
        }
        Vector(v, _) => {
            let mut lst: MalArgs = vec![];
            for a in v.iter() {
                lst.push(eval(a.clone(), env.clone())?)
            }
            Ok(vector!(lst))
        }
        Hash(hm, _) => {
            let mut new_hm: FnvHashMap<String, MalVal> = FnvHashMap::default();
            for (k, v) in hm.iter() {
                new_hm.insert(k.to_string(), eval(v.clone(), env.clone())?);
            }
            Ok(Hash(Rc::new(new_hm), Rc::new(Nil)))
        }
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: MalVal, env: Env) -> MalRet {
    match ast.clone() {
        List(l, _) => {
            if l.len() == 0 {
                return Ok(ast);
            }
            let a0 = &l[0];
            match a0 {
                Sym(ref a0sym) if a0sym == "def!" => {
                    env_set(&env, l[1].clone(), eval(l[2].clone(), env.clone())?)
                }
                _ => match eval_ast(&ast, &env)? {
                    List(ref el, _) => {
                        let ref f = el[0].clone();
                        f.apply(el[1..].to_vec())
                    }
                    _ => error("expected a list")
                }
            }
        }
        _ => eval_ast(&ast, &env),
    }
}

fn print(ast: &MalVal) -> String {
    ast.pr_str(true)
}

fn rep(str: &str, env: &Env) -> Result<String, MalErr> {
    let ast = read(str)?;
    let exp = eval(ast, env.clone())?;
    Ok(print(&exp))
}

fn int_op(op: fn(i64, i64) -> i64, a: MalArgs) -> MalRet {
    match (a[0].clone(), a[1].clone()) {
        (Int(a0), Int(a1)) => Ok(Int(op(a0, a1))),
        _ => error("invalid int_op args"),
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    let repl_env = env_new(None);
    env_sets(&repl_env, "+", func(|a: MalArgs| int_op(|i, j| i + j, a)));
    env_sets(&repl_env, "-", func(|a: MalArgs| int_op(|i, j| i - j, a)));
    env_sets(&repl_env, "*", func(|a: MalArgs| int_op(|i, j| i * j, a)));
    env_sets(&repl_env, "/", func(|a: MalArgs| int_op(|i, j| i / j, a)));

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rl.save_history(".mal-history").unwrap();
                if line.len() > 0 {
                    match rep(&line, &repl_env) {
                        Ok(out) => println!("{}", out),
                        Err(e) => println!("Error: {}", format_error(e)),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
