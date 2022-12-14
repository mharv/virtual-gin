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

    fn peek_two(&self) -> (&Card, &Card) {
        (&self.cards[self.cards.len()-1], &self.cards[self.cards.len()-2])
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

struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    fn new(name: String) -> Self {
        Player { name, hand: Vec::new() }
    }

    fn display_player_hand(&self) {
        println!("{}'s hand: ", self.name);
        for card in self.hand.iter() {
            println!("{}", card.reveal());
        }
        println!(" ");
    }
}

struct GinGame {
    first_player: Player,
    second_player: Player,
    deck: Deck,
    discard_pile: Vec<Card>,
    current_turn: String,
}

impl GinGame {
    fn new(first_player_name: String, second_player_name: String) -> Self {
        let deck = Deck::create();
        let discard_pile = Vec::new();
        let first_player = Player::new(first_player_name);
        let second_player = Player::new(second_player_name);
        let current_turn = String::from("");
        GinGame { first_player, second_player, deck, discard_pile, current_turn }
    }

    fn deal_starting_hands(&mut self) {
        self.deck.shuffle_deck();
        for _ in 0..10 {
            self.deck.draw_card(&mut self.first_player.hand);
            self.deck.draw_card(&mut self.second_player.hand);
        }
        self.deck.draw_card(& mut self.discard_pile);
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

    fn decide_first_turn(&mut self) {
        loop {
            self.deck.shuffle_deck();

            let (first_player_card, second_player_card) = self.deck.peek_two();

            println!("{}'s card is {}", self.first_player.name, first_player_card.reveal());
            println!("{}'s card is {}", self.second_player.name, second_player_card.reveal());

            if RANK_VALUES.get(&first_player_card.rank) == RANK_VALUES.get(&second_player_card.rank) {
                println!("Draw again!");
                println!(" ");
            }

            if RANK_VALUES.get(&first_player_card.rank) > RANK_VALUES.get(&second_player_card.rank) {
                println!("{} goes first.", self.first_player.name);
                self.current_turn = self.first_player.name.clone();
                break;
            }
            if RANK_VALUES.get(&first_player_card.rank) < RANK_VALUES.get(&second_player_card.rank) {
                println!("{} goes first.", self.second_player.name);
                self.current_turn = self.second_player.name.clone();
                break;
            }
        }
        println!(" ");
    }
}

fn main() {
    let mut game = GinGame::new(String::from("Mitch"), String::from("Phoebe"));
    game.decide_first_turn();
    game.deal_starting_hands();

    game.first_player.display_player_hand();
    game.second_player.display_player_hand();
    game.display_discard_pile();


}
