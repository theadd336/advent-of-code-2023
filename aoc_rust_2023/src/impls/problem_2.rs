use crate::{AocResult, Part, Solution};

#[derive(Debug)]
struct CubeResults {
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeResults {
    fn try_from_str(game_substr: &str) -> AocResult<Self> {
        let mut parser = CubeResults {
            red: 0,
            blue: 0,
            green: 0,
        };

        for game_result in game_substr.split(';') {
            parser.handle_color_sections(game_result)?;
        }

        Ok(parser)
    }

    fn handle_color_sections(&mut self, color_sections: &str) -> AocResult<()> {
        for each_color in color_sections.split(',') {
            let mut expect_color = false;
            let mut color_to_increment = &mut self.blue;
            let mut color_value = 0;
            for piece in each_color.split(' ') {
                if piece.is_empty() {
                    continue;
                }
                if expect_color {
                    match piece {
                        "blue" => color_to_increment = &mut self.blue,
                        "green" => color_to_increment = &mut self.green,
                        "red" => color_to_increment = &mut self.red,
                        _ => anyhow::bail!("Expected a color, but piece: {piece} is not a color"),
                    }
                    *color_to_increment = std::cmp::max(color_value, *color_to_increment);
                } else {
                    match piece.parse::<u32>() {
                        Ok(val) => {
                            color_value = val;
                            expect_color = true;
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
        Ok(())
    }

    fn is_valid_for_game(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }

    fn cube_power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

fn parse_game_id(game_id_substr: &str) -> AocResult<u32> {
    let game_id = game_id_substr
        .split(' ')
        .last()
        .ok_or(anyhow::anyhow!("Game ID Substr: {game_id_substr} is empty"))?;
    let game_id = game_id.parse()?;
    Ok(game_id)
}

fn solve_p1_part_1(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut sum = 0;
    for line in lines {
        let line = line?;
        let (game_id_str, game_substr) = line
            .split_once(':')
            .ok_or(anyhow::anyhow!("No ':' found in game input: {line}"))?;
        let game_id = parse_game_id(game_id_str)?;
        let cube_values = CubeResults::try_from_str(game_substr)?;
        println!("Game ID: {game_id}, cube results: {cube_values:?}");
        if cube_values.is_valid_for_game(12, 13, 14) {
            sum += game_id;
        }
    }
    Ok(Solution::Int(sum as i32))
}

fn solve_p2_part_2(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut sum = 0;
    for line in lines {
        let line = line?;
        let (game_id_str, game_substr) = line
            .split_once(':')
            .ok_or(anyhow::anyhow!("No ':' found in game input: {line}"))?;
        let game_id = parse_game_id(game_id_str)?;
        let cube_values = CubeResults::try_from_str(game_substr)?;
        println!("Game ID: {game_id}, cube results: {cube_values:?}");
        sum += cube_values.cube_power();
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
