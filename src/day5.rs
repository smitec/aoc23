use anyhow::{Context, Result};
use bio::data_structures::interval_tree::Entry;
use bio::data_structures::interval_tree::IntervalTree;
use bio::utils::Interval;
use regex::Regex;
use std::collections::HashMap;
use std::{fs::File, io::Read};

#[derive(Clone, Debug)]
struct RangeMap {
    lower_source: i64,
    lower_dest: i64,
    range: i64,
}

struct TreeDest {
    dest: String,
    tree: IntervalTree<i64, RangeMap>,
}

pub fn day5() -> Result<()> {
    // Step 1, parse the input file into useful data types.
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: HashMap<String, TreeDest> = HashMap::new();

    let mut file = File::open("./input/day5.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut map_mode = false;
    let mut source = "".to_string();
    let mut dest = "".to_string();
    let mut mappings: IntervalTree<i64, RangeMap> = IntervalTree::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            if map_mode {
                // Clean up the map
                maps.insert(
                    source.clone(),
                    TreeDest {
                        dest: dest.clone(),
                        tree: mappings.clone(),
                    },
                );
                map_mode = false;
            }
            continue;
        }

        if line.starts_with("seeds:") {
            //parse the seeds
            let seed_numbers = line.replace("seeds: ", "");
            for seed_number in seed_numbers.split_whitespace() {
                let seed_as_int = seed_number.parse::<i64>().context("Could not parse int")?;
                seeds.push(seed_as_int);
            }
        } else if line.contains("map:") {
            // Start parsing in a new map
            map_mode = true;
            let re = Regex::new(r"^(?<source>[a-z]+)-to-(?<dest>[a-z]+) map:$")
                .context("Couldn't setup regex")?;
            let captures = re.captures(line).context("Map line did not match regex")?;
            source = captures["source"].to_string();
            dest = captures["dest"].to_string();
            mappings = IntervalTree::new();
        } else if map_mode {
            let re = Regex::new(r"^(?<deststart>[0-9]+) (?<sourcestart>[0-9]+) (?<range>[0-9]+)$")
                .context("Couldn't setup range parse regex")?;
            let captures = re
                .captures(line)
                .context(format!("Map Data line did not match regex: {:?}", line))?;
            let lower_source = captures["sourcestart"]
                .parse::<i64>()
                .context("Couldn't parse lower source")?;
            let range = captures["range"]
                .parse::<i64>()
                .context("Couldn't parse lower dest")?;
            let this_map = RangeMap {
                lower_source,
                lower_dest: captures["deststart"]
                    .parse::<i64>()
                    .context("Couldn't parse lower dest")?,
                range,
            };
            mappings.insert(lower_source..lower_source + range, this_map.clone());
        }
    }

    // For part 2, map the seeds into the new range format.
    let mut first = 0;
    let mut seed_ranges: Vec<Interval<i64>> = Vec::new();
    for (i, s) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            first = *s;
        } else {
            seed_ranges.push(
                Interval::new(first..first + *s).context("Couldn't form interval from seeds")?,
            );
        }
    }
    // Loop through the seed locations
    let mut current_stage = "seed".to_string();
    let mut current_seeds = seed_ranges.clone();
    while current_stage != *"location" {
        let mut next_seeds: Vec<Interval<i64>> = Vec::new();
        let mapper = maps
            .get(current_stage.as_str())
            .context(format!("No maps for stage {:?}", current_stage))?;
        current_stage = mapper.dest.clone();
        for seed in current_seeds.iter() {
            let mut current_pt = seed.start;
            let mut matches: Vec<Entry<'_, i64, RangeMap>> =
                mapper.tree.find(seed.clone()).collect();
            matches.sort_by_key(|x| x.interval().start);
            for matched_range in matches.iter() {
                if matched_range.interval().start > current_pt {
                    // Add a range from current -> new start with no adjustment
                    next_seeds.push(
                        Interval::new(current_pt..matched_range.interval().start)
                            .context("Couldn't form new range from start to current_pt")?,
                    );
                    current_pt = matched_range.interval().start;
                }
                // Map the overlapping portion to a new range
                if matched_range.interval().end < seed.end {
                    // More of the range exists beyond the end of the matched range
                    let offset = current_pt - matched_range.data().lower_source;
                    let dest_start = matched_range.data().lower_dest;
                    let dest_length = matched_range.data().range;
                    next_seeds.push(
                        Interval::new(dest_start + offset..dest_start + dest_length).context(
                            "Couldn't form new range from dest_start to dest_start + length",
                        )?,
                    );
                    current_pt = matched_range.interval().end;
                    // TODO: possible need to +1 here because its exclusive
                } else {
                    // The seed ends within this matched range
                    let offset = current_pt - matched_range.data().lower_source;
                    let dest_start = matched_range.data().lower_dest;
                    let dest_length = seed.end - current_pt;
                    next_seeds.push(
                        Interval::new(dest_start + offset..dest_start + offset + dest_length)
                            .context("Couldn't form new range from dest_start to end of seed")?,
                    );
                    current_pt = seed.end;
                }
            }

            if seed.end > current_pt {
                // Add a range from current -> end with no adjustment
                next_seeds.push(
                    Interval::new(current_pt..seed.end)
                        .context("Couldn't form new range with remaining")?,
                );
            }
        }
        current_seeds = next_seeds;
    }
    let mut min_match = -1;
    for seed in current_seeds {
        if min_match == -1 || seed.start < min_match {
            min_match = seed.start;
        }
    }
    println!("Part B min: {:?}", min_match);

    // Find a path for each seed to its location
    let mut min_loc = -1;
    for seed in seeds.iter() {
        let mut current_stage = "seed".to_string();
        let mut current_value = *seed;
        while current_stage != *"location" {
            let mapper = maps
                .get(current_stage.as_str())
                .context(format!("No maps for stage {:?}", current_stage))?;
            let dest_stage = mapper.dest.clone();
            for map in mapper.tree.find(current_value..current_value + 1) {
                let data = map.data();
                if current_value >= data.lower_source
                    && current_value < data.lower_source + data.range
                {
                    current_value = data.lower_dest + (current_value - data.lower_source);
                    break;
                }
            }

            current_stage = dest_stage;
        }
        if min_loc == -1 || current_value < min_loc {
            min_loc = current_value;
        }
    }
    println!("Lowest location: {:?}", min_loc); // Part 1 should be 535088217

    Ok(())
}
