use std::{
    cmp::min, collections::HashMap, fs::read_to_string, ops::Range, thread::{sleep, JoinHandle, self}, time::Duration, sync::Arc,
};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumi,
    HumiToLoc,
}

#[derive(Debug, Clone)]
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

pub fn get_lowest(seeds: &Range<usize>, source_maps: Arc<Vec<SourceDestination>>) -> usize {
    let mut current_value: usize;
    let mut min_value: usize = usize::MAX;

    for seed in seeds.start..seeds.end {
        // vec should be in order
        current_value = seed;
        for source_destination in source_maps.iter() {
            for (index, source) in source_destination.source_range.iter().enumerate() {
                if source.contains(&current_value) {
                    current_value =
                        source_destination.destination_range[index].start + (current_value - source.start);
                    break;
                } else {
                    current_value = current_value;
                }
            }
        }
        min_value = min(min_value, current_value);
    }
    min_value
}

pub fn solve_two(input: Vec<String>) -> usize {
    let seeds = Vec::leak(parse_seeds(&input[0]));
    let source_maps = Arc::new(parse_source_maps(input));
    let mut lowest: usize = usize::MAX;
    let mut handles: Vec<JoinHandle<usize>> = Vec::new();


    for seed in seeds.iter() {
        let source_clone = source_maps.clone();
        handles.push(thread::spawn(move || {
            println!("Starting need range {:?}", seed);
            get_lowest(seed, source_clone)
        }));
    }
    for handle in handles.into_iter(){
        lowest = min(handle.join().unwrap(), lowest);
    }
    lowest
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

pub fn parse_seeds(input: &String) -> Vec<Range<usize>> {
    let input = input.replace("seeds: ", "");
    let seed_range = input.split(" ").collect::<Vec<&str>>();
    let mut seed_ranges: Vec<Range<usize>> = Vec::new();
    
    for seed in seed_range.windows(2).step_by(2) {
        let start = seed[0].parse::<usize>().unwrap();
        let end = start + seed[1].parse::<usize>().unwrap();
        seed_ranges.push(start..end);
    }
    seed_ranges
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
        let result = solve_two(read_lines("src/test_input"));
        assert_eq!(result, 46);
    }
}
