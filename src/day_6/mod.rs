pub fn parse_line(line: String) -> i64 {
    line.split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .fold("".to_owned(), |mut a: String, b| {
            a.push_str(b);
            a
        })
        .parse::<i64>()
        .unwrap()
}

pub fn calculate_wins(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let distance_times = times.iter().zip(distances.iter());

    let mut wins = Vec::new();

    for (time, distance) in distance_times {
        let f_time = *time as f64;
        let f_distance = *distance as f64;

        let discriminant = f64::sqrt((time.pow(2) as f64) - 4.0 * f_distance);

        let min = ((f_time - discriminant) / 2.0).ceil() as i64;
        let max = ((f_time + discriminant) / 2.0).floor() as i64;
        wins.push(max - min + 1);
    }

    wins.into_iter().reduce(|a, b| a * b).unwrap()
}
