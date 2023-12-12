fn get_winning_card_from_scratchcard(scratchcards: &str) -> Vec<&str> {
    scratchcards
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
}

fn get_my_card_from_scratchcard(scratchcards: &str) -> Vec<&str> {
    scratchcards.split_whitespace().collect::<Vec<&str>>()
}

fn get_points_from_scratchcards(s: &str) -> u32 {
    let mut points = 0;

    let mut scratchcards = s.split(" | ");
    let winning_card = get_winning_card_from_scratchcard(scratchcards.next().unwrap());
    let my_card_numbers = get_my_card_from_scratchcard(scratchcards.next().unwrap());

    for number in my_card_numbers {
        if winning_card.contains(&number) {
            points += match points {
                0 => 1,
                n => n,
            };
        }
    }

    points
}

fn get_total_cards_from_pile(pile: Vec<&str>) -> u32 {
    let num_games = pile.len();
    let mut stack = vec![1; num_games];

    for (i, card) in pile.iter().enumerate() {
        let mut scratchcards = card.split(" | ");
        let winning_card = get_winning_card_from_scratchcard(scratchcards.next().unwrap());
        let my_card_numbers = get_my_card_from_scratchcard(scratchcards.next().unwrap());

        let mut won = 0;
        let mut points = 0;

        for number in my_card_numbers {
            if winning_card.contains(&number) {
                won += 1;

                points += match points {
                    0 => 1,
                    n => n,
                };
            }
        }

        for j in 1..=won {
            if i + j < num_games {
                stack[i + j] += stack[i];
            }
        }
    }

    stack.iter().sum()
}

fn main() {
    let lines = include_str!("puzzle.txt").lines();

    println!(
        "Total points from scratchcards: {}",
        lines
            .map(|line| get_points_from_scratchcards(line))
            .sum::<u32>()
    );

    let lines = include_str!("puzzle.txt").lines();

    println!(
        "Total stack of cards from pile: {}",
        get_total_cards_from_pile(lines.collect::<Vec<&str>>())
    );
}

#[cfg(test)]
mod test {
    use crate::{get_points_from_scratchcards, get_total_cards_from_pile};

    #[test]
    fn should_return_points_from_scratchcards() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17 9 48 53";
        assert_eq!(get_points_from_scratchcards(s), 8);

        let s = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        assert_eq!(get_points_from_scratchcards(s), 2);

        let s = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        assert_eq!(get_points_from_scratchcards(s), 2);

        let s = "Card 4: 41 92 73 84 69 | 59 84 76 51 58 5 54 83";
        assert_eq!(get_points_from_scratchcards(s), 1);

        let s = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        assert_eq!(get_points_from_scratchcards(s), 0);

        let s = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(get_points_from_scratchcards(s), 0);
    }

    #[test]
    fn should_return_numbers_of_total_cards() {
        let pile = Vec::from([
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17 9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58 5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]);

        assert_eq!(get_total_cards_from_pile(pile), 30);
    }
}
