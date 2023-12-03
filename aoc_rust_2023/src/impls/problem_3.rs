use std::collections::HashSet;

use crate::{AocResult, Part, Solution};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Span {
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Clone, Copy)]
struct TaggedNum {
    num: u32,
    span: Span,
}

#[derive(Debug)]
struct Board {
    symbol_positions: HashSet<(usize, usize)>,
    nums: Vec<TaggedNum>,
}

impl Board {
    fn new() -> Self {
        Self {
            symbol_positions: HashSet::new(),
            nums: Vec::new(),
        }
    }

    fn build_from_lines(
        &mut self,
        lines: impl Iterator<Item = Result<String, std::io::Error>>,
    ) -> AocResult<()> {
        for (line_index, line) in lines.enumerate() {
            let line = line?;
            let mut eval_num = false;
            let mut start_span = [line_index, 0];
            let mut num_substr = Vec::new();
            for (char_index, c) in line.chars().enumerate() {
                if eval_num && c.is_digit(10) {
                    num_substr.push(c);
                } else if eval_num {
                    eval_num = false;
                    let num_str_buffer = num_substr.drain(..);
                    let number_str = String::from_iter(num_str_buffer);
                    let number: u32 = number_str.parse().unwrap();
                    let tagged_num = TaggedNum {
                        num: number,
                        span: Span {
                            start: (start_span[0], start_span[1]),
                            end: (line_index, char_index - 1),
                        },
                    };
                    self.nums.push(tagged_num);
                    start_span = [line_index, 0];
                    if c != '.' {
                        self.symbol_positions.insert((line_index, char_index));
                    }
                } else if !eval_num && c != '.' && !c.is_digit(10) {
                    self.symbol_positions.insert((line_index, char_index));
                } else if !eval_num && c != '.' && c.is_digit(10) {
                    eval_num = true;
                    start_span[1] = char_index;
                    num_substr.push(c);
                }
            }
            if eval_num {
                let num_str_buffer = num_substr.drain(..);
                let number_str = String::from_iter(num_str_buffer);
                let number: u32 = number_str.parse().unwrap();
                let tagged_num = TaggedNum {
                    num: number,
                    span: Span {
                        start: (start_span[0], start_span[1]),
                        end: (line_index, line.len() - 1),
                    },
                };
                self.nums.push(tagged_num);
            }
        }
        Ok(())
    }

    fn sum_parts(&self) -> u32 {
        let mut sum = 0;
        for &tagged_num in &self.nums {
            'outer_loop: for position in Self::span_iter(tagged_num.span) {
                for adjacent_position in Self::adjacent_positions(position) {
                    if self.symbol_positions.contains(&adjacent_position) {
                        sum += tagged_num.num;
                        break 'outer_loop;
                    }
                }
            }
        }
        sum
    }

    fn calculate_gear_ratios(&self) -> u32 {
        let mut sum = 0;
        for &symbol_position in &self.symbol_positions {
            let mut adjacent_num_count = 0;
            let mut adjacent_nums = [0, 0];
            'advance_symbol: for &tagged_num in &self.nums {
                for position in Self::span_iter(tagged_num.span) {
                    for adjacent_position in Self::adjacent_positions(position) {
                        if symbol_position == adjacent_position {
                            adjacent_nums[adjacent_num_count] = tagged_num.num;
                            adjacent_num_count += 1;
                            continue 'advance_symbol;
                        }
                    }
                }
            }
            if adjacent_num_count == 2 {
                sum += adjacent_nums[0] * adjacent_nums[1];
            }
        }
        sum
    }

    fn span_iter(span: Span) -> impl Iterator<Item = (usize, usize)> {
        let line_num = span.start.0;
        (span.start.1..=span.end.1).map(move |char_idx| (line_num, char_idx))
    }

    fn adjacent_positions(position: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            (position.0, position.1.saturating_sub(1)),
            (position.0, position.1 + 1),
            (position.0 + 1, position.1),
            (position.0.saturating_sub(1), position.1),
            (position.0.saturating_sub(1), position.1.saturating_sub(1)),
            (position.0.saturating_sub(1), position.1 + 1),
            (position.0 + 1, position.1.saturating_sub(1)),
            (position.0 + 1, position.1 + 1),
        ]
        .into_iter()
    }
}

fn solve_p1_part_1(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut board = Board::new();
    board.build_from_lines(lines)?;
    let sum = board.sum_parts();
    Ok(Solution::Int(sum as i32))
}

fn solve_p2_part_2(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut board = Board::new();
    board.build_from_lines(lines)?;
    let sum = board.calculate_gear_ratios();
    Ok(Solution::Int(sum as i32))
}

pub fn solve(
    part: Part,
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    match part {
        Part::One => solve_p1_part_1(lines),
        Part::Two => solve_p2_part_2(lines),
    }
}
