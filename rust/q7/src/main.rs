use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: u16,
}

fn main() {
    let input = include_str!("../input.txt");

    let solution1 = get_winnings(input, false);
    println!("Solution 1 {}", solution1);

    let solution2 = get_winnings(input, true);
    println!("Solution 2 {}", solution2);
}

fn get_winnings(input: &str, use_wildcards: bool) -> u32 {
    let hands = input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cards_part = parts[0];
        let bid_part = parts[1];

        let cards: Vec<u8> = cards_part
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if use_wildcards {
                        0
                    } else {
                        11
                    }
                }
                'T' => 10,
                _ => c.to_string().parse().unwrap(),
            })
            .collect();

        Hand {
            cards,
            bid: bid_part.parse().unwrap(),
        }
    });

    let hand_types = hands.map(|hand| {
        let mut same_counts: HashMap<u8, u8> = HashMap::new();
        for card in &hand.cards {
            let count: &mut u8 = same_counts.entry(*card).or_insert(0);
            *count += 1;
        }

        let wildcard_ct = if use_wildcards {
            same_counts.remove(&0).unwrap_or(0)
        } else {
            0
        };

        let hand_type: u8 = match same_counts.len() {
            0 | 1 => 6, // five of a kind
            2 => {
                match same_counts.values().max().unwrap() + wildcard_ct {
                    4 => 5, // four of a kind
                    _ => 4, // full house
                }
            }
            3 => {
                match same_counts.values().max().unwrap() + wildcard_ct {
                    3 => 3, // three of a kind
                    _ => 2, // two pair
                }
            }
            4 => 1, // one pair
            5 => 0, // high card
            _ => panic!("Invalid hand, {:?}", hand.cards),
        };
        (hand, hand_type)
    });

    // sort hand types
    let mut ranked_hands: Vec<(Hand, u8)> = hand_types.collect();
    ranked_hands.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cards.cmp(&b.0.cards)));

    let winnings = ranked_hands
        .iter()
        .enumerate()
        .map(|(idx, (hand, _hand_type))| hand.bid as u32 * ((idx as u32) + 1));

    let total_winnings = winnings.sum::<u32>();

    total_winnings
}
