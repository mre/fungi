use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::error::Error;
use std::str::{self, FromStr};

// --- Face --------------------------------------------------------------------

// Face, in order of definition to benefit of the automatic comparison.
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Copy, Clone, Hash)]
enum Face {
    N(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Face {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Face::*;
        return match s {
            // add all possible Faces here
            // ... boring ...
            // "_ 2 3 4 5 6 7 8 9 10 J Q K A"
            "A" => Ok(Ace),
            "K" => Ok(King),
            "Q" => Ok(Queen),
            "J" => Ok(Jack),
            "10" => Ok(N(10)),
            // Some(ch @ '2'...'9') => ch.to_digit(10).map(|n| Rank::N(n as u8)),
            "9" => Ok(N(9)),
            "8" => Ok(N(8)),
            "7" => Ok(N(7)),
            "6" => Ok(N(6)),
            "5" => Ok(N(5)),
            "4" => Ok(N(4)),
            "3" => Ok(N(3)),
            "2" => Ok(N(2)),
            _ => Err(Box::<Error>::from("cannot parse Rank from string")),
        };
    }
}

impl From<u8> for Face {
    fn from(n: u8) -> Self {
        use self::Face::*;
        return match n {
            14 => Ace,
            f @ 2...10 => N(f),
            11 => Jack,
            12 => Queen,
            13 => King,
            _ => panic!("unknown face"),
        };
    }
}

impl From<i8> for Face {
    fn from(n: i8) -> Self {
        use self::Face::*;
        return match n {
            14 => Ace,
            f @ 2...10 => N(f as u8),
            11 => Jack,
            12 => Queen,
            13 => King,
            _ => panic!("unknown face"),
        };
    }
}

impl Into<u8> for Face {
    fn into(self) -> u8 {
        use self::Face::*;
        return match self {
            N(n) => n as u8,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 14,
        };
    }
}

// --- Suit --------------------------------------------------------------------

#[derive(Debug, Eq, Copy, Clone, PartialOrd, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl FromStr for Suit {
    type Err = Box<dyn std::error::Error>;

    // ("HSCD".split("").position(|i| i == s).unwrap() as u8);
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Suit::*;
        return match s {
            "H" => Ok(Hearts),
            "D" => Ok(Diamonds),
            "S" => Ok(Spades),
            "C" => Ok(Clubs),
            _ => Err(Box::<Error>::from("cannot parse Suit from string")),
        };
    }
}

impl Into<u8> for Suit {
    fn into(self) -> u8 {
        use self::Suit::*;
        return match self {
            Hearts => 0,
            Diamonds => 1,
            Spades => 2,
            Clubs => 3,
        };
    }
}

// --- Card --------------------------------------------------------------------

#[derive(Debug, Eq, Copy, Clone)]
struct Card {
    face: Face,
    suit: Suit,
}

// Sort by comparing the face of the card.
impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.face.cmp(&other.face)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Equality by comparing the face of the card.
impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.face == other.face
    }
}

impl Card {
    // Create a card from a string for a face and a string for the suit.
    #[allow(dead_code)]
    fn new(face: &str, suit: &str) -> Result<Self, Box<dyn std::error::Error>> {
        return Ok(Card {
            face: Face::from_str(face)?,
            suit: Suit::from_str(suit)?,
        });
    }
}

// --- Rank --------------------------------------------------------------------

// Rank, in order of definition to benefit of the automatic comparison.
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Copy, Clone)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl Into<u8> for Rank {
    fn into(self) -> u8 {
        use self::Rank::*;
        return match self {
            HighCard => 0,
            OnePair => 1,
            TwoPair => 2,
            ThreeKind => 3,
            Straight => 4,
            Flush => 5,
            FullHouse => 6,
            FourOfAKind => 7,
            StraightFlush => 8,
        };
    }
}

impl Into<usize> for Rank {
    fn into(self) -> usize {
        use self::Rank::*;
        return match self {
            HighCard => 0,
            OnePair => 1,
            TwoPair => 2,
            ThreeKind => 3,
            Straight => 4,
            Flush => 5,
            FullHouse => 6,
            FourOfAKind => 7,
            StraightFlush => 8,
        };
    }
}

impl From<usize> for Rank {
    fn from(n: usize) -> Rank {
        use self::Rank::*;
        return match n {
            0 => HighCard,
            1 => OnePair,
            2 => TwoPair,
            3 => ThreeKind,
            4 => Straight,
            5 => Flush,
            6 => FullHouse,
            7 => FourOfAKind,
            8 => StraightFlush,
            _ => panic!("not a rankeable usize"),
        };
    }
}

impl FromStr for Rank {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            _ => Err(Box::<Error>::from("cannot parse Rank from string")),
        };
    }
}

// Hand ------------------------------------------------------------------------

struct Hand<'a> {
    raw: &'a str,
    cards: [Card; 5],
    rank: Option<Rank>,
}

impl<'a> Hand<'a> {
    #[allow(dead_code)]
    fn as_str(self) -> &'a str {
        self.raw
    }
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(raw: &'a str) -> Self {
        let mut hand: Vec<Card> = Vec::with_capacity(5);
        for c in raw.split_whitespace() {
            let (r, s) = c.split_at(c.len() - 1);
            let face: Face = Face::from_str(r).unwrap();
            let suit: Suit = Suit::from_str(s).unwrap();
            hand.push(Card { face, suit });
        }
        let cards: [Card; 5] = [hand[0], hand[1], hand[2], hand[3], hand[4]];
        let rank: Option<Rank> = None;
        Hand { raw, cards, rank }
    }
}

impl<'a> std::cmp::PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl<'a> std::cmp::PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl<'a> Hand<'a> {}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_
/// reference to the winning hand(s) as were passed in, not
/// reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut top: Vec<Hand> = Vec::new();
    for h in hands {
        let mut m: HashMap<usize, Card> = HashMap::new();
        for (i, c) in h.split_whitespace().enumerate() {
            // https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust/24542608#24542608
            // .chars().last().unwrap();
            //
            // https://doc.rust-lang.org/std/string/struct.String.html#method.split_at
            let (a, b): (&str, &str) = c.split_at(c.len() - 1);
            let card: Card = Card {
                face: Face::from_str(a).unwrap(),
                suit: Suit::from_str(b).unwrap(),
            };
            m.insert(i, card);
        }
        let cards: [Card; 5] = [m[&0], m[&1], m[&2], m[&3], m[&4]];
        let hand: Hand = Hand {
            raw: h,
            cards: cards,
            rank: None,
        };
        if top.len() == 0 || hand.rank() == top[0].rank() {
            top.push(hand);
        } else if hand.rank() > top[0].rank() {
            top.clear();
            top.push(hand);
        }
    }
    let mut winners: Vec<&'a str> = Vec::new();
    for t in top {
        winners.push(t.raw);
    }
    Some(winners)
}

impl<'a> Hand<'a> {
    fn rank(&self) -> (Option<Rank>, Vec<u8>) {
        let ranks: [fn(&Vec<(u8, u8)>, &HashMap<u8, u8>) -> Option<Vec<u8>>; 9] = [
            rank_00_high_card,
            rank_01_one_pair,
            rank_02_two_pair,
            rank_03_three_of_a_kind,
            rank_04_straight,
            rank_05_flush,
            rank_06_full_house,
            rank_07_four_of_a_kind,
            rank_08_straigh_flush,
        ];

        let faces: Vec<(u8, u8)> = self.faces();
        let suits: HashMap<u8, u8> = self.suits();

        // the highest IDX wins compared to another rank.
        for (idx, func) in ranks.iter().enumerate().rev() {
            if let Some(score) = func(&faces, &suits) {
                return (Some(idx.into()), score);
            }
        }
        return (None, Vec::new());
    }

    // faces frequencies
    fn faces(&self) -> Vec<(u8, u8)> {
        let mut freq: HashMap<u8, u8> = HashMap::new();
        for card in self.cards.iter() {
            *freq.entry(card.face.into()).or_insert(0) += 1;
        }
        let mut faces = freq.iter().collect::<Vec<(&u8, &u8)>>();
        faces.sort_by(|&a, &b| {
            if a.1 != b.1 {
                // sort by frequency
                b.1.cmp(a.1)
            } else {
                // sort by face
                b.0.cmp(a.0)
            }
        });

        return faces.iter().cloned().map(|(&a, &b)| (a, b)).collect();
    }

    // suits count
    fn suits(&self) -> HashMap<u8, u8> {
        let mut freq: HashMap<u8, u8> = HashMap::new();
        for card in self.cards.iter() {
            *freq.entry(card.suit.into()).or_insert(0) += 1;
        }
        return freq;
    }
}

// values is a frequency vector: Vec<(Face, frequency)>
// this function returns the list of faces in this hand.
fn rank_00_high_card(values: &Vec<(u8, u8)>, _: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    let counts: Vec<u8> = values.iter().map(|v| v.1).collect();
    if counts == [1, 1, 1, 1, 1] {
        Some(values.iter().map(|v| v.0).collect())
    } else {
        None
    }
}

fn rank_01_one_pair(values: &Vec<(u8, u8)>, _: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    let counts: Vec<u8> = values.iter().map(|v| v.1).collect();
    if counts == [2, 1, 1, 1] {
        Some(values.iter().map(|v| v.0).collect())
    } else {
        None
    }
}

fn rank_02_two_pair(values: &Vec<(u8, u8)>, _: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    let counts: Vec<u8> = values.iter().map(|v| v.1).collect();
    if counts == [2, 2, 1] {
        Some(values.iter().map(|v| v.0).collect())
    } else {
        None
    }
}

fn rank_03_three_of_a_kind(values: &Vec<(u8, u8)>, _: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    let counts: Vec<u8> = values.iter().map(|v| v.1).collect();
    if counts == [3, 1, 1] {
        Some(values.iter().map(|v| v.0).collect())
    } else {
        None
    }
}

fn rank_04_straight(values: &Vec<(u8, u8)>, _: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    let max = values.first().unwrap().0 as i8;
    let diffs = values
        .iter()
        .map(|&v| max - (v.0 as i8))
        .collect::<Vec<i8>>();
    if diffs == &[0, 1, 2, 3, 4] {
        Some(values.iter().map(|&v| v.0).collect())
    } else if Face::from(max) == Face::Ace && diffs == [0, 9, 10, 11, 12] {
        Some(vec![0, 1, 2, 3, 4])
    } else {
        None
    }
}

fn rank_05_flush(values: &Vec<(u8, u8)>, suits: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    if suits.values().len() == 1 {
        Some(values.iter().map(|&v| v.0).collect())
    } else {
        None
    }
}

fn rank_06_full_house(values: &Vec<(u8, u8)>, _: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    let counts: Vec<u8> = values.iter().map(|v| v.1).collect();
    if counts == [3, 2] {
        Some(values.iter().map(|v| v.0).collect())
    } else {
        None
    }
}

fn rank_07_four_of_a_kind(values: &Vec<(u8, u8)>, _: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    let counts: Vec<u8> = values.iter().map(|v| v.1).collect();
    return if counts == [4, 1] {
        Some(values.iter().map(|v| v.0).collect())
    } else {
        None
    };
}

fn rank_08_straigh_flush(values: &Vec<(u8, u8)>, suits: &HashMap<u8, u8>) -> Option<Vec<u8>> {
    return if rank_04_straight(values, suits).is_some() && rank_05_flush(values, suits).is_some() {
        rank_04_straight(values, suits)
    } else {
        None
    };
}
