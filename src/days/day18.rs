use crate::day::Day;

use core::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub enum Expression {
    Value(usize),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
    Brackets(Box<Expression>),
}

impl Expression {
    fn read(bytes: &Vec<u8>, i: usize) -> (Self, usize) {
        let (mut curr_expr, mut my_i) = Expression::read_one(bytes, i);
        while my_i < bytes.len() && bytes[my_i] != b')' {
            let ret = Expression::chain(curr_expr, bytes, my_i);
            curr_expr = ret.0;
            my_i = ret.1;
        }

        if my_i < bytes.len() && bytes[my_i] == b')' {
            my_i += 1;
        }

        (curr_expr, my_i)
    }

    fn read_one(bytes: &Vec<u8>, i: usize) -> (Self, usize) {
        let mut my_i = i;
        let left;
        if bytes[my_i] == b'(' {
            let ret = Expression::read(bytes, my_i + 1);
            left = Expression::Brackets(Box::new(ret.0));
            my_i = ret.1;
        } else {
            left = Expression::Value((bytes[my_i] - b'0') as usize);
            my_i += 1;
        }

        (left, my_i)
    }

    fn chain(left: Expression, bytes: &Vec<u8>, i: usize) -> (Self, usize) {
        let mut my_i = i;
        let operator = bytes[my_i];
        my_i += 1;

        let (right, my_i) = Expression::read_one(bytes, my_i);

        match operator {
            b'+' => (Expression::Sum(Box::new(left), Box::new(right)), my_i),
            b'*' => (Expression::Product(Box::new(left), Box::new(right)), my_i),
            x => panic!("Invalid Input: {}", x),
        }
    }

    fn calculate(&self) -> usize {
        match self {
            Expression::Value(x) => *x,
            Expression::Sum(x, y) => x.calculate() + y.calculate(),
            Expression::Product(x, y) => x.calculate() * y.calculate(),
            Expression::Brackets(x) => x.calculate(),
        }
    }

    pub fn to_part2(&self) -> (Self, bool) {
        match self {
            Expression::Value(x) => (Expression::Value(*x), false),
            Expression::Product(x, y) => {
                let (left, left_loop) = x.to_part2();
                let (right, right_loop) = y.to_part2();
                (
                    Expression::Product(Box::new(left), Box::new(right)),
                    left_loop || right_loop,
                )
            }
            Expression::Sum(x, y) => {
                if let Expression::Product(px, py) = *(x.clone()) {
                    (
                        Expression::Product(
                            Box::new(px.to_part2().0),
                            Box::new(Expression::Sum(Box::new(py.to_part2().0), y.clone())),
                        ),
                        true,
                    )
                } else {
                    let (left, left_loop) = x.to_part2();
                    let (right, right_loop) = y.to_part2();
                    (
                        Expression::Sum(Box::new(left), Box::new(right)),
                        left_loop || right_loop,
                    )
                }
            }
            Expression::Brackets(x) => {
                let (expr, expr_loop) = x.to_part2();
                (Expression::Brackets(Box::new(expr)), expr_loop)
            }
        }
    }
}

impl FromStr for Expression {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.bytes().filter(|x| *x != b' ').collect();
        Ok(Expression::read(&parts, 0).0)
    }
}

pub struct Day18 {}

impl Day for Day18 {
    type InputElement = Expression;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        input.iter().map(|x| x.calculate()).sum()
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        input.iter().map(|x| {
            let (mut ret, mut looping) = x.to_part2();
            while looping {
                let value = ret.to_part2();
                ret = value.0;
                looping = value.1;
            }
            ret
        }).map(|x| x.calculate()).sum()
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content
            .lines()
            .map(|x| x.parse::<Self::InputElement>().expect("Invalid input"))
            .collect()
    }
}
