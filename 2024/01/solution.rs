use std::fs;

fn parse_file(content: &str) -> (Vec<i32>, Vec<i32>) {
    let content = content
        .split("\n")
        .map(|c| c.split(' ').filter(|c| *c != "").collect::<Vec<&str>>())
        .filter(|c| c.len() > 0)
        .map(|c| (c[0].parse::<i32>().unwrap(), c[1].parse::<i32>().unwrap()));

    let fst = content.clone().map(|c| c.0).collect::<Vec<i32>>();
    let snd = content.map(|c| c.1).collect::<Vec<i32>>();

    (fst, snd)
}

fn part_1(file: String) -> i32 {
    let (mut fst, mut snd) = parse_file(&file);

    fst.sort();
    snd.sort();

    let mut acc: Vec<i32> = vec![];
    for i in 0..fst.len() {
        let distance = (fst[i] - snd[i]).abs();

        acc.push(distance);
    }

    acc.into_iter().reduce(|a, b| a + b).unwrap()
}

fn part_2(file: String) -> i32 {
    let (fst, snd) = parse_file(&file);

    let mut acc: Vec<i32> = vec![];
    for i in 0..fst.len() {
        let left_value = fst[i];
        let at = snd
            .iter()
            .filter(|c| *c == &left_value)
            .collect::<Vec<_>>()
            .len() as i32;

        acc.push(left_value * at);
    }

    acc.into_iter().reduce(|a, b| a + b).unwrap()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File not found");

    println!("Part 1: {}", part_1(file.clone()));
    println!("Part 2: {}", part_2(file));
}
