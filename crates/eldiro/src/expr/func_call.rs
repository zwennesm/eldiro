use super::Expr;
use crate::env::Env;
use crate::val::Val;
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_identifier(s)?;
        let (s, _) = utils::take_while(|c| c == ' ', s);

        let (s, params) = utils::sequence_error(
            Expr::new, 
            |s| utils::take_while(|c| c == ' ', s),
            s
        )?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params,
            }
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut child_env = env.create_child();

        let (param_names, body) = env.get_func(&self.callee)?;

        let num_expected_params = param_names.len();
        let num_actual_params = self.params.len();

        if num_expected_params != num_actual_params {
            return Err(format!(
                "expected {} parameters, got {}",
                num_expected_params, num_actual_params,
            ));
        }

        for (param_name, param_expr) in param_names.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.store_binding(param_name, param_val);
        }

        body.eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Number;
    use super::*; 

    #[test]
    fn parse_func_call_with_one_parameter() {
        assert_eq!(
            FuncCall::new("factorial 10"),
            Ok((
                "",
                FuncCall {
                    callee: "factorial".to_string(),
                    params: vec![Expr::Number(Number(10))],
                },
            )),
        );
    }
}