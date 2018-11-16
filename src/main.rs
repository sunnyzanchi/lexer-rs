extern crate itertools;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Op {
  Add,
  Divide,
  Multiply,
  Subtract,
}

#[derive(Debug, PartialEq)]
enum Token {
  LeftParen,
  Number(i32),
  Operator(Op),
  RightParen,
}

fn lexer(input: &String) -> Result<Vec<Token>, String> {
  use Token::*;
  let mut chars = input.chars().peekable();
  let mut result: Vec<Token> = Vec::new();

  while let Some(&c) = chars.peek() {
    match c {
      '0'...'9' => {
        let str_number =
          chars
            .take_while_ref(|c| c.is_numeric())
            .fold(String::new(), |mut acc, c| {
              acc.push(c);
              acc
            });
        if let Ok(number) = str_number.parse() {
          result.push(Number(number))
        } else {
          return Err(format!("failed to parse {} as an i32", str_number));
        }
      }
      '+' => {
        result.push(Operator(Op::Add));
        chars.next();
      }
      '/' => {
        result.push(Operator(Op::Divide));
        chars.next();
      }
      '*' => {
        result.push(Operator(Op::Multiply));
        chars.next();
      }
      '-' => {
        result.push(Operator(Op::Subtract));
        chars.next();
      }
      '(' => {
        result.push(LeftParen);
        chars.next();
      }
      ')' => {
        result.push(RightParen);
        chars.next();
      }
      ' ' => {
        chars.next();
      }
      _ => return Err(format!("unexpected character {}", c)),
    }
  }

  Ok(result)
}

fn main() {}

mod test {
  #[test]
  fn lexer() {
    use lexer;
    use Op;
    use Token::*;
    let input = String::from("(13 + 42) * (12 / 2)");

    let tokens = lexer(&input).unwrap();
    let expected = vec![
      LeftParen,
      Number(13),
      Operator(Op::Add),
      Number(42),
      RightParen,
      Operator(Op::Multiply),
      LeftParen,
      Number(12),
      Operator(Op::Divide),
      Number(2),
      RightParen,
    ];
    assert_eq!(tokens, expected);
  }
}
