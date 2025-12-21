//! Advent of Code 2025 day 06
//! ```
//! use aoc::day06::Homework;
//! let homework = Homework::from_file("input/day06_input.txt");
//! assert_eq!(homework.calc(), 8108520669952);
//! assert_eq!(homework.calc2(), 11708563470209);
//! ```

use std::fs::read_to_string;

use nom::IResult;
use nom::Parser;
use nom::character::complete::char;
use nom::character::complete::i64 as nomi64;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Mult,
    Add,
}
pub struct Homework {
    raw_data: String,
    table: Vec<Vec<i64>>,
    operations: Vec<Op>,
}

impl Homework {
    pub fn from_file(fname: &str) -> Self {
        let raw_data = read_to_string(fname).unwrap();
        let (_, (table, operations)) = separated_pair(
            separated_list1(line_ending, number_line),
            line_ending,
            op_line,
        )
        .parse(raw_data.as_str())
        .unwrap();

        Homework {
            raw_data,
            table,
            operations,
        }
    }

    pub fn calc(&self) -> i64 {
        let mut sum = 0;
        for col in 0..self.table[0].len() {
            let mut operands = Vec::new();
            for row in 0..self.table.len() {
                operands.push(self.table[row][col].clone());
            }
            sum += apply_op(self.operations[col], operands);
        }

        sum
    }

    // Part 2
    pub fn calc2(&self) -> i64 {
        // First, extract the columnt widths
        // Then constrcut the numbers and perform the operations
        let mut lines = self
            .raw_data
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        let mut op_line = lines.pop().unwrap();
        // handle empty line if file ends with it:
        if op_line.is_empty() {
            op_line = lines.pop().unwrap();
        }
        dbg!(format!("{op_line}"));
        debug_assert_eq!(op_line[0..1], *"*");

        let mut col_len: Vec<usize> = Vec::new();
        let mut ops: Vec<Op> = Vec::new();
        let mut cur_len: usize = 0;
        for c in op_line.chars() {
            match c {
                '*' => {
                    ops.push(Op::Mult);
                    if cur_len > 0 {
                        col_len.push(cur_len);
                        cur_len = 0;
                    }
                }
                '+' => {
                    ops.push(Op::Add);
                    if cur_len > 0 {
                        col_len.push(cur_len);
                        cur_len = 0;
                    }
                }
                ' ' => {
                    cur_len += 1;
                }
                _ => {
                    panic!("Illegal character {c}");
                }
            }
        }
        // Add the final column's length
        // Have to add 1 - the other columns have added the column separator space
        col_len.push(cur_len + 1);
        dbg!(format!("Col lens: {:?}", col_len));
        dbg!(format!("Ops: {:?}", ops));

        // Extract the numbers from the columns
        let mut acc: i64 = 0;
        for col in 0..ops.len() {
            // Determine start of column, account for the separators after the first column
            dbg!("Column: {:?}", col);
            let col_start = col_len[0..col].iter().sum::<usize>() + col;
            let col_end = col_start + col_len[col];

            let mut number_row: Vec<i64> = Vec::new();
            for n in col_start..col_end {
                let mut n_string = "".to_string();
                for line in lines.clone() {
                    n_string += &line.chars().nth(n).unwrap().to_string();
                }
                dbg!(format!("number: {n_string}"));
                number_row.push(n_string.trim().parse().unwrap());
            }
            // Perform operation on column
            acc += apply_op(ops[col], number_row);
        }

        acc
    }
}

// parsers
fn number_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (rest, number_line) = terminated(
        separated_list1(char(' '), preceded(opt(many1(char(' '))), nomi64)),
        opt(many1(char(' '))),
    )
    .parse(input)?;
    Ok((rest, number_line))
}

fn op_line(input: &str) -> IResult<&str, Vec<Op>> {
    let (rest, op_line_raw) = separated_list1(many1(char(' ')), one_of("+*")).parse(input)?;
    let op_line = op_line_raw
        .iter()
        .map(|x| match x {
            '+' => Op::Add,
            '*' => Op::Mult,
            _ => {
                panic!("Illegal operand: {x}")
            }
        })
        .collect();
    Ok((rest, op_line))
}

fn apply_op(op: Op, row: Vec<i64>) -> i64 {
    let first = row.first().unwrap().clone();
    row.iter().cloned().skip(1).fold(first, |acc, x| match op {
        Op::Mult => acc * x,
        Op::Add => acc + x,
    })
}

#[cfg(test)]
mod tests {
    use super::Homework;
    use super::Op;
    use super::apply_op;
    use super::number_line;
    use nom::Parser;

    #[test]
    fn test_homework() {
        let homework = Homework::from_file("input/day06_sample.txt");

        assert_eq!(homework.table.len(), 3);
        assert_eq!(homework.table[0][0], 123);
        assert_eq!(homework.table[2][3], 314);

        assert_eq!(homework.operations[0], Op::Mult);

        assert_eq!(homework.calc(), 4277556);

        assert_eq!(homework.calc2(), 3263827);
    }

    #[test]
    fn test_apply_op() {
        let row = vec![1, 2, 4];
        let op = Op::Mult;
        assert_eq!(apply_op(op, row), 8);

        let row2 = vec![1, 2, 4];
        let op2 = Op::Add;
        assert_eq!(apply_op(op2, row2), 7);
    }

    #[test]
    fn test_number_line() {
        let input = "3 4 5";
        assert_eq!(number_line.parse(input).unwrap().1, vec![3, 4, 5]);
    }
}
