const WHITESPACE: &[char] = &[' ', '\n', '\t'];

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted = s.char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    (&s[extracted..], &s[..extracted])
}

fn take_while_error(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

pub(crate) fn sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    mut s: &str,
) -> Result<(&str, Vec<T>), String> {
    let mut items = Vec::new();

    while let Ok((new_s, item)) = parser(s) {
        s = new_s;
        items.push(item);

        let (new_s, _) = separator_parser(s);
        s = new_s;
    }

    Ok((s, items))
}

pub(crate) fn sequence_error<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    s: &str,
) -> Result<(&str, Vec<T>), String> {
    let (s, sequence) = sequence(parser, separator_parser, s)?;

    if sequence.is_empty() {
        Err("expected a sequence with more than one item".to_string())
    } else {
        Ok((s, sequence))
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("expected {}", starting_text))
    }
} 

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while_error(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

pub(crate) fn extract_whitespace_error(s: &str) -> Result<(&str, &str), String> {
    take_while_error(|c| WHITESPACE.contains(&c), s, "expected a whitespace".to_string())
}

pub(crate) fn extract_identifier(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);
    
    if input_starts_with_alphabetic {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("expected identifier".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }

    #[test]
    fn extract_digits_that_take_full_input() {
        assert_eq!(extract_digits("10000"), Ok(("", "10000")));
    }

    #[test]
    fn do_not_extract_spaces_when_input_does_not_start_with_them() {
        assert_eq!(
            extract_whitespace_error("blah"),
            Err("expected a whitespace".to_string())
        )
    }

    #[test]
    fn extract_newlines_spaces_and_tabs() {
        assert_eq!(extract_whitespace(" \n   \n\tabc"), ("abc", " \n   \n\t"))
    }

    #[test]
    fn extract_alphabetic_identifier() {
        assert_eq!(extract_identifier("GreetingS hi"), Ok((" hi", "GreetingS")));
    }

    #[test]
    fn extract_alphanumeric_identifier() {
        assert_eq!(extract_identifier("base64()"), Ok(("()", "base64")));
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(
            extract_identifier("123foo"),
            Err("expected identifier".to_string())    
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
}