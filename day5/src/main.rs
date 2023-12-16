use std::ops::Range;

fn main() {
    part_1_main();
    part_2_main();
}

fn part_1_main() {
    let input: &'static str = include_str!("../input_part1.txt");
    
    let mut input: Vec<&str> = input.trim().split("\n\n").collect();

    let seeds: Vec<u64> = (&input.remove(0)[7..])
        .split(' ')
        .map(|substring| {substring.parse::<u64>().unwrap()})
        .collect();

    let input: Vec<Vec<&str>> = input
        .into_iter()
        .map(|paragraph| {
            paragraph.lines().skip(1).collect::<Vec<&str>>()
        })
        .collect();

    let hypermap = input
        .into_iter()
        .map(|paragraph| {
            paragraph
                .into_iter()
                .map(|line| { parse_tuple(parse_line(line)) })
                .collect::<Vec<Box<dyn Fn(u64) -> Option<u64>>>>()
        })
        .collect::<Vec<Vec<Box<dyn Fn(u64) -> Option<u64>>>>>();

    let mut locations: Vec<u64> = seeds
        .into_iter()
        .map(|mut seed| {
            for supermap in hypermap.iter() {
                'break_point:
                for map in supermap {
                    if let Some(new_seed) = map(seed) {
                        seed = new_seed;
                        break 'break_point;
                    }
                }
            }
            seed
        })
        .collect();

    locations.sort();

    println!("{}", locations[0]);
}

/// instead of brute forcing a incomprehensible amount of seeds, we will start from the bottom up until we reach a valid initial seed
fn part_2_main() {
    let input: &'static str = include_str!("../input_part2.txt");
    
    let mut input: Vec<&str> = input.trim().split("\n\n").collect();

    let seeds: Vec<u64> = (&input.remove(0)[7..])
        .split(' ')
        .map(|substring| {substring.parse::<u64>().unwrap()})
        .collect();

    let seeds: Vec<(u64, u64)> = seeds
        .iter()
        .zip(seeds.iter().skip(1))
        .step_by(2)
        .map(|(a, b)| {(*a, *b)})
        .collect();

    println!("{:?}", seeds);

    let seeds: Vec<Range<u64>> = seeds
        .into_iter()
        .map(|(range_start, length)| {range_start..range_start+length})
        .collect();

    let check_seed = |seed: &u64| -> bool {
        let mut output: bool = false;
        for range in seeds.iter() {
            if range.contains(seed) {
                output = true; 
                break;
            }
        }
        output
    };

    let input: Vec<Vec<&str>> = input
        .into_iter()
        .map(|paragraph| {
            paragraph.lines().skip(1).collect::<Vec<&str>>()
        })
        .collect();

    let hypermap = input
        .into_iter()
        .map(|paragraph| {
            paragraph
                .into_iter()
                .map(|line| { rev_parse_tuple(parse_line(line)) })
                .collect::<Vec<Box<dyn Fn(u64) -> Option<u64>>>>()
        })
        .collect::<Vec<Vec<Box<dyn Fn(u64) -> Option<u64>>>>>();

    for mut index in 1_u64.. {
        let location: u64 = index.clone();
        // we simply have to reverse the hypermap and use rev_parse_tuple to start from the bottom up
        for supermap in hypermap.iter().rev() {
            'break_point:
            for map in supermap {
                if let Some(new) = map(index) {
                    index = new;
                    break 'break_point;
                }
            }
        }
        if check_seed(&index) {
            println!("{} -> {}", index, location);
            break;
        }
    }
}

fn parse_line(input: &str) -> (u64, u64, u64) {
    let input: Vec<u64> = input
        .split(' ')
        .map(|substring| {substring.parse::<u64>().unwrap()})
        .collect();

    (input[0], input[1], input[2])
}

// (dest_range_start, sour_range_start, length)
fn parse_tuple(input: (u64, u64, u64)) -> Box<dyn Fn(u64) -> Option<u64>> {
    Box::new(move |value: u64| -> Option<u64> {
        if (input.1..input.1+input.2).contains(&value) {
            Some(input.0 + (value - input.1))
        } 
        else {
            None
        }
    })
}

// (dest_range_start, sour_range_start, length)
/// 'inverses' the result of parse_tuple, instead of getting an output if it exists in the
/// dest_range we instead get the output in the sour_range if again it exists
fn rev_parse_tuple(input: (u64, u64, u64)) -> Box<dyn Fn(u64) -> Option<u64>> {
    Box::new(move |value: u64| -> Option<u64> {
        if (input.0..input.0+input.2).contains(&value) {
            Some(input.1 + (value - input.0))
        } 
        else {
            None
        }
    })
}


#[cfg(test)]
pub mod tests {
    #[test]
    fn parse_line() {
        assert_eq!(
            crate::parse_line("1 23 456"),
            (1_u64, 23_u64, 456_u64)
        )
    }
    #[test]
    pub fn test() {
        use crate::{
            parse_line,
            parse_tuple,
        };
        
        let seeds: &'static str = "79 14 55 13";
        let seeds: Vec<u64> = seeds
            .split(' ')
            .map(|substring| {substring.parse::<u64>().unwrap()})
            .collect();
        
        let seed_to_soil_map: &'static str = "50 98 2\n52 50 48";
        let seed_to_soil_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = seed_to_soil_map
            .lines()
            .map(|line| {parse_tuple(parse_line(line))})
            .collect();

        let soil_to_fertilizer_map: &'static str = "0 15 37\n37 52 2\n39 0 15";
        let soil_to_fertilizer_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = soil_to_fertilizer_map
            .lines()
            .map(|line| {parse_tuple(parse_line(line))})
            .collect();

        let ferilizer_to_water_map: &'static str = "49 53 8\n0 11 42\n42 0 7\n57 7 4";
        let ferilizer_to_water_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = ferilizer_to_water_map
            .lines()
            .map(|line| {parse_tuple(parse_line(line))})
            .collect();

        let water_to_light_map: &'static str = "88 18 7\n18 25 70";
        let water_to_light_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = water_to_light_map
            .lines()
            .map(|line| {parse_tuple(parse_line(line))})
            .collect();

        let light_to_temperature_map: &'static str = "45 77 23\n81 45 19\n68 64 13";
        let light_to_temperature_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = light_to_temperature_map
            .lines()
            .map(|line| {parse_tuple(parse_line(line))})
            .collect();

        let temperature_to_humidity_map: &'static str = "0 69 1\n1 0 69";
        let temperature_to_humidity_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = temperature_to_humidity_map
            .lines()
            .map(|line| {parse_tuple(parse_line(line))})
            .collect();

        let humidity_to_location_map: &'static str = "60 56 37\n56 93 4";
        let humidity_to_location_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = humidity_to_location_map
            .lines()
            .map(|line| {parse_tuple(parse_line(line))})
            .collect();

        let seed_to_location_hypermap: Vec<Vec<Box<dyn Fn(u64) -> Option<u64>>>> = vec![
            seed_to_soil_map,
            soil_to_fertilizer_map,
            ferilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        ];

        let mut output: Vec<u64> = Vec::new();

        for mut seed in seeds.into_iter() {
            print!("{}", seed);
            for supermap in seed_to_location_hypermap.iter() {
                'break_point :
                for map in supermap {
                    if let Some(new_seed) = map(seed) {
                        seed = new_seed;
                        
                        break 'break_point;
                    }
                }
                print!(" -> {}", seed);
            }
            println!("");
            output.push(seed);
        }

        assert_eq!(
            output,
            vec![82_u64, 43_u64, 86_u64, 35_u64]
        )
    
    }
    #[test]
    pub fn test_alt() {
        use crate::{
            parse_line,
            rev_parse_tuple,
        };
        
        let seeds: &'static str = "79 14 55 13";
        let seeds: Vec<u64> = seeds
            .split(' ')
            .map(|substring| {substring.parse::<u64>().unwrap()})
            .collect();
        
        let seed_to_soil_map: &'static str = "50 98 2\n52 50 48";
        let seed_to_soil_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = seed_to_soil_map
            .lines()
            .map(|line| {rev_parse_tuple(parse_line(line))})
            .collect();

        let soil_to_fertilizer_map: &'static str = "0 15 37\n37 52 2\n39 0 15";
        let soil_to_fertilizer_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = soil_to_fertilizer_map
            .lines()
            .map(|line| {rev_parse_tuple(parse_line(line))})
            .collect();

        let ferilizer_to_water_map: &'static str = "49 53 8\n0 11 42\n42 0 7\n57 7 4";
        let ferilizer_to_water_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = ferilizer_to_water_map
            .lines()
            .map(|line| {rev_parse_tuple(parse_line(line))})
            .collect();

        let water_to_light_map: &'static str = "88 18 7\n18 25 70";
        let water_to_light_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = water_to_light_map
            .lines()
            .map(|line| {rev_parse_tuple(parse_line(line))})
            .collect();

        let light_to_temperature_map: &'static str = "45 77 23\n81 45 19\n68 64 13";
        let light_to_temperature_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = light_to_temperature_map
            .lines()
            .map(|line| {rev_parse_tuple(parse_line(line))})
            .collect();

        let temperature_to_humidity_map: &'static str = "0 69 1\n1 0 69";
        let temperature_to_humidity_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = temperature_to_humidity_map
            .lines()
            .map(|line| {rev_parse_tuple(parse_line(line))})
            .collect();

        let humidity_to_location_map: &'static str = "60 56 37\n56 93 4";
        let humidity_to_location_map: Vec<Box<dyn Fn(u64) -> Option<u64>>> = humidity_to_location_map
            .lines()
            .map(|line| {rev_parse_tuple(parse_line(line))})
            .collect();

        let location_to_seed_hypermap: Vec<Vec<Box<dyn Fn(u64) -> Option<u64>>>> = vec![
            humidity_to_location_map,
            temperature_to_humidity_map,
            light_to_temperature_map,
            water_to_light_map,
            ferilizer_to_water_map,
            soil_to_fertilizer_map,
            seed_to_soil_map,
        ];

        let mut output: Vec<u64> = Vec::new();

        for mut index in 0..10000 {
            let early_copy: u64 = index.clone();
            for supermap in location_to_seed_hypermap.iter() {
                'break_point :
                for map in supermap {
                    if let Some(new) = map(index) {
                        index = new;
                        break 'break_point;
                    }
                }
            }
            if seeds.contains(&index) {output.push(early_copy)}
        }

        assert_eq!(
            output,
            vec![35, 43, 82, 86]
        )
    }
}
