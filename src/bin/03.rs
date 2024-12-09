use std::iter::from_fn;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let tokens = tokenizer(input);

    let mut total_result = 0;
    let mut tok_cursor = 0;

    while tok_cursor < tokens.len() {
        match tokens[tok_cursor] {
            Token::Mul => {
                let (start, end) = (tok_cursor, (tok_cursor + 6));
                if end > tokens.len() {
                    break;
                }
                if let Some((num1, num2)) = process_seq(&tokens[start..end]) {
                    total_result += num1 * num2;
                    tok_cursor += 5;
                }
            }
            Token::Eof => break,
            _ => {}
        }
        tok_cursor += 1
    }

    Some(total_result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tokens = tokenizer(input);

    let mut total_result = 0;
    let mut tok_cursor = 0;
    let mut mul_enabled = true;

    while tok_cursor < tokens.len() {
        match tokens[tok_cursor] {
            Token::Mul => {
                let (start, end) = (tok_cursor, (tok_cursor + 6));
                if end > tokens.len() {
                    break;
                }
                if let Some((num1, num2)) = process_seq(&tokens[start..end]) {
                    if mul_enabled {
                        total_result += num1 * num2;
                    }
                    tok_cursor += 5;
                }
            }
            Token::Do => mul_enabled = true,
            Token::Dont => mul_enabled = false,
            Token::Eof => break,
            _ => {}
        }
        tok_cursor += 1
    }

    Some(total_result)
}

#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    Comma,
    Mul,
    Do,
    Dont,
    Number(u32),
    WhiteSpace,
    Invalid,
    Eof,
}

fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ',' => tokens.push(Token::Comma),
            'm' if matches_suffix(&mut iter, "ul") => {
                tokens.push(Token::Mul);
            }
            'd' => {
                if matches_suffix(&mut iter, "o") {
                    if matches_suffix(&mut iter, "n't()") {
                        tokens.push(Token::Dont);
                    } else if matches_suffix(&mut iter, "()") {
                        tokens.push(Token::Do);
                    } else {
                        tokens.push(Token::Invalid);
                    }
                } else {
                    tokens.push(Token::Invalid);
                }
            }
            first_digit @ '1'..='9' => {
                let other_digits: String =
                    from_fn(|| iter.by_ref().next_if(|c| c.is_ascii_digit())).collect();

                let num = format!("{first_digit}{other_digits}");

                match num.parse::<u32>() {
                    Ok(n) if num.len() <= 3 => tokens.push(Token::Number(n)),
                    _ => tokens.push(Token::Invalid),
                }
            }
            ch if ch.is_whitespace() => tokens.push(Token::WhiteSpace),
            _ => tokens.push(Token::Invalid),
        }
    }

    tokens.push(Token::Eof);

    tokens
}

fn matches_suffix(iter: &mut std::iter::Peekable<std::str::Chars>, keyword: &str) -> bool {
    for expected in keyword.chars() {
        if iter.next_if_eq(&expected).is_none() {
            return false;
        }
    }
    true
}

fn process_seq(seq: &[Token]) -> Option<(u32, u32)> {
    use Token::*;

    match seq {
        [Mul, OpenParen, Number(num1), Comma, Number(num2), CloseParen] => Some((*num1, *num2)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
