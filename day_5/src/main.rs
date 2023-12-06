use std::{collections::HashMap, ops::Range, str::FromStr};

use strum_macros::EnumString;

fn main() {
    let text = read_input();
    let almanac = process_input(&text);
    let part_one_answer = part_one_solution(&almanac);
    println!("Part One Answer is {part_one_answer}");
}

fn part_one_solution(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|seed| location_for_seed(almanac, seed))
        .min()
        .unwrap()
}

fn location_for_seed(almanac: &Almanac, seed: &u64) -> u64 {
    let soil_dest = almanac
        .maps
        .get(&MapType::SeedtoSoil)
        .unwrap()
        .destination_for(seed);
    let fertilizer_dest = almanac
        .maps
        .get(&MapType::SoiltoFertilizer)
        .unwrap()
        .destination_for(&soil_dest);
    let water_dest = almanac
        .maps
        .get(&MapType::FertilizertoWater)
        .unwrap()
        .destination_for(&fertilizer_dest);
    let light_dest = almanac
        .maps
        .get(&MapType::WatertoLight)
        .unwrap()
        .destination_for(&water_dest);
    let temperature_dest = almanac
        .maps
        .get(&MapType::LighttoTemperature)
        .unwrap()
        .destination_for(&light_dest);
    let humidity_dest = almanac
        .maps
        .get(&MapType::TemperaturetoHumidity)
        .unwrap()
        .destination_for(&temperature_dest);
    let location_dest = almanac
        .maps
        .get(&MapType::HumiditytoLocation)
        .unwrap()
        .destination_for(&humidity_dest);

    location_dest
}

fn read_input() -> String {
    String::from(include_str!("../data/puzzle_input.txt"))
}

fn process_input(text: &str) -> Almanac {
    let parts: Vec<&str> = text.split("\n\n").collect();
    // seeds
    let seeds: Vec<u64> = parts[0].split(' ').collect::<Vec<&str>>()[1..]
        .iter()
        .map(|seed| seed.parse::<u64>().unwrap())
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
                    let range_parts: Vec<u64> = range_line
                        .split(' ')
                        .map(|range_part| range_part.parse::<u64>().unwrap())
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
    seeds: Vec<u64>,
    maps: HashMap<MapType, Map>,
}

struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn destination_for(&self, source: &u64) -> u64 {
        let map_range = self
            .ranges
            .iter()
            .find(|map_range| map_range.source_range.contains(&source));

        match map_range {
            Some(map_range) => {
                let diff = source - map_range.source_range.start;
                map_range.destination_range.start + diff
            }
            None => *source, // if no mapping then its the same value in the destination
        }
    }
}

struct MapRange {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
    destination_range: Range<u64>,
    source_range: Range<u64>,
}

impl MapRange {
    fn new(dest: u64, source: u64, length: u64) -> Self {
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

    #[test]
    fn test_destination_for_in_range() {
        let almanac = test_data();
        let seed_to_soil_map = almanac.maps.get(&MapType::SeedtoSoil).unwrap();
        let soil_number = seed_to_soil_map.destination_for(&53);

        assert_eq!(soil_number, 55);
    }

    #[test]
    fn test_destination_for_out_of_range() {
        let almanac = test_data();
        let seed_to_soil_map = almanac.maps.get(&MapType::SeedtoSoil).unwrap();
        let soil_number = seed_to_soil_map.destination_for(&10);

        assert_eq!(soil_number, 10);
    }

    #[test]
    fn test_location_for_seed() {
        let almanac = test_data();

        assert_eq!(location_for_seed(&almanac, &79), 82);
        assert_eq!(location_for_seed(&almanac, &14), 43);
        assert_eq!(location_for_seed(&almanac, &55), 86);
        assert_eq!(location_for_seed(&almanac, &13), 35);
    }

    #[test]
    fn test_part_one_solution() {
        let almanac = test_data();
        let part_one_answer = part_one_solution(&almanac);

        assert_eq!(part_one_answer, 35);
    }

    #[test]
    fn test_part_one_real_answer() {
        let input = String::from(include_str!("../data/puzzle_input.txt"));
        let almanac = process_input(&input);

        let answer = part_one_solution(&almanac);

        assert_eq!(answer, 806029445);
    }
}
