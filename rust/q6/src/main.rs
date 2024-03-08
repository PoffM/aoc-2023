use std::cmp::min;

#[derive(Debug)]
struct Race {
    time: i128,
    record: i128,
}

fn main() {
    // read file
    let lines = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    // split on spaces
    let times = &lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    // split on spaces
    let records = &lines[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let races = times
        .iter()
        .zip(records.iter())
        .map(|(time, distance)| Race {
            time: *time,
            record: *distance,
        })
        .collect::<Vec<Race>>();

    let mut num_solutions_vec = Vec::new();

    for race in races {
        num_solutions_vec.push(count_solutions(&race))
    }

    let product = num_solutions_vec.iter().fold(1, |a, b| a * b);

    println!("{:?}", num_solutions_vec);
    println!("Solution for part 1: {}", product);

    // Solution for part 2
    let time_chars = lines[0].chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>();

    let record_chars = lines[1].chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>();

    let time = String::from_iter(time_chars).parse().unwrap();
    let record = String::from_iter(record_chars).parse().unwrap();

    println!("Time: {}, distance: {}", time, record);

    let sol2 = count_solutions(&Race { record, time });
    println!("Solution for part 2: {}", sol2);
}

fn calc_distance(race: &Race, start: i128) -> i128 {
    let speed = start;
    let remaining_time = race.time - start;
    let distance = speed * remaining_time;

    return distance;
}

fn count_solutions(race: &Race) -> i128 {
    let mut num_solutions: i128 = 1;

    let directions: Vec<i128> = vec![-1, 1];
    for direction in directions {
        let mut cursor = race.time / 2;
        let mut step_size = 1;
        while step_size > 0 {
            let mut new_cursor = cursor + (step_size * direction);
            if new_cursor < 0 {
                new_cursor = 0;
            }
            if new_cursor > race.time {
                new_cursor = race.time;
            }

            let new_distance = calc_distance(&race, new_cursor);
            if new_distance >= race.record {
                cursor = new_cursor;
                num_solutions += step_size;
                step_size *= 2;
            } else {
                if step_size == 1 {
                    break;
                }
                step_size /= 2;
            }
        }
    }
    return num_solutions
}
