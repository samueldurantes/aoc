fn parse_ranges(input: String) -> Vec<std::ops::RangeInclusive<i64>> {
    input
        .trim()
        .split(',')
        .into_iter()
        .map(|ranges| {
            let (start, end) = ranges
                .split_once('-')
                .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                .unwrap();
            start..=end
        })
        .collect()
}

fn part_1(ranges: Vec<std::ops::RangeInclusive<i64>>) -> i64 {
    ranges
        .into_iter()
        .map(|range| {
            range.into_iter().filter(|id| {
                let id = id.to_string();
                let len = id.len();

                if len % 2 != 0 {
                    return false;
                }

                let half = len / 2;
                let (first, second) = id.split_at(half);

                first == second
            })
        })
        .flatten()
        .sum()
}

fn part_2(ranges: Vec<std::ops::RangeInclusive<i64>>) -> i64 {
    ranges
        .into_iter()
        .map(|range| {
            range.into_iter().filter(|id| {
                let id = id.to_string();
                let n = id.len();

                for len in 1..=n / 2 {
                    if n % len != 0 {
                        continue;
                    }

                    let pattern = &id[..len];
                    let repeats = n / len;

                    if pattern.repeat(repeats) == id {
                        return true;
                    }
                }

                false
            })
        })
        .flatten()
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input.txt")?;
    let ranges = parse_ranges(file);

    println!("Part 1: {}", part_1(ranges.clone()));
    println!("Part 2: {}", part_2(ranges));

    Ok(())
}
