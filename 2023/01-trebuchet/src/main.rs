fn get_first_and_last_num(s: &str) -> u16 {
    let list_of_numbers: Vec<u16> = s
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as u16)
        .collect();

    if list_of_numbers.len() == 0 {
        return 0;
    }

    let first_num = list_of_numbers.first().unwrap();

    if list_of_numbers.len() == 1 {
        return first_num * 10 + first_num;
    }

    let last_num = list_of_numbers.last().unwrap();
    return first_num * 10 + last_num;
}

fn get_first_and_last_num_including_str(s: &str) -> u16 {
    // 🤷‍♂️ you just need one char to make other word read as number.
    let str = s
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "f4or")
        .replace("five", "f5ve")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "n9ne");
    return get_first_and_last_num(str.as_str());
}

fn main() {
    let loaded_puzzle = std::fs::read_to_string("puzzle.txt").unwrap();
    let lines = loaded_puzzle.split("\n").collect::<Vec<&str>>();
    let numbs = lines
        .clone()
        .iter()
        .map(|s| get_first_and_last_num(*s))
        .collect::<Vec<u16>>();

    println!("Result #1: {:?}", numbs.iter().sum::<u16>());

    let numbs = lines
        .clone()
        .iter()
        .map(|s| get_first_and_last_num_including_str(*s))
        .collect::<Vec<u16>>();

    println!("Result #2: {:?}", numbs.iter().sum::<u16>());
}

#[cfg(test)]
mod tests {
    use crate::get_first_and_last_num;
    use crate::get_first_and_last_num_including_str;

    #[test]
    fn always_true() {
        assert_eq!(true, true);
    }

    #[test]
    fn should_return_first_and_last_numb() {
        assert_eq!(get_first_and_last_num("1abc2"), 12);
        assert_eq!(get_first_and_last_num("pqr3stu16vwx"), 36);
        assert_eq!(get_first_and_last_num("a1b2c3d4e5f"), 15);
        assert_eq!(get_first_and_last_num("treb7uchet"), 77);
    }

    #[test]
    fn should_return_first_and_last_numb_including_str() {
        assert_eq!(get_first_and_last_num_including_str("two1nine"), 29);
        assert_eq!(get_first_and_last_num_including_str("eightwothree"), 83);
        assert_eq!(get_first_and_last_num_including_str("abcone2threexyz"), 13);
        assert_eq!(get_first_and_last_num_including_str("xtwone3four"), 24);
        assert_eq!(get_first_and_last_num_including_str("4nineeightseven2"), 42);
        assert_eq!(get_first_and_last_num_including_str("zoneight234"), 14);
        assert_eq!(get_first_and_last_num_including_str("7pqrstsixteen"), 76);
    }
}
