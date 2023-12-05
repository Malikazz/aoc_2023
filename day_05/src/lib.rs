use std::{collections::HashMap, fs::read_to_string, ops::Range, thread::sleep, time::Duration};

#[derive(Debug)]
pub enum Direction {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumi,
    HumiToLoc,
}

#[derive(Debug)]
pub struct SourceDestination {
    pub source_range: Vec<Range<usize>>,
    pub destination_range: Vec<Range<usize>>,
    pub direction: Direction,
}
#[derive(Debug)]
pub struct FinishedMap {
    pub seed: usize,
    pub soil: usize,
    pub fetilizier: usize,
    pub water: usize,
    pub light: usize,
    pub temperature: usize,
    pub humidity: usize,
    pub location: usize,
}

pub fn solve(input: Vec<String>) -> usize {
    let mut seeds: Vec<FinishedMap> = parse_seeds(&input);
    let source_maps: Vec<SourceDestination> = parse_source_maps(input);

    for seed in seeds.iter_mut() {
        // vec should be in order
        for source_destination in source_maps.iter() {
            match source_destination.direction {
                Direction::SeedToSoil => {
                    for (index, source) in source_destination.source_range.iter().enumerate() {
                        if source.contains(&seed.seed) {
                            seed.soil = source_destination.destination_range[index].start
                                + (&seed.seed - source.start);
                            break;
                        } else {
                            seed.soil = seed.seed;
                        }
                    }
                }
                Direction::SoilToFert => {
                    for (index, source) in source_destination.source_range.iter().enumerate() {
                        if source.contains(&seed.soil) {
                            seed.fetilizier = source_destination.destination_range[index].start
                                + (&seed.soil - source.start);
                            break;
                        } else {
                            seed.fetilizier = seed.soil;
                        }
                    }
                }
                Direction::FertToWater => {
                    for (index, source) in source_destination.source_range.iter().enumerate() {
                        if source.contains(&seed.fetilizier) {
                            seed.water = source_destination.destination_range[index].start
                                + (&seed.fetilizier - source.start);
                            break;
                        } else {
                            seed.water = seed.fetilizier;
                        }
                    }
                }
                Direction::WaterToLight => {
                    for (index, source) in source_destination.source_range.iter().enumerate() {
                        if source.contains(&seed.water) {
                            seed.light = source_destination.destination_range[index].start
                                + (&seed.water - source.start);
                            break;
                        } else {
                            seed.light = seed.water;
                        }
                    }
                }
                Direction::LightToTemp => {
                    for (index, source) in source_destination.source_range.iter().enumerate() {
                        if source.contains(&seed.light) {
                            seed.temperature = source_destination.destination_range[index].start
                                + (&seed.light - source.start);
                            break;
                        } else {
                            seed.temperature = seed.light;
                        }
                    }
                }
                Direction::TempToHumi => {
                    for (index, source) in source_destination.source_range.iter().enumerate() {
                        if source.contains(&seed.temperature) {
                            seed.humidity = source_destination.destination_range[index].start
                                + (&seed.temperature - source.start);
                            break;
                        } else {
                            seed.humidity = seed.temperature;
                        }
                    }
                }
                Direction::HumiToLoc => {
                    for (index, source) in source_destination.source_range.iter().enumerate() {
                        if source.contains(&seed.humidity) {
                            seed.location = source_destination.destination_range[index].start
                                + (&seed.humidity - source.start);
                            break;
                        } else {
                            seed.location = seed.humidity;
                        }
                    }
                }
            }
        }
    }
    *seeds
        .iter()
        .map(|e| e.location)
        .collect::<Vec<usize>>()
        .iter_mut()
        .min()
        .unwrap()
}

pub fn get_direction(input: &str) -> Direction {
    match input {
        "seed-to-soil map:" => return Direction::SeedToSoil,
        "soil-to-fertilizer map:" => return Direction::SoilToFert,
        "fertilizer-to-water map:" => return Direction::FertToWater,
        "water-to-light map:" => return Direction::WaterToLight,
        "light-to-temperature map:" => return Direction::LightToTemp,
        "temperature-to-humidity map:" => return Direction::TempToHumi,
        "humidity-to-location map:" => return Direction::HumiToLoc,
        _ => panic!("Getting direction failed"),
    }
}

pub fn parse_source_maps(input: Vec<String>) -> Vec<SourceDestination> {
    let mut source_maps: Vec<SourceDestination> = Vec::new();
    let mut temp_source_map: SourceDestination = SourceDestination {
        source_range: Vec::new(),
        destination_range: Vec::new(),
        direction: Direction::SeedToSoil,
    };

    for line in input[1..].iter() {
        if line == "" {
            continue;
        }
        if line.contains("to") {
            if temp_source_map.source_range.len() > 0 {
                source_maps.push(temp_source_map);
            }
            // start new source map
            temp_source_map = SourceDestination {
                source_range: Vec::new(),
                destination_range: Vec::new(),
                direction: get_direction(line as &str),
            };
            continue;
        } else {
            let numbers = line.split(" ").collect::<Vec<&str>>();
            let source = numbers[1].parse::<usize>().unwrap();
            let destination = numbers[0].parse::<usize>().unwrap();
            let index_offset = numbers[2].parse::<usize>().unwrap();
            temp_source_map
                .source_range
                .push(source..source + index_offset);
            temp_source_map
                .destination_range
                .push(destination..destination + index_offset);
        }
    }
    source_maps.push(temp_source_map);
    source_maps
}

pub fn parse_seeds(input: &Vec<String>) -> Vec<FinishedMap> {
    let mut seeds: Vec<FinishedMap> = Vec::new();
    // seeds:
    let input = input[0].replace("seeds: ", "");
    let seed_range = input.split(" ").collect::<Vec<&str>>();

    for item in seed_range.windows(2).step_by(2) {
        println!("Making seeds from{:?} to {:?}", item[0], item[1]);
        // soooooooo fuck
        let start = item[0].parse::<usize>().unwrap();
        let offset = item[1].parse::<usize>().unwrap() + start;
        for number in start..offset {
            seeds.push(FinishedMap {
                seed: number,
                soil: 0,
                fetilizier: 0,
                temperature: 0,
                humidity: 0,
                water: 0,
                location: 0,
                light: 0,
            });
        }
    }
    seeds
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05_it_works() {
        let result = solve(read_lines("src/test_input"));
        assert_eq!(result, 46);
    }
}
