use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: u16,
}

fn main() {
    // let path = Path::new("./q7/input.txt").canonicalize().unwrap();
    // let file = File::open(path).unwrap();
    // let reader = BufReader::new(file);

    let input = include_str!("../input.txt");

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
                'J' => 11,
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
        let mut groups: HashMap<u8, u8> = HashMap::new();
        for card in &hand.cards {
            let count: &mut u8 = groups.entry(*card).or_insert(0);
            *count += 1;
        }
        let hand_type: u8 = match groups.len() {
            1 => 6, // five of a kind
            2 => {
                match groups.values().max().unwrap() {
                    4 => 5, // four of a kind
                    _ => 4, // full house
                }
            }
            3 => {
                match groups.values().max().unwrap() {
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
    ranked_hands.sort_by(|a, b| {
        a.1.cmp(&b.1).then(a.0.cards.cmp(&b.0.cards))
    });

    println!("{:#?}", ranked_hands);

    let winnings = ranked_hands
        .iter()
        .enumerate()
        .map(|(idx, (hand, _hand_type))| hand.bid as u32 * ((idx as u32) + 1) );

    let total_winnings = winnings.sum::<u32>();

    println!("{:?}", total_winnings);
}
