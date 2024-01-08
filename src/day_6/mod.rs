pub fn parse_line(line: String) -> Vec<i32> {
    line.split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn calculate_wins(times: Vec<i32>, distances: Vec<i32>) -> i32 {
    let distance_times = times.iter().zip(distances.iter());

    let mut wins = Vec::new();

    for (time, distance) in distance_times {
        let f_time = *time as f32;
        let f_distance = *distance as f32;

        let discriminant = f32::sqrt((time.pow(2) as f32) - 4.0 * f_distance);

        let min = ((f_time - discriminant) / 2.0).ceil() as i32;
        let max = ((f_time + discriminant) / 2.0).floor() as i32;
        wins.push(max - min + 1);
    }

    wins.into_iter().reduce(|a, b| a * b).unwrap()
}
