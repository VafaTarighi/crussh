
const WHITESPACE: &[char] = &[' ', '\n'];

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

        let extracted = &s[0..extracted_end];
        let remainder = &s[extracted_end..];
        (remainder, extracted)
}

fn take_while1(
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

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("expected {}", starting_text))
    }
}

pub(crate) fn extract_shell_ident(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_quote = s
        .chars()
        .next()
        .map(|c| c == '"')
        .unwrap_or(false);

    if input_starts_with_quote {
        let s = tag("\"", s)?;

        let (s, id) = take_while1(
            |c| c != '"', 
            s, 
            "expected at least one character".to_string()
        )?;
            
        let s = tag("\"", s)?;
        Ok((s, id))
    } else {
        take_while1(|c| !vec![' ', '<', '>', '|'].contains(&c), s, "expected at least one character".to_string())
    }
}