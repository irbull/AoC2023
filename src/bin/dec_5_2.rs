use std::{cmp::min, collections::HashMap};

use regex::Regex;

fn seeds(seeds: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let re = Regex::new(r"([0-9]+)")?;
    for cap in re.captures_iter(seeds) {
        result.push(cap[1].parse::<usize>()?);
    }
    Ok(result)
}

#[derive(Debug)]
struct PlantingMap {
    src: Vec<usize>,
    dest: Vec<usize>,
    range: Vec<usize>,
}

impl PlantingMap {
    fn new(mapping: Vec<&str>) -> PlantingMap {
        let mut src = Vec::new();
        let mut dest = Vec::new();
        let mut range = Vec::new();
        for s in mapping {
            let numbers = s.split(" ").collect::<Vec<&str>>();
            dest.push(numbers.get(0).unwrap().parse::<usize>().unwrap());
            src.push(numbers.get(1).unwrap().parse::<usize>().unwrap());
            range.push(numbers.get(2).unwrap().parse::<usize>().unwrap());
        }

        PlantingMap { src, dest, range }
    }

    fn get_mapping(&self, index: usize) -> usize {
        for i in 0..self.range.len() {
            if index >= self.src[i] && index <= self.src[i] + self.range[i] {
                let offset = index - self.src[i];
                return self.dest[i] + offset;
            }
        }
        index
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_5_1.txt");
    let mut lines = input.lines();
    let seeds = seeds(lines.next().unwrap().split(":").collect::<Vec<&str>>()[1])?;
    lines.next(); // skip blank line

    let mut mapping = Vec::new();
    let mut mappings = HashMap::new();
    let mut current_mapping: &str = "";
    for line in lines {
        if line.contains("map") {
            current_mapping = line.split(" ").collect::<Vec<&str>>().get(0).unwrap();
        } else if line.is_empty() {
            let map = PlantingMap::new(mapping.clone());
            mappings.insert(current_mapping, map);
            mapping.clear();
        } else {
            mapping.push(line);
        }
    }
    let map = PlantingMap::new(mapping.clone());
    mappings.insert(current_mapping, map);
    mapping.clear();

    let mut locations = vec![];

    let s2s = mappings.get("seed-to-soil").unwrap();
    let s2f = mappings.get("soil-to-fertilizer").unwrap();
    let f2w = mappings.get("fertilizer-to-water").unwrap();
    let w2l = mappings.get("water-to-light").unwrap();
    let l2t = mappings.get("light-to-temperature").unwrap();
    let t2h = mappings.get("temperature-to-humidity").unwrap();
    let h2l = mappings.get("humidity-to-location").unwrap();

    for i in (0..seeds.len()).step_by(2) {
        println!("Computing seed {}", i);
        let rng = seeds[i + 1];
        let start = seeds[i];
        for s in 0usize..rng {
            let seed = s + start;
            let soil = s2s.get_mapping(seed);
            let fertilizer = s2f.get_mapping(soil);
            let water = f2w.get_mapping(fertilizer);
            let light = w2l.get_mapping(water);
            let temperature = l2t.get_mapping(light);
            let humidity = t2h.get_mapping(temperature);
            let location = h2l.get_mapping(humidity);
            locations.push(location);
        }
    }

    let smallest = locations.into_iter().reduce(|a, b| min(a, b)).unwrap();
    println!("{:?}", smallest);

    assert_eq!(smallest, 46294175);

    Ok(())
}
