mod binding_def;
mod expr;
mod val;
mod stmt;

mod env;
mod utils;

use env::Env;
use val::Val;

struct Parse(stmt::Stmt);

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("Input was not consumed fully by parser".to_string())
    }
}