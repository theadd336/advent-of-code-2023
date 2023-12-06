use crate::{AocResult, Part, Solution};

struct RaceParams {
    time: u64,
    distance: u64,
}

impl RaceParams {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn num_ways_to_solve(&self) -> u64 {
        let mut speed = 0;
        let mut time = self.time;
        let mut num_ways_to_solve = 0;
        while time > 0 {
            let travelled_distance = speed * time;
            if travelled_distance > self.distance {
                num_ways_to_solve += 1;
            } else if num_ways_to_solve > 0 {
                break;
            }
            speed += 1;
            time -= 1;
        }
        num_ways_to_solve
    }
}

fn line_to_num<'a>(line: impl Iterator<Item = &'a str>) -> u64 {
    let mut full_num = String::new();
    for piece in line {
        if !piece.chars().next().unwrap().is_ascii_digit() {
            continue;
        }
        full_num.push_str(piece);
    }
    full_num.parse().unwrap()
}

fn lines_to_race_params(
    mut lines: impl Iterator<Item = Result<String, std::io::Error>>,
    squash_to_one_race: bool,
) -> AocResult<Vec<RaceParams>> {
    let mut race_params: Vec<RaceParams> = vec![];
    let first_line = lines
        .next()
        .ok_or(anyhow::anyhow!("No lines present in the input"))??;
    let second_line = lines
        .next()
        .ok_or(anyhow::anyhow!("Second line not present in input"))??;

    let time_pieces = first_line.split(' ').filter(|piece| !piece.is_empty());
    let distance_pieces = second_line.split(' ').filter(|piece| !piece.is_empty());
    if !squash_to_one_race {
        for (time_piece, distance_piece) in time_pieces.zip(distance_pieces) {
            if time_piece == "Time:" {
                continue;
            }
            let time = time_piece.parse()?;
            let distance = distance_piece.parse()?;
            let race_param = RaceParams::new(time, distance);
            race_params.push(race_param);
        }
    } else {
        let time = line_to_num(time_pieces);
        let distance = line_to_num(distance_pieces);
        race_params.push(RaceParams { time, distance });
    }

    Ok(race_params)
}

fn solve_p1_part_1(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut solution = 1;
    let race_params = lines_to_race_params(lines, false)?;
    for race in race_params {
        solution *= race.num_ways_to_solve();
    }
    Ok(Solution::Int(solution as i32))
}

fn solve_p2_part_2(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut solution = 1;
    let race_params = lines_to_race_params(lines, true)?;
    for race in race_params {
        solution *= race.num_ways_to_solve();
    }
    Ok(Solution::Int(solution as i32))
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

#[cfg(test)]
mod tests {
    use super::RaceParams;

    #[test]
    fn test_race_params_num_ways_to_solve() {
        let race_params = vec![
            RaceParams::new(7, 9),
            RaceParams::new(15, 40),
            RaceParams::new(30, 200),
        ];
        for (race, expected) in race_params.into_iter().zip([4, 8, 9]) {
            let ways_to_solve = race.num_ways_to_solve();
            assert_eq!(ways_to_solve, expected);
        }
    }
}
