fn get_ways_to_win(race_time: u64, race_distance: u64) -> u64 {
    let mut ways_to_win = 0;

    // 0 1 2 3 4 5 6 7
    let mut speed = 0;
    for time_to_told in 0..race_time {
        speed += time_to_told;
        let distance = speed * (race_time - time_to_told);
        if distance > race_distance {
            ways_to_win += 1;
        }
        speed = 0;
    }

    ways_to_win
}

fn main() {
    // Puzzle input
    // Time:        54     70     82     75
    // Distance:   239   1142   1295   1253

    println!(
        "Ways to win: {}",
        vec![
            get_ways_to_win(54, 239),
            get_ways_to_win(70, 1142),
            get_ways_to_win(82, 1295),
            get_ways_to_win(75, 1253)
        ]
        .iter()
        .product::<u64>()
    );

    println!(
        "Ways to win: {}",
        vec![get_ways_to_win(54708275, 239114212951253),]
            .iter()
            .product::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use crate::get_ways_to_win;

    #[test]
    fn should_return_time_to_hold() {
        assert_eq!(get_ways_to_win(7, 9), 4);
        assert_eq!(get_ways_to_win(15, 40), 8);
        assert_eq!(get_ways_to_win(30, 200), 9);
    }
}
