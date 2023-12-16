

fn main() {
    let input: &'static str = include_str!("../input.txt");
    
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
}
