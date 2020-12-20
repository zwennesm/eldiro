use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, ident) = utils::extract_identifier(s)?;

        Ok((
            s,
            Self {
                name: ident.to_string(),
            }
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string(),
                }
            ))
        )
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_non_existing_binding_usage() {
        let empty_env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "missing".to_string(),
            }
            .eval(&empty_env),
            Err("binding with name \'missing\' does not exist".to_string())
        )
    }
}