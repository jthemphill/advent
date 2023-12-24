#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from_byte(byte: u8) -> Card {
        match byte {
            b'A' => Card::Ace,
            b'K' => Card::King,
            b'Q' => Card::Queen,
            b'J' => Card::Jack,
            b'T' => Card::Ten,
            b'9' => Card::Nine,
            b'8' => Card::Eight,
            b'7' => Card::Seven,
            b'6' => Card::Six,
            b'5' => Card::Five,
            b'4' => Card::Four,
            b'3' => Card::Three,
            b'2' => Card::Two,
            _ => panic!("{} is not a card", byte),
        }
    }

    fn rank(&self) -> usize {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
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

#[derive(Clone, Copy, Eq, Ord, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn ranking(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
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

impl Hand {
    fn hand_type(&self) -> HandType {}
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.cards.partial_cmp(&other.cards) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.bid.partial_cmp(&other.bid)
    }
}

fn main() {
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace();
        let cards = parse_cards(line.next().unwrap());
        let bid = line.next().unwrap().parse::<usize>();
    }
}
