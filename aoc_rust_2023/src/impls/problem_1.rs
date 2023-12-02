use crate::{AocResult, Part, Solution};

fn find_number(line_chars: impl Iterator<Item = char>) -> AocResult<u32> {
    for maybe_digit in line_chars {
        if let Some(digit) = maybe_digit.to_digit(10) {
            return Ok(digit);
        }
    }
    anyhow::bail!("The AOC input is broken?!?!?!?!?!?!?");
}

fn find_number_or_written_out_number(
    line_chars: impl Iterator<Item = char>,
    mini_string_maker: &[char],
) -> AocResult<u32> {
    for (index, maybe_digit) in line_chars.enumerate() {
        if let Some(digit) = maybe_digit.to_digit(10) {
            return Ok(digit);
        }
        let mini_str = &mini_string_maker[index..std::cmp::min(index + 5, mini_string_maker.len())];
        match mini_str {
            ['t', 'h', 'r', 'e', 'e'] | ['e', 'e', 'r', 'h', 't'] => return Ok(3),
            ['s', 'e', 'v', 'e', 'n'] | ['n', 'e', 'v', 'e', 's'] => return Ok(7),
            ['e', 'i', 'g', 'h', 't'] | ['t', 'h', 'g', 'i', 'e'] => return Ok(8),
            _ => (),
        };
        let mini_str = &mini_str[0..std::cmp::min(4, mini_str.len())];
        match mini_str {
            ['f', 'o', 'u', 'r'] | ['r', 'u', 'o', 'f'] => return Ok(4),
            ['f', 'i', 'v', 'e'] | ['e', 'v', 'i', 'f'] => return Ok(5),
            ['n', 'i', 'n', 'e'] | ['e', 'n', 'i', 'n'] => return Ok(9),
            _ => (),
        };
        let mini_str = &mini_str[0..std::cmp::min(3, mini_str.len())];
        match mini_str {
            ['o', 'n', 'e'] | ['e', 'n', 'o'] => return Ok(1),
            ['t', 'w', 'o'] | ['o', 'w', 't'] => return Ok(2),
            ['s', 'i', 'x'] | ['x', 'i', 's'] => return Ok(6),
            _ => (),
        }
    }
    anyhow::bail!("The AOC input is broken?!?!?!?!?!?!?");
}

fn solve_p1_part_1(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut sum = 0;
    for line in lines {
        let line = line?;
        let first_digit = find_number(line.trim().chars())?;
        let second_digit = find_number(line.trim().chars().rev())?;
        sum += first_digit * 10 + second_digit;
    }
    Ok(Solution::Int(sum as i32))
}

fn solve_p2_part_2(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut sum = 0;
    for line in lines {
        let line = line?;
        let char_array: Vec<char> = line.chars().collect();
        let reversed_char_array: Vec<char> = line.chars().rev().collect();
        let first_digit = find_number_or_written_out_number(line.trim().chars(), &char_array)?;
        let second_digit =
            find_number_or_written_out_number(line.trim().chars().rev(), &reversed_char_array)?;
        sum += first_digit * 10 + second_digit;
    }
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
