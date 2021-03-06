// https://en.wikipedia.org/wiki/Shunting_yard_algorithm
#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens,
}

pub struct Calculator {}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expression: T) -> Result<Vec<Token>, Error> {
        let expression = expression.as_ref();
        let chars = expression.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();

        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit))
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                }
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                ' ' => {}
                '\n' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }

        if !parens.is_empty() {
            return Err(Error::MismatchedParens);
        }

        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                }
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                }
                _ => {}
            }
        }
        while !stack.is_empty() {
            queue.push(stack.pop().unwrap());
        }

        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Op(Operator::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    stack.push(left + right);
                }
                Token::Op(Operator::Sub) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    stack.push(left - right);
                }
                Token::Op(Operator::Mul) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    stack.push(left * right);
                }
                Token::Op(Operator::Div) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    stack.push(left / right);
                }
                _ => {}
            }
        }

        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn parse_expression_to_tokens() {
        let tokens = Calculator::parse("2 * 2 + 48 / 4").unwrap();
        let inflix_expr = vec![
            Token::Number(2),
            Token::Op(Operator::Mul),
            Token::Number(2),
            Token::Op(Operator::Add),
            Token::Number(48),
            Token::Op(Operator::Div),
            Token::Number(4),
        ];

        assert_eq!(inflix_expr, tokens);
    }

    #[test]
    fn inflix_to_postfix_notation() {
        let tokens = Calculator::parse("2 * 2 + 48 / 4").unwrap();
        let postfix_expr = vec![
            Token::Number(2),
            Token::Number(2),
            Token::Op(Operator::Mul),
            Token::Number(48),
            Token::Number(4),
            Token::Op(Operator::Div),
            Token::Op(Operator::Add),
        ];

        assert_eq!(postfix_expr, Calculator::expression(tokens))
    }

    #[test]
    fn evaluate_postfix_expression() {
        let tokens = Calculator::parse("2 * 2 + 48 / 4").unwrap();
        let expression = Calculator::expression(tokens);

        assert_eq!(16_f32, Calculator::evaluate(expression).unwrap_or(10.0));
    }
}
