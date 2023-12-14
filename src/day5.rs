mod utils;
use std::ops::Range;
use std::cmp::{min, max};

#[derive(Debug)]
struct RangeMapping {
    src_range: Range<usize>,
    dst_range: Range<usize>,
}

impl RangeMapping {
    fn identity(start: usize, end: usize) -> Self {
        RangeMapping {
            src_range: Range { start, end },
            dst_range: Range { start, end },
        }
    }

    fn map(&self, input: usize) -> Option<usize> {
        if self.src_range.contains(&input) {
            return Some(input - self.src_range.start + self.dst_range.start);
        }
        None
    }

    fn map_range(&self, input: &Range<usize>) -> Option<Range<usize>> {
        if self.src_range.start <= input.end && self.src_range.end >= input.start {

            let start = max(self.src_range.start, input.start);
            let end = min(self.src_range.end, input.end);
            return Some(Range {
                start: start - self.src_range.start + self.dst_range.start,
                end: end - self.src_range.start + self.dst_range.start,
            });
        }

        None
    }
}

#[derive(Debug)]
struct AlmanacMapping {
    mappings: Vec<RangeMapping>
}

impl AlmanacMapping {
    fn map(&self, input: usize) -> usize {
        let mapped: Vec<_> = self.mappings.iter()
                                        .filter_map(|mapping| mapping.map(input))
                                        .collect();

        match mapped.len() {
            0 => input,
            _ => mapped[0],
        }
    }

    fn map_range(&self, input: &Range<usize>) -> Vec<Range<usize>> {
        let mapped: Vec<Range<usize>> = self.mappings.iter()
                                                    .filter_map(|mapping| mapping.map_range(input))
                                                    .collect();

        mapped
    }

    fn map_ranges(&self, input: &Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mapped: Vec<Range<usize>> = input.iter()
                                                    .map(|i| self.map_range(i))
                                                    .flatten()
                                                    .collect();
        mapped
    }
}

fn part1() {
    const FILE: &str = "./inputs/day5.txt";
    let lines = utils::read_lines_in_file(FILE);

    let (seeds, _, mappings) = parse_almanac(&lines);
    
    let lowest = seeds.iter()
                        .map(|number| { 
                            let mut result = *number;
                            for mapping in &mappings {
                                result = mapping.map(result);
                            }
                            result
                        })
                        .min()
                        .unwrap();

    println!("Part 1: Lowest location is {lowest}");
}

fn part2() {
    const FILE: &str = "./inputs/day5.txt";
    let lines = utils::read_lines_in_file(FILE);

    let (_, seed_ranges, mappings) = parse_almanac(&lines);

    let mut current_map = seed_ranges;
    for mapping in &mappings {
        current_map = mapping.map_ranges(&current_map);
    }

    let lowest = current_map.iter()
                            .map(|item| item.start)
                            .min()
                            .unwrap();

    println!("Part 2: Lowest location is {lowest}");
}

fn ranges_from(seeds: &Vec<usize>) -> Vec<Range<usize>> {

    let mut ranges: Vec<Range<usize>> = Vec::new();

    for i in (0..seeds.len()).step_by(2) {
        let start_seed = seeds[i];
        let len = seeds[i+1];
        ranges.push(Range {
            start: start_seed,
            end: start_seed+len,
        });
    }

    ranges
}

fn parse_almanac(lines: &Vec<String>) -> (Vec<usize>, Vec<Range<usize>>, Vec<AlmanacMapping>) {
    let seeds: Vec<usize> = lines[0].split([':', ' '])
                        .filter_map(|item| if let Ok(n) = item.trim().parse::<usize>() { Some(n) } else { None })
                        .collect();
    let mut mappings: Vec<AlmanacMapping> = Vec::new();
    
    // Parse the input file and derive list of seed values and a list of almanac mappings that each
    // contain a list of range mappings to map ranges of numbers onto other ranges
    // Note: We assume that all almanac mappings are in order, i.e. that the n-th block of range
    // mappings in the file receives it's inputs from the (n-1)-th block of ranges
    for line in lines[1..].iter() {
        if !line.is_empty() && !line.chars().next().unwrap().is_numeric() {
            mappings.push(AlmanacMapping {
                mappings: vec![]
            });
        } else if !line.is_empty() && line.chars().next().unwrap().is_numeric() {
            let parts: Vec<_> = line.split(' ').collect();
            let dst_min = parts[0].parse::<usize>().unwrap();
            let src_min = parts[1].parse::<usize>().unwrap();
            let len = parts[2].parse::<usize>().unwrap();

            let num_mappings = mappings.len();
            let current_almanac_mapping: &mut AlmanacMapping = &mut mappings[num_mappings - 1];
            current_almanac_mapping.mappings.push(RangeMapping {
                src_range: Range { start: src_min, end: src_min+len },
                dst_range: Range { start: dst_min, end: dst_min+len },
            });
        }
    }

    // Handle holes in the mapping ranges
    // We want to fill those holes with ID mappings and also consider the edge cases where a
    // previous layer's min/max output exceeds the bouds of the newly defined input ranges
    let seed_ranges = ranges_from(&seeds);
    let mut mapping_layer_min = seed_ranges.iter().map(|seed_range| seed_range.start).min().unwrap();
    let mut mapping_layer_max = seed_ranges.iter().map(|seed_range| seed_range.end).max().unwrap();

    for mapping in &mut mappings {
        mapping.mappings.sort_by(|a, b| a.src_range.start.cmp(&b.src_range.start));

        let mut mappings_to_insert: Vec<RangeMapping> = Vec::new();
        for i in 0..mapping.mappings.len() {

            // Insert ID mapping from lowest value of the previous mapping layer to first mapping range
            if i == 0 {
                if mapping_layer_min < mapping.mappings[i].src_range.start {
                    mappings_to_insert.push(RangeMapping::identity(mapping_layer_min, mapping.mappings[i].src_range.start));
                }
            }

            // Insert ID mapping between non-contiguous ranges
            if i != mapping.mappings.len() - 1 && mapping.mappings[i].src_range.end != mapping.mappings[i+1].src_range.start {
                mappings_to_insert.push(RangeMapping::identity(mapping.mappings[i].src_range.start, mapping.mappings[i+1].src_range.start));
            }

            // Insert ID mapping from last mapping range to highest value of the previous mapping
            // layer
            if i == mapping.mappings.len() - 1 {
                if mapping_layer_max > mapping.mappings[i].src_range.end {
                    mappings_to_insert.push(RangeMapping::identity(mapping.mappings[i].src_range.end, mapping_layer_max));
                }
            }
        }

        mapping.mappings.append(&mut mappings_to_insert);

        mapping_layer_min = mapping.mappings.iter().map(|m| m.dst_range.start).min().unwrap();
        mapping_layer_max = mapping.mappings.iter().map(|m| m.dst_range.end).max().unwrap();
    }

    (seeds, seed_ranges, mappings)
}

fn main() {
    part1();
    part2();
}
