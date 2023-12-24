use std::{cmp::Ordering, collections::HashMap};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 0,
}

impl Card {
    fn from_byte(byte: u8) -> Card {
        match byte {
            b'A' => Card::Ace,
            b'K' => Card::King,
            b'Q' => Card::Queen,
            b'T' => Card::Ten,
            b'9' => Card::Nine,
            b'8' => Card::Eight,
            b'7' => Card::Seven,
            b'6' => Card::Six,
            b'5' => Card::Five,
            b'4' => Card::Four,
            b'3' => Card::Three,
            b'2' => Card::Two,
            b'J' => Card::Joker,
            _ => panic!("{} is not a card", byte),
        }
    }
    fn to_byte(self) -> u8 {
        match self {
            Card::Ace => b'A',
            Card::King => b'K',
            Card::Queen => b'Q',
            Card::Ten => b'T',
            Card::Nine => b'9',
            Card::Eight => b'8',
            Card::Seven => b'7',
            Card::Six => b'6',
            Card::Five => b'5',
            Card::Four => b'4',
            Card::Three => b'3',
            Card::Two => b'2',
            Card::Joker => b'J',
        }
    }

    fn rank(&self) -> usize {
        *self as usize
    }
}

fn as_string(cards: &[Card; 5]) -> String {
    let mut s: [u8; 5] = [0; 5];
    for i in 0..5 {
        s[i] = cards[i].to_byte();
    }
    std::str::from_utf8(&s).unwrap().to_string()
}

fn parse_cards(card_str: &str) -> [Card; 5] {
    let card_bytes = card_str.as_bytes();
    assert_eq!(card_bytes.len(), 5);
    let mut cards = [Card::Ace; 5];
    for i in 0..5 {
        cards[i] = Card::from_byte(card_bytes[i]);
    }
    cards
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn ranking(&self) -> usize {
        *self as usize
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ranking().partial_cmp(&other.ranking())
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

fn hand_type(cards: &[Card; 5]) -> HandType {
    let mut counter: HashMap<Card, usize> = HashMap::new();
    let mut num_jokers = 0;
    for &card in cards {
        *counter.entry(card).or_insert(0) += 1;
        if card == Card::Joker {
            num_jokers += 1;
        }
    }

    // Sort highest count first
    let mut card_counts: Vec<(Card, usize)> = counter.into_iter().collect();
    card_counts
        .sort_by(|(card1, count1), (card2, count2)| count2.cmp(&count1).then(card2.cmp(&card1)));

    let mut num_three_of_a_kind = 0;
    let mut num_two_of_a_kind = 0;
    for (card, count) in card_counts {
        // Ignore Jokers unless the whole hand is Jokers
        if card == Card::Joker {
            if count == 5 {
                return HandType::FiveOfAKind;
            } else {
                continue;
            }
        }
        assert!(count + num_jokers <= 5);
        match count + num_jokers {
            5 => {
                return HandType::FiveOfAKind;
            }
            4 => {
                return HandType::FourOfAKind;
            }
            3 => {
                num_jokers -= 3 - count;
                num_three_of_a_kind += 1;
            }
            2 => {
                num_jokers -= 2 - count;
                num_two_of_a_kind += 1;
            }
            _ => {}
        };
    }
    if num_three_of_a_kind == 1 && num_two_of_a_kind == 1 {
        HandType::FullHouse
    } else if num_three_of_a_kind == 1 {
        HandType::ThreeOfAKind
    } else if num_two_of_a_kind == 2 {
        HandType::TwoPair
    } else if num_two_of_a_kind == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type_ordering = hand_type(&self.cards).cmp(&hand_type(&other.cards));
        if hand_type_ordering != Ordering::Equal {
            return Some(hand_type_ordering);
        }
        for i in 0..5 {
            let card_ordering = self.cards[i].cmp(&other.cards[i]);
            if card_ordering != Ordering::Equal {
                return Some(card_ordering);
            }
        }
        self.bid.partial_cmp(&other.bid)
    }
}

fn main() {
    let mut hands = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace();
        let cards = parse_cards(line.next().unwrap());
        let bid = line.next().unwrap().parse::<usize>().unwrap();
        hands.push(Hand { cards, bid });
    }

    hands.sort();

    let mut score = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = i + 1;
        let hand_score = rank * hand.bid;
        println!(
            "Rank {:4}: {:12?} {:5} {:3} = {}",
            rank,
            hand_type(&hand.cards),
            as_string(&hand.cards),
            hand.bid,
            hand_score,
        );
        score += hand_score;
    }
    println!("Total score: {}", score);
}
