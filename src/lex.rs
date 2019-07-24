/***
 * Defines all of the tokens that can be produced by the lexer's `next_token` function.
 * These tokens define both the valid and invalid lexemes of an oso source file.
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Fn,
    Identifier(String),
    Invalid,
}

/***
 * Defines all of the internal states of the lexer. Since the lexer is an NFA, transition functions
 * exist on each state for each possible input character.
 */
enum States {
    Begin,
    Collect,
    Hex,
    Numeric,
    Number,
    Resolve,
}

trait TokenResolver {
    fn resolve(&self) -> Token;
}

impl TokenResolver for String {
    fn resolve(&self) -> Token {
        return match self.as_str() {
            "fn" => Token::Fn,
            string => Token::Identifier(String::from(string))
        }
    }
}

/***
 * Scans a vector of characters that represents a source file for the first lexeme and returns the
 * result.
 */
pub fn next_token(input: &Vec<char>, current_ptr: &mut usize) -> Token {
    let mut result = Token::EOF;
    let mut current_state = States::Begin;
    let mut collected = String::from("");
    while *current_ptr < input.len() {
        match current_state {
            States::Begin => {
                if input[*current_ptr].is_alphabetic() {
                    current_state = States::Collect;
                    collected.push(input[*current_ptr]);
                } else if input[*current_ptr].is_numeric() {
                    current_state = States::Numeric;
                    collected.push(input[*current_ptr]);
                }
                *current_ptr += 1;
            }
            States::Collect => {
                if input[*current_ptr].is_alphanumeric() {
                    collected.push(input[*current_ptr]);
                    *current_ptr += 1;
                } else {
                    current_state = States::Resolve;
                }
            }
            States::Hex => { /* FIXME */ }
            States::Numeric => { /* FIXME */ }
            States::Number => {
                if input[*current_ptr].is_numeric() {
                    collected.push(input[*current_ptr]);
                } else if input[*current_ptr].is_alphabetic() {
                    return Token::Invalid;
                }
                *current_ptr += 1;
            }
            States::Resolve => {
                result = collected.resolve();
                break;
            }
        }
    }
    if collected.len() > 0 {
        result = collected.resolve();
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    /*** Helpers ***/

    macro_rules! assert_eq_pretty {
        ($e1:expr, $e2:expr) => {
            if $e1 != $e2 {
                panic!("Expected {:#?} | Actual {:#?}", $e1, $e2);
            }
        }
    }

    fn to_chars(string: &str) -> Vec<char> {
        String::from(string).chars().collect::<Vec<char>>()
    }

    /*** Unit Tests ***/

    #[test]
    fn eof_test1() {
        let result = next_token(&to_chars(""), &mut 0);
        assert_eq_pretty!(Token::EOF, result);
    }

    #[test]
    fn eof_test2() {
        let current_ptr = &mut 0;
        let input = &to_chars("fn ");
        next_token(input, current_ptr);
        let result = next_token(input, current_ptr);
        assert_eq_pretty!(Token::EOF, result);
    }

    #[test]
    fn fn_test1() {
        let result = next_token(&to_chars("fn"), &mut 0);
        assert_eq_pretty!(Token::Fn, result);
    }

    #[test]
    fn fn_test2() {
        let result = next_token(&to_chars(" fn"), &mut 0);
        assert_eq_pretty!(Token::Fn, result);
    }

    #[test]
    fn fn_test3() {
        let result = next_token(&to_chars("\nfn"), &mut 0);
        assert_eq_pretty!(Token::Fn, result);
    }
}
