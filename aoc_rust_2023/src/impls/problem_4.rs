use std::collections::HashSet;
use std::collections::VecDeque;

use crate::{AocResult, Part, Solution};

fn create_card_hashmap(line: &str) -> AocResult<HashSet<u32>> {
    let mut winning_nums = HashSet::new();
    for winning_num_str in line.split(' ') {
        if winning_num_str.is_empty() {
            continue;
        }
        let num = winning_num_str.parse()?;
        winning_nums.insert(num);
    }
    Ok(winning_nums)
}

fn create_winning_nums(line: &str) -> AocResult<Box<[u32]>> {
    let my_nums = line.split(' ');
    let mut my_num_vec = match my_nums.size_hint() {
        (_, None) => Vec::new(),
        (_, Some(size)) => Vec::with_capacity(size),
    };
    for num in my_nums {
        if num.is_empty() {
            continue;
        }
        let num = num.parse()?;
        my_num_vec.push(num);
    }
    Ok(my_num_vec.into_boxed_slice())
}

fn solve_p1_part_1(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut sum = 0;
    for card_row in lines {
        let card_row = card_row?;
        let (_, num_portion) = card_row.split_once(':').ok_or(anyhow::anyhow!(
            "AOC has invalid input!!?!??! Input: {card_row}"
        ))?;
        let (winning_nums, my_nums) = num_portion.split_once('|').ok_or(anyhow::anyhow!(
            "AOC has invalid input?!?!?!?!?! Input: {num_portion}"
        ))?;
        let winning_num_set = create_card_hashmap(winning_nums)?;
        let my_nums = create_winning_nums(&my_nums)?;
        let mut row_sum = 0;
        for &my_num in my_nums.iter() {
            if winning_num_set.contains(&my_num) {
                if row_sum == 0 {
                    row_sum += 1;
                } else {
                    row_sum *= 2;
                }
            }
        }
        sum += row_sum;
    }
    Ok(Solution::Int(sum as i32))
}

fn solve_p2_part_2(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut sum = 0;
    let mut look_forward_scratchcards: VecDeque<u32> = VecDeque::new();
    for card_row in lines {
        let card_row = card_row?;
        let (_, num_portion) = card_row.split_once(':').ok_or(anyhow::anyhow!(
            "AOC has invalid input!!?!??! Input: {card_row}"
        ))?;
        let (winning_nums, my_nums) = num_portion.split_once('|').ok_or(anyhow::anyhow!(
            "AOC has invalid input?!?!?!?!?! Input: {num_portion}"
        ))?;
        let winning_num_set = create_card_hashmap(winning_nums)?;
        let my_nums = create_winning_nums(&my_nums)?;
        let cards = 1 + look_forward_scratchcards.pop_front().unwrap_or_default();
        let mut total_wins = 0;
        for my_num in my_nums.iter() {
            if winning_num_set.contains(my_num) {
                total_wins += 1;
            }
        }
        let current_len = look_forward_scratchcards.len();
        for i in 0..std::cmp::min(total_wins as usize, current_len) {
            look_forward_scratchcards[i] += 1 * cards;
        }
        for _ in current_len..total_wins {
            look_forward_scratchcards.push_back(1 * cards);
        }
        sum += cards;
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
