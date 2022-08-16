use rand::thread_rng;
use rand::seq::SliceRandom;
use phf::phf_map;

static RANK_VALUES: phf::Map<&'static str, i32> = phf_map! {
    "Ace" => 1,
    "King"=> 13,
    "Queen" => 12,
    "Jack" => 11,
    "Ten" => 10,
    "Nine" => 9,
    "Eight" => 8,
    "Seven" => 7,
    "Six" => 6,
    "Five" => 5,
    "Four" => 4,
    "Three" => 3,
    "Two" => 2,
};

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

    fn peek_two(&mut self) {
        self.shuffle_deck();
        let mut fp_index_offset = 1;
        let mut sp_index_offset = 2;

        let first_player_card = &self.cards[self.cards.len()-fp_index_offset];
        let second_player_card = &self.cards[self.cards.len()-sp_index_offset];

        println!("First player's card is {}", first_player_card.reveal());
        println!("Second player's card is {}", second_player_card.reveal());

        if RANK_VALUES.get(&first_player_card.rank) == RANK_VALUES.get(&second_player_card.rank) {
            println!("Draw again!");
        }
        if RANK_VALUES.get(&first_player_card.rank) > RANK_VALUES.get(&second_player_card.rank) {
            println!("First player goes first.");
        } else {
            println!("Second player goes first.");
        }
    }

    fn shuffle_deck(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn draw_card(&mut self, destination: &mut Vec<Card>) {
        self.shuffle_deck();
        let drawn_card = self.cards.pop();
        match drawn_card {
            None => panic!("no more cards left!"),
            Some(card) => destination.push(card),
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
        let deck = Deck::create();
        let discard_pile = Vec::new();
        let first_player_hand = Vec::new();
        let second_player_hand = Vec::new();
        GinGame { first_player_hand, second_player_hand, deck, discard_pile}
    }

    fn deal_starting_hands(&mut self) {
        self.deck.shuffle_deck();
        for _ in 0..10 {
            self.deck.draw_card(&mut self.first_player_hand);
            self.deck.draw_card(&mut self.second_player_hand);
        }
        self.deck.draw_card(& mut self.discard_pile);
    }

    fn display_first_player_hand(&self) {
        println!("First player's hand: ");
        for card in self.first_player_hand.iter() {
            println!("{}", card.reveal());
        }
        println!(" ");
    }

    fn display_second_player_hand(&self) {
        println!("Second player's hand: ");
        for card in self.second_player_hand.iter() {
            println!("{}", card.reveal());
        }
        println!(" ");
    }

    fn display_discard_pile(&self) {
        println!("Top card of discard pile: ");
        if self.discard_pile.len() == 0 {
            println!("Discard pile is empty!");
        } else {
            println!("{}", self.discard_pile[self.discard_pile.len()-1].reveal());
        }
        println!(" ");
    }
}

fn main() {
    let mut game = GinGame::new();
    game.deck.peek_two();
    game.deal_starting_hands();

    println!("{:?}", game.deck.cards.len());

    game.display_first_player_hand();
    game.display_second_player_hand();
    game.display_discard_pile();


}
