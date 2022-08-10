use rand::thread_rng;
use rand::seq::SliceRandom;

const SUITS: [&str; 4] = [
    "Clubs",
    "Spades",
    "Diamonds",
    "Hearts",
];

const RANKS: [&str; 13] = [
    "Ace",
    "King",
    "Queen",
    "Jack",
    "Ten",
    "Nine",
    "Eight",
    "Seven",
    "Six",
    "Five",
    "Four",
    "Three",
    "Two",
];

struct Card {
    suit: String,
    rank: String,
}

impl Card {
    fn reveal(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn create() -> Self {
        let mut cards = Vec::new();
        for suit in SUITS {
            for rank in RANKS {
                let suit = String::from(suit);
                let rank = String::from(rank);
                cards.push(Card { suit , rank });
            }
        }
        Deck { cards }
    }

    fn peek_top(&self) {
        println!("Top card is {}", self.cards[0].reveal());
    }

    fn shuffle_deck(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

fn main() {
    let mut deck = Deck::create();

    println!("Deck has {} many cards.", deck.cards.len());
    deck.peek_top();
    deck.shuffle_deck();
    deck.peek_top();
}
