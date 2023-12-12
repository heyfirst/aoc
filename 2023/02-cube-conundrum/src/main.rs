fn get_cube_bag_possible(s: &str) -> u16 {
    let game_id = s
        .split(":")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .replace("Game ", "")
        .parse::<u16>()
        .unwrap();

    let sets = s
        .split(":")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(";")
        .collect::<Vec<&str>>();

    for set in sets {
        let colors = set.split(",").collect::<Vec<&str>>();
        for color in colors {
            let color = color.trim();
            if color.contains("red") {
                let red = color.replace("red", "").trim().parse::<u16>().unwrap();
                if red > 12 {
                    return 0;
                }
            } else if color.contains("green") {
                let green = color.replace("green", "").trim().parse::<u16>().unwrap();
                if green > 13 {
                    return 0;
                }
            } else if color.contains("blue") {
                let blue = color.replace("blue", "").trim().parse::<u16>().unwrap();
                if blue > 14 {
                    return 0;
                }
            }
        }
    }

    return game_id;
}

fn get_power_from_cube_bag(s: &str) -> u32 {
    let sets = s
        .split(":")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(";")
        .collect::<Vec<&str>>();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for set in sets {
        let colors = set.split(",").collect::<Vec<&str>>();
        for color in colors {
            let color = color.trim();
            if color.contains("red") {
                let red = color.replace("red", "").trim().parse::<u16>().unwrap();
                if red > max_red {
                    max_red = red
                }
            } else if color.contains("green") {
                let green = color.replace("green", "").trim().parse::<u16>().unwrap();
                if green > max_green {
                    max_green = green
                }
            } else if color.contains("blue") {
                let blue = color.replace("blue", "").trim().parse::<u16>().unwrap();
                if blue > max_blue {
                    max_blue = blue
                }
            }
        }
    }

    return (max_red * max_green * max_blue).into();
}

fn main() {
    let loaded_puzzle = std::fs::read_to_string("puzzle.txt").unwrap();
    let lines = loaded_puzzle.split("\n").collect::<Vec<&str>>();
    let numbs = lines
        .iter()
        .map(|s| get_cube_bag_possible(*s))
        .collect::<Vec<u16>>();

    println!("Result #1: {:?}", numbs.iter().sum::<u16>());

    let numbs = lines
        .iter()
        .map(|s| get_power_from_cube_bag(*s))
        .collect::<Vec<u32>>();
    println!("Result #2: {:?}", numbs.iter().sum::<u32>());
}

#[cfg(test)]
mod test {
    use crate::{get_cube_bag_possible, get_power_from_cube_bag};

    #[test]
    fn always_true() {
        assert_eq!(true, true);
    }

    #[test]
    fn should_return_correct_cube_possible() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(get_cube_bag_possible(s), 1);

        let s = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(get_cube_bag_possible(s), 2);

        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(get_cube_bag_possible(s), 0);

        let s = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(get_cube_bag_possible(s), 0);

        let s = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(get_cube_bag_possible(s), 5);
    }

    #[test]
    fn should_return_powers() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(get_power_from_cube_bag(s), 48);

        let s = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(get_power_from_cube_bag(s), 12);

        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(get_power_from_cube_bag(s), 1560);

        let s = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(get_power_from_cube_bag(s), 630);

        let s = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(get_power_from_cube_bag(s), 36);
    }
}
