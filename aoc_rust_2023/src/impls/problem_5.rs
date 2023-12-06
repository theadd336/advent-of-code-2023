use crate::{AocResult, Part, Solution};

#[derive(Debug, Clone, Copy)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl TryFrom<&str> for MapType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed-to-soil" => Ok(Self::SeedToSoil),
            "soil-to-fertilizer" => Ok(Self::SoilToFertilizer),
            "fertilizer-to-water" => Ok(Self::FertilizerToWater),
            "water-to-light" => Ok(Self::WaterToLight),
            "light-to-temperature" => Ok(Self::LightToTemperature),
            "temperature-to-humidity" => Ok(Self::TemperatureToHumidity),
            "humidity-to-location" => Ok(Self::HumidityToLocation),
            _ => Err(anyhow::anyhow!("Value={value} is not a valid map type")),
        }
    }
}

#[derive(Debug)]
struct Range {
    origin_start: u64,
    origin_end: u64,
    offset: i64,
}

impl TryFrom<&str> for Range {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut num_str = String::new();
        let mut nums = [0; 3];
        let mut num_idx = 0;
        for c in value.chars() {
            if c.is_ascii_digit() {
                num_str.push(c);
            } else {
                if num_idx >= nums.len() {
                    anyhow::bail!("Value={value} has more than three nums");
                }
                let num = num_str.parse()?;
                nums[num_idx] = num;
                num_idx += 1;
                num_str = String::new();
            }
        }
        let num = num_str.parse()?;
        nums[num_idx] = num;
        num_idx += 1;
        if num_idx != nums.len() {
            anyhow::bail!("Value={value} has fewer than three nums");
        }
        Ok(Self {
            origin_start: nums[1],
            origin_end: nums[1] + nums[2],
            offset: nums[0] as i64 - nums[1] as i64,
        })
    }
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.origin_start && value < self.origin_end
    }

    fn map_from(&self, value: u64) -> u64 {
        value.saturating_add_signed(self.offset)
    }
}

#[derive(Debug)]
struct ALotOfMaps {
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

impl ALotOfMaps {
    fn new() -> Self {
        Self {
            seed_to_soil: vec![],
            soil_to_fertilizer: vec![],
            fertilizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temperature: vec![],
            temperature_to_humidity: vec![],
            humidity_to_location: vec![],
        }
    }

    fn add_range(&mut self, range: Range, map_type: MapType) {
        let map_pointer = match map_type {
            MapType::SeedToSoil => &mut self.seed_to_soil,
            MapType::SoilToFertilizer => &mut self.soil_to_fertilizer,
            MapType::FertilizerToWater => &mut self.fertilizer_to_water,
            MapType::WaterToLight => &mut self.water_to_light,
            MapType::LightToTemperature => &mut self.light_to_temperature,
            MapType::TemperatureToHumidity => &mut self.temperature_to_humidity,
            MapType::HumidityToLocation => &mut self.humidity_to_location,
        };
        map_pointer.push(range);
    }

    fn sort_ranges(&mut self) {
        self.seed_to_soil
            .sort_by(|r1, r2| r1.origin_start.cmp(&r2.origin_start));
        self.soil_to_fertilizer
            .sort_by(|r1, r2| r1.origin_start.cmp(&r2.origin_start));
        self.fertilizer_to_water
            .sort_by(|r1, r2| r1.origin_start.cmp(&r2.origin_start));
        self.water_to_light
            .sort_by(|r1, r2| r1.origin_start.cmp(&r2.origin_start));
        self.light_to_temperature
            .sort_by(|r1, r2| r1.origin_start.cmp(&r2.origin_start));
        self.temperature_to_humidity
            .sort_by(|r1, r2| r1.origin_start.cmp(&r2.origin_start));
        self.humidity_to_location
            .sort_by(|r1, r2| r1.origin_start.cmp(&r2.origin_start));
    }

    fn calculate_next_value(ranges: &[Range], value: u64) -> u64 {
        match ranges.binary_search_by_key(&value, |r| r.origin_start) {
            // Edge case, we are right at the start of a range
            Ok(idx) => ranges[idx]
                .origin_start
                .saturating_add_signed(ranges[idx].offset),
            Err(idx) => {
                if idx == 0 {
                    return value;
                }
                let range = &ranges[idx - 1];
                if range.contains(value) {
                    return range.map_from(value);
                }
                return value;
            }
        }
    }

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        // println!("Evaluating seed {seed}");
        let soil = Self::calculate_next_value(&self.seed_to_soil, seed);
        // println!("seed: {seed}, soil: {soil}");
        let fertilizer = Self::calculate_next_value(&self.soil_to_fertilizer, soil);
        // println!("soil: {soil}, fertilizer: {fertilizer}");
        let water = Self::calculate_next_value(&self.fertilizer_to_water, fertilizer);
        // println!("fertilizer: {fertilizer}, water: {water}");
        let light = Self::calculate_next_value(&self.water_to_light, water);
        // println!("water: {water}, light: {light}");
        let temp = Self::calculate_next_value(&self.light_to_temperature, light);
        // println!("light: {light}, temp: {temp}");
        let humidity = Self::calculate_next_value(&self.temperature_to_humidity, temp);
        // println!("temp: {temp}, humidity: {humidity}");
        let location = Self::calculate_next_value(&self.humidity_to_location, humidity);
        // println!("humidity: {humidity}, location: {location}");
        location
    }
}

fn create_seed_list(seed_line: &str, seeds_as_ranges: bool) -> AocResult<Vec<u64>> {
    let mut seeds = Vec::new();
    let mut scratch_seed: Option<u64> = None;
    for num_str in seed_line.split(' ') {
        if num_str == "seeds:" {
            continue;
        }
        let num = num_str.parse()?;
        if !seeds_as_ranges {
            seeds.push(num);
        } else {
            match scratch_seed.take() {
                None => scratch_seed = Some(num),
                Some(start_seed) => {
                    for seed in start_seed..start_seed + num {
                        seeds.push(seed);
                    }
                }
            }
        }
    }
    Ok(seeds)
}

fn populate_seeds_and_maps(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
    seeds_as_ranges: bool,
) -> AocResult<(Vec<u64>, ALotOfMaps)> {
    let mut seeds = Vec::new();
    let mut map_type = MapType::FertilizerToWater;
    let mut lots_o_maps = ALotOfMaps::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        if line.starts_with("seeds") {
            seeds = create_seed_list(&line, seeds_as_ranges)?;
            continue;
        }
        let first_char = line.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            let range = Range::try_from(line.as_str())?;
            lots_o_maps.add_range(range, map_type);
        } else {
            let (map_type_str, _) = line.split_once(' ').unwrap();
            map_type = MapType::try_from(map_type_str)?;
        }
    }
    lots_o_maps.sort_ranges();
    Ok((seeds, lots_o_maps))
}

fn solve_p1_part_1(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut lowest_location: u64 = u64::MAX;
    let (seeds, lots_o_maps) = populate_seeds_and_maps(lines, false)?;
    for seed in seeds {
        let location = lots_o_maps.map_seed_to_location(seed);
        // .ok_or(anyhow::anyhow!("Seed={seed} has no location"))?;
        lowest_location = std::cmp::min(lowest_location, location);
    }
    Ok(Solution::Int(lowest_location as i32))
}

fn solve_p2_part_2(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> AocResult<Solution> {
    let mut lowest_location: u64 = u64::MAX;
    let (seeds, lots_o_maps) = populate_seeds_and_maps(lines, true)?;
    for seed in seeds {
        let location = lots_o_maps.map_seed_to_location(seed);
        lowest_location = std::cmp::min(lowest_location, location);
    }
    println!("Actual solution in case it overflows: {lowest_location}");
    Ok(Solution::Int(lowest_location as i32))
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
