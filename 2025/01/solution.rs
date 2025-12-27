#[derive(Debug, Clone)]
enum Directions {
    Left(i32),
    Right(i32),
}

impl std::ops::Deref for Directions {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        match self {
            Directions::Left(s) => s,
            Directions::Right(s) => s,
        }
    }
}

fn parse_directions(input: String) -> Vec<Directions> {
    let mut directions = Vec::new();

    for input in input.trim().lines().collect::<Vec<_>>() {
        let input = input.trim();
        let direction = &input[0..1];
        let steps = (&input[1..]).parse::<i32>().unwrap();

        let direction = match direction {
            "L" => Directions::Left(steps),
            "R" => Directions::Right(steps),
            _ => std::panic!("invalid input"),
        };

        directions.push(direction);
    }

    directions
}

fn part_1(directions: Vec<Directions>) -> usize {
    directions
        .into_iter()
        .fold((50, Vec::new()), |(prev, mut acc), curr| {
            let next = {
                let delta = match curr {
                    Directions::Left(s) => prev - s,
                    Directions::Right(s) => prev + s,
                };

                ((delta % 100) + 100) % 100
            };
            acc.push(next);
            (next, acc)
        })
        .1
        .iter()
        .filter(|x| **x == 0)
        .collect::<Vec<_>>()
        .len()
}

fn part_2(directions: Vec<Directions>) -> usize {
    let mut pos = 50;
    let mut count = 0;

    directions.iter().for_each(|direction| {
        for _ in 0..**direction {
            match direction {
                Directions::Left(_) => {
                    pos = (pos - 1 + 100) % 100;
                }
                Directions::Right(_) => {
                    pos = (pos + 1 + 100) % 100;
                }
            }

            if pos == 0 {
                count += 1;
            }
        }
    });

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input.txt")?;
    let directions = parse_directions(file);

    println!("Part 1: {}", part_1(directions.clone()));
    println!("Part 2: {}", part_2(directions));

    Ok(())
}
