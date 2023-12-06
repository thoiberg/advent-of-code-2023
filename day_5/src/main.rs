use std::{collections::HashMap, ops::Range, str::FromStr};

use strum_macros::EnumString;

fn main() {
    println!("Hello, world!");

    let text = read_input();
    let almanac = process_input(&text);
    // write helper function when given a value will respond with the equiv
    //      if in source range, find offset from start and apply offset to dest
    //      if not then return same value
    // write high level function to map from seed -> soil -> fert -> water -> light -> temp -> humidity -> location
    let part_one_answer = part_one_solution(&almanac);
}

fn part_one_solution(almanac: &Almanac) -> u32 {
    todo!()
}

fn read_input() -> String {
    String::from(include_str!("../data/puzzle_input.txt"))
}

fn process_input(text: &str) -> Almanac {
    let parts: Vec<&str> = text.split("\n\n").collect();
    // seeds
    let seeds: Vec<u32> = parts[0].split(' ').collect::<Vec<&str>>()[1..]
        .iter()
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect();

    // maps
    let maps = parts[1..]
        .iter()
        .map(|map_data| {
            let map_lines: Vec<&str> = map_data.split('\n').collect();
            let map_name = map_lines[0].split(' ').collect::<Vec<&str>>()[0];

            let map_ranges: Vec<MapRange> = map_lines[1..]
                .iter()
                .map(|range_line| {
                    let range_parts: Vec<u32> = range_line
                        .split(' ')
                        .map(|range_part| range_part.parse::<u32>().unwrap())
                        .collect();
                    MapRange::new(range_parts[0], range_parts[1], range_parts[2])
                })
                .collect();

            (
                MapType::from_str(map_name).unwrap(),
                Map { ranges: map_ranges },
            )
        })
        .collect();

    Almanac { seeds, maps }
}

#[derive(EnumString, PartialEq, Eq, Hash)]
enum MapType {
    #[strum(serialize = "seed-to-soil")]
    SeedtoSoil,
    #[strum(serialize = "soil-to-fertilizer")]
    SoiltoFertilizer,
    #[strum(serialize = "fertilizer-to-water")]
    FertilizertoWater,
    #[strum(serialize = "water-to-light")]
    WatertoLight,
    #[strum(serialize = "light-to-temperature")]
    LighttoTemperature,
    #[strum(serialize = "temperature-to-humidity")]
    TemperaturetoHumidity,
    #[strum(serialize = "humidity-to-location")]
    HumiditytoLocation,
}
struct Almanac {
    seeds: Vec<u32>,
    maps: HashMap<MapType, Map>,
}

struct Map {
    ranges: Vec<MapRange>,
}

struct MapRange {
    dest_range_start: u32,
    source_range_start: u32,
    range_length: u32,
    destination_range: Range<u32>,
    source_range: Range<u32>,
}

impl MapRange {
    fn new(dest: u32, source: u32, length: u32) -> Self {
        let destination_range = dest..(dest + length);
        let source_range = source..(source + length);

        Self {
            dest_range_start: dest,
            source_range_start: source,
            range_length: length,
            destination_range,
            source_range,
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Almanac {
        let input = String::from(include_str!("../data/test_input.txt"));
        process_input(&input)
    }

    #[test]
    fn test_process_input() {
        let almanac = test_data();

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);

        assert_eq!(almanac.maps.len(), 7);
        assert!(almanac.maps.get(&MapType::SeedtoSoil).is_some());
        assert!(almanac.maps.get(&MapType::SoiltoFertilizer).is_some());
        assert!(almanac.maps.get(&MapType::FertilizertoWater).is_some());
        assert!(almanac.maps.get(&MapType::WatertoLight).is_some());
        assert!(almanac.maps.get(&MapType::LighttoTemperature).is_some());
        assert!(almanac.maps.get(&MapType::TemperaturetoHumidity).is_some());
        assert!(almanac.maps.get(&MapType::HumiditytoLocation).is_some());

        let soil_map = almanac.maps.get(&MapType::SeedtoSoil).unwrap();
        assert_eq!(soil_map.ranges[0].dest_range_start, 50);
        assert_eq!(soil_map.ranges[0].source_range_start, 98);
        assert_eq!(soil_map.ranges[0].range_length, 2);
        assert_eq!(soil_map.ranges[0].destination_range, (50..52));
        assert_eq!(soil_map.ranges[0].source_range, (98..100));
    }
}
