enum ParsePhase {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ResourceRange {
    src: usize,
    len: usize,
    reason: &'static str,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ConversionRange {
    src: usize,
    dest: usize,
    len: usize,
}

impl ConversionRange {
    fn convert(&self, src_value: usize) -> Option<usize> {
        if src_value < self.src || src_value - self.src >= self.len {
            None
        } else {
            Some(self.dest + (src_value - self.src))
        }
    }

    //
    //      self.dest|________|
    //
    //    other_range.src|________________|
    fn overlap(&self, other_range: &ConversionRange) -> Option<ConversionRange> {
        let my_start = self.dest;
        let my_end = self.dest + self.len;
        let their_start = other_range.src;
        let their_end = other_range.src + other_range.len;
        if my_start <= their_end && my_end >= their_start {
            if my_start >= their_start {
                let offset = my_start - their_start;
                Some(ConversionRange {
                    dest: other_range.dest + offset,
                    src: self.src,
                    len: self.len.min(other_range.len - offset),
                })
            } else {
                let offset = their_start - my_start;
                Some(ConversionRange {
                    dest: other_range.dest,
                    src: self.src + offset,
                    len: other_range.len.min(self.len - offset),
                })
            }
        } else {
            None
        }
    }
}

fn main() {
    let mut phase = ParsePhase::SeedToSoil;

    let mut seed_ranges = vec![];
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
            let mut seeds = vec![];
            let seed_str = line.split("seeds: ").nth(1).unwrap();
            for seed in seed_str.split_ascii_whitespace() {
                seeds.push(seed.parse::<usize>().unwrap());
            }
            for i in (0..seeds.len()).step_by(2) {
                seed_ranges.push(ResourceRange {
                    src: seeds[i],
                    len: seeds[i + 1],
                    reason: "seed",
                });
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
        let range = ConversionRange { dest, src, len };
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

    fn convert_range(
        source_range: &Vec<ResourceRange>,
        conversions: &Vec<ConversionRange>,
    ) -> Vec<ResourceRange> {
        let mut converted = vec![];
        for mut resource_range in source_range.iter().cloned() {
            for &transform_range in conversions {
                if transform_range.src + transform_range.len <= resource_range.src {
                    // The ranges don't overlap yet
                    continue;
                }
                if resource_range.src + resource_range.len <= transform_range.src {
                    // The ranges don't overlap.
                    // Transform ranges are sorted, so we won't see any overlap again.
                    break;
                }

                if resource_range.src < transform_range.src {
                    let offset = transform_range.src - resource_range.src;
                    assert!(offset < resource_range.len);

                    // Any source numbers that aren't mapped correspond to the same destination number.
                    converted.push(ResourceRange {
                        src: resource_range.src,
                        len: offset,
                        reason: "left no mapping",
                    });

                    resource_range = ResourceRange {
                        src: resource_range.src + offset,
                        len: resource_range.len - offset,
                        reason: resource_range.reason,
                    };
                }

                if transform_range.src <= resource_range.src {
                    let offset = resource_range.src - transform_range.src;
                    assert!(offset < transform_range.len);

                    let converted_len = (transform_range.len - offset).min(resource_range.len);

                    let converted_range = ResourceRange {
                        src: transform_range.dest + offset,
                        len: converted_len,
                        reason: "Conversion",
                    };
                    converted.push(converted_range);

                    resource_range = ResourceRange {
                        src: resource_range.src + converted_len,
                        len: resource_range.len - converted_len,
                        reason: resource_range.reason,
                    }
                }
            }

            // Any source numbers that aren't mapped correspond to the same destination number.
            if resource_range.len > 0 {
                assert_ne!(resource_range.src, 0);
                converted.push(ResourceRange {
                    src: resource_range.src,
                    len: resource_range.len,
                    reason: "right no mapping",
                });
            }
        }
        converted
    }

    println!("soils");
    let soils = convert_range(&seed_ranges, &seed_to_soil);
    println!("fertilizers");
    let fertilizers = convert_range(&soils, &soil_to_fertilizer);
    println!("waters");
    let waters = convert_range(&fertilizers, &fertilizer_to_water);
    println!("lights");
    let lights = convert_range(&waters, &water_to_light);
    println!("temperatures");
    let temperatures = convert_range(&lights, &light_to_temperature);
    println!("humidities");
    let humidities = convert_range(&temperatures, &temperature_to_humidity);
    println!("locations");
    let locations = convert_range(&humidities, &humidity_to_location);

    let min_range = locations
        .iter()
        .min_by(|range1, range2| range1.src.cmp(&range2.src))
        .unwrap();
    println!("Min location: {}", min_range.src);
}
