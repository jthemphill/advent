enum ParsePhase {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    src: usize,
    dest: usize,
    len: usize,
}

impl Range {
    fn convert(&self, src_value: usize) -> Option<usize> {
        if src_value < self.src || src_value - self.src >= self.len {
            None
        } else {
            Some(self.dest + (src_value - self.src))
        }
    }
}

fn main() {
    let mut phase = ParsePhase::SeedToSoil;

    let mut seeds = vec![];
    let mut seed_to_soil = vec![];
    let mut soil_to_fertilizer = vec![];
    let mut fertilizer_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temperature = vec![];
    let mut temperature_to_humidity = vec![];
    let mut humidity_to_location = vec![];

    for line in std::io::stdin().lines() {
        let line = line.unwrap().trim().to_owned();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds: ") {
            let seed_str = line.split("seeds: ").nth(1).unwrap();
            for seed in seed_str.split_ascii_whitespace() {
                seeds.push(seed.parse::<usize>().unwrap());
            }
            continue;
        }

        if let Some(new_phase) = match line.as_str() {
            "seed-to-soil map:" => Some(ParsePhase::SeedToSoil),
            "soil-to-fertilizer map:" => Some(ParsePhase::SoilToFertilizer),
            "fertilizer-to-water map:" => Some(ParsePhase::FertilizerToWater),
            "water-to-light map:" => Some(ParsePhase::WaterToLight),
            "light-to-temperature map:" => Some(ParsePhase::LightToTemperature),
            "temperature-to-humidity map:" => Some(ParsePhase::TemperatureToHumidity),
            "humidity-to-location map:" => Some(ParsePhase::HumidityToLocation),
            _ => None,
        } {
            phase = new_phase;
            continue;
        }

        let mut nums = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap());
        let dest = nums.next().unwrap();
        let src = nums.next().unwrap();
        let len = nums.next().unwrap();
        let range = Range { src, dest, len };
        match phase {
            ParsePhase::SeedToSoil => seed_to_soil.push(range),
            ParsePhase::SoilToFertilizer => soil_to_fertilizer.push(range),
            ParsePhase::FertilizerToWater => fertilizer_to_water.push(range),
            ParsePhase::WaterToLight => water_to_light.push(range),
            ParsePhase::LightToTemperature => light_to_temperature.push(range),
            ParsePhase::TemperatureToHumidity => temperature_to_humidity.push(range),
            ParsePhase::HumidityToLocation => humidity_to_location.push(range),
        };
    }

    seed_to_soil.sort();
    soil_to_fertilizer.sort();
    fertilizer_to_water.sort();
    water_to_light.sort();
    light_to_temperature.sort();
    temperature_to_humidity.sort();
    humidity_to_location.sort();

    let mut soils = vec![];

    let get_locations = |seeds: Vec<usize>| -> Vec<usize> {
        macro_rules! lookup {
            ($srcs: ident, $dsts: ident, $lookup: ident) => {
                let mut $dsts = vec![];
                for src in $srcs {
                    let dst = $lookup
                        .iter()
                        .flat_map(|range| range.convert(src))
                        .next()
                        .unwrap_or(src);
                    $dsts.push(dst);
                }
            };
        }

        lookup!(seeds, soils, seed_to_soil);
        lookup!(soils, fertilizers, soil_to_fertilizer);
        lookup!(fertilizers, waters, fertilizer_to_water);
        lookup!(waters, lights, water_to_light);
        lookup!(lights, temperatures, light_to_temperature);
        lookup!(temperatures, humidities, temperature_to_humidity);
        lookup!(humidities, locations, humidity_to_location);
        locations
    };

    let locations = get_locations(seeds);
    let min_location = locations.into_iter().min().unwrap();
    println!("Min location: {}", min_location);
}
