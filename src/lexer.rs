#[derive(Debug, Clone, Copy)]
enum State {
    Plain,
    Single,
    Double,
}

pub fn lex(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut word: Option<String> = None;
    let mut state: State = State::Plain;
    let mut input_chars = input.chars();
    while let Some(c) = input_chars.next() {
        match (state, c) {
            (State::Plain, ' ') => {
                if let Some(finished) = word.take() {
                    output.push(finished);
                }
            }
            (State::Plain, '\'') => {
                word.get_or_insert_with(String::new);
                state = State::Single;
            }
            (State::Plain, '"') => {
                word.get_or_insert_with(String::new);
                state = State::Double;
            }
            (State::Plain, c) if c.is_whitespace() => {
                if let Some(finished) = word.take() {
                    output.push(finished);
                }
            }

            (State::Single, '\'') => {
                state = State::Plain;
            }

            (State::Double, '"') => {
                state = State::Plain;
            }

            (_, c) => word.get_or_insert_with(String::new).push(c),
        }
    }
    if let Some(finished) = word.take() {
        output.push(finished);
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_words() {
        assert_eq!(lex("echo hi"), ["echo", "hi"]);
    }

    #[test]
    fn single_quote() {
        assert_eq!(lex("echo 'a    b'"), ["echo", "a    b"]);
    }

    #[test]
    fn double_quotes_preserve_spaces() {
        assert_eq!(lex("echo \"a   b\""), ["echo", "a   b"]);
    }

    #[test]
    fn quotes_concatenate_into_one_word() {
        assert_eq!(lex("echo a'b'c"), ["echo", "abc"]);
    }

    #[test]
    fn empty_quotes_are_still_a_word() {
        assert_eq!(lex("echo ''"), ["echo", ""]);
    }

    #[test]
    fn repeated_spaces_do_not_make_empty_words() {
        assert_eq!(lex("echo   hi"), ["echo", "hi"]);
    }

    #[test]
    fn trailing_space_does_not_make_empty_word() {
        assert_eq!(lex("echo hi "), ["echo", "hi"]);
    }

    #[test]
    fn leading_space_does_not_make_empty_word() {
        assert_eq!(lex(" echo hi"), ["echo", "hi"]);
    }

    #[test]
    fn empty_input_makes_no_words() {
        let empty: Vec<String> = Vec::new();
        assert_eq!(lex(""), empty);
    }

    #[test]
    fn spaces_split_outside_quotes_only() {
        assert_eq!(lex("a \"b c\" d"), ["a", "b c", "d"]);
    }

    #[test]
    fn trailing_newline_is_not_part_of_a_word() {
        assert_eq!(lex("echo hi\n"), ["echo", "hi"]);
    }

    #[test]
    fn tabs_separate_words() {
        assert_eq!(lex("echo\thi"), ["echo", "hi"]);
    }
}
