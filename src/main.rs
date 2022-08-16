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

#[derive(Debug)]
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
        println!("Top card is {}", self.cards[self.cards.len()-1].reveal());
    }

    fn shuffle_deck(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn draw_card(&mut self, player_hand: &mut Vec<Card>) {
        self.shuffle_deck();
        let drawn_card = self.cards.pop();
        match drawn_card {
            None => panic!("no more cards left!"),
            Some(card) => player_hand.push(card),
        };
    }
}

struct GinGame {
    first_player_hand: Vec<Card>,
    second_player_hand: Vec<Card>,
    deck: Deck,
    discard_pile: Vec<Card>,
}

impl GinGame {
    fn new() -> Self {
        let mut deck = Deck::create();
        let discard_pile = Vec::new();
        let first_player_hand = Vec::new();
        let second_player_hand = Vec::new();



        GinGame { first_player_hand, second_player_hand, deck, discard_pile}
    }
}

fn main() {
    let mut game = GinGame::new();
    game.deck.draw_card(&mut game.first_player_hand);

    println!("{:?}", game.first_player_hand);
    println!("{:?}", game.deck.cards.len());

}
