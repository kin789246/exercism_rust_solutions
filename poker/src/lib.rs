/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Category {
    FiveOfAKind = 9,
    StraightFlush = 8,
    FourOfAKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1, 
    HighCard = 0,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Diamnds,
    Clubs,
    NotSuit,
}

#[derive(Debug, Eq)]
struct Hand<'a> {
    ranking: Vec<u8>, // [category, key, kick1, kick2, kick3]
    cards: Vec<Card>,
    raw: &'a str,
}

impl<'a> Hand<'a> {
    fn new(input: &'a str) -> Self {
        let mut hand: Self = Self{ 
            ranking: vec![0; 6], 
            cards: Vec::new(),
            raw: input,
        };
        input.split(' ').for_each(|x| {
            hand.cards.push(Card::new(x));
        });
        hand.cards.sort();
        hand.get_category();
        hand
    }

    fn get_category(&mut self) {
        let mut m: HashMap<u8, u8> = HashMap::new();
        self.cards.iter().for_each(|x| {
            m.entry(x.rank).and_modify(|v| *v += 1).or_insert(1);
        });
        if m.keys().count() == 1 {
            self.ranking.push(Category::FiveOfAKind as u8);
        }
        else if m.values().any(|v| *v == 4) {
            for (k, v) in m.iter() {
                if *v == 4 {
                    self.ranking[0] = Category::FourOfAKind as u8;
                    self.ranking[1] = *k;
                }
                if *v == 1 {
                    self.ranking[2] = *k;
                }
            }
        }
        else if m.keys().count() == 5 {
            let mut flush = false;
            let mut straight = true;
            let mut suits: Vec<Suit> = self.cards.iter().map(|c| c.suit).collect();
            suits.dedup();
            if suits.len() == 1 {
                flush = true;
            }
            let ranks: Vec<u8> = self.cards.iter().map(|c| c.rank).collect();
            if ranks == vec![2, 3, 4, 5, 14] {
                self.cards[4].rank = 1;
                self.cards.sort();
            }
            for i in 0..self.cards.len()-1 {
                if self.cards[i].rank + 1 != self.cards[i+1].rank {
                    straight = false;
                    break;
                }
            }
            match (flush, straight) {
                (true, true) => {
                    self.ranking[0] = Category::StraightFlush as u8;
                    self.cards.iter().rev().enumerate()
                        .for_each(|(i, c)| self.ranking[i+1] = c.rank);
                },
                (true, false) => {
                    self.ranking[0] = Category::Flush as u8;
                    self.cards.iter().rev().enumerate()
                        .for_each(|(i, c)| self.ranking[i+1] = c.rank);
                },
                (false, true) => {
                    self.ranking[0] = Category::Straight as u8;
                    self.cards.iter().rev().enumerate()
                        .for_each(|(i, c)| self.ranking[i+1] = c.rank);
                },
                (false, false) => {
                    self.ranking[0] = Category::HighCard as u8;
                    self.cards.iter().rev().enumerate()
                        .for_each(|(i, c)| self.ranking[i+1] = c.rank);
                }
            }
        }
        else if m.keys().count() == 2 {
            for (k, v) in m.iter() {
                if *v == 3 {
                    self.ranking[0] = Category::FullHouse as u8;
                    self.ranking[1] = *k;
                }
                if *v == 2 {
                    self.ranking[2] = *k;
                }
            }
        }
        else if m.keys().count() == 3 {
            if m.values().any(|v| *v == 3) {
                for (k, v) in m.iter() {
                    if *v == 3 {
                        self.ranking[0] = Category::ThreeOfAKind as u8;
                        self.ranking[1] = *k;
                    }
                }
                self.ranking[2] = self.cards.iter()
                    .filter(|c| c.rank != self.ranking[1]).max().unwrap().rank;
                self.ranking[3] = self.cards.iter()
                    .filter(|c| c.rank != self.ranking[1]).min().unwrap().rank;
            }
            else {
                self.ranking[0] = Category::TwoPair as u8;
                self.ranking[1] = *m.iter().filter(|(&_k, &v)| v == 2).max().unwrap().0;
                self.ranking[2] = *m.iter().filter(|(&_k, &v)| v == 2).min().unwrap().0;
                self.ranking[3] = self.cards.iter()
                    .filter(|c| c.rank != self.ranking[1] && c.rank != self.ranking[2])
                    .nth(0).unwrap().rank;
            }
        }
        else if m.keys().count() == 4 {
            self.ranking[0] = Category::OnePair as u8;
            for (k, v) in m.iter() {
                if *v == 2 {
                    self.ranking[1] = *k;
                    break;
                }
            }
            let mut rest: Vec<u8> = Vec::new();
                self.cards.iter().for_each(|c| {
                    if c.rank != self.ranking[1] {
                        rest.push(c.rank);
                    }
                });
            rest.sort();
            let mut i: usize = 2;
            for v in rest.iter() {
                self.ranking[i] = *v;
                i += 1;
            }
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        //let mut highest: usize = 0;
        for i in 0..6 {
            if self.ranking[i] != other.ranking[i] {
                return self.ranking[i].cmp(&other.ranking[i]);
                //highest = i;
                //break;
            }
        }
        Ordering::Equal
        //self.ranking[highest].cmp(&other.ranking[highest])
    }
}

#[derive(Debug, Eq, Clone, Copy)]
struct Card {
    rank: u8,
    suit: Suit,    
}

impl Card {
    fn new(input: &str) -> Self {
        let s: Suit;
        let mut c = &input[0..input.len()-1];
            match c {
            "A" => c = "14",
            "J" => c = "11",
            "Q" => c = "12",
            "K" => c = "13",
            _ => c = c
        }
        let c2 = input.chars().nth(input.len()-1).unwrap();
        match c2 {
            'S' => s = Suit::Spades,
            'H' => s = Suit::Hearts,
            'D' => s = Suit::Diamnds,
            'C' => s = Suit::Clubs,
            _ => s = Suit::NotSuit,
        }
        Self { rank: c.parse().unwrap(), suit: s }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut to_hands: Vec<Hand> = Vec::new();
    for hand in hands {
        to_hands.push(Hand::new(hand));
    }
    to_hands.sort();
    let mut ans: Vec<&'a str> = Vec::new();
    ans.push(to_hands[to_hands.len()-1].raw);
    for i in 1..to_hands.len() {
        if to_hands[i] == to_hands[to_hands.len()-1] {
            ans.push(to_hands[i].raw);
        }
    }

    ans
}
