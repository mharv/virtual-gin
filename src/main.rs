use std::fmt::Error;
use std::io;

use phf::phf_map;
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;

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

const SUITS: [&str; 4] = ["Clubs", "Spades", "Diamonds", "Hearts"];

const RANKS: [&str; 13] = [
    "Ace", "King", "Queen", "Jack", "Ten", "Nine", "Eight", "Seven", "Six", "Five", "Four",
    "Three", "Two",
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

struct DiscardPile {
    cards: Vec<Card>,
}

impl DiscardPile {
    fn create() -> Self {
        DiscardPile { cards: Vec::new() }
    }

    fn draw_card(&mut self, destination: &mut Vec<Card>) {
        let drawn_card = self.cards.pop();
        match drawn_card {
            Some(card) => destination.push(card),
            None => panic!("no more cards left!"),
        };
    }

    fn discard_card(&mut self, origin: &mut Vec<Card>, card_index: usize) {
        let card = origin.remove(card_index);
        self.cards.push(card);
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
                cards.push(Card { suit, rank });
            }
        }
        Deck { cards }
    }

    fn peek_two(&self) -> (&Card, &Card) {
        (
            &self.cards[self.cards.len() - 1],
            &self.cards[self.cards.len() - 2],
        )
    }

    fn shuffle_deck(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn draw_card(&mut self, destination: &mut Vec<Card>) {
        let drawn_card = self.cards.pop();
        match drawn_card {
            Some(card) => destination.push(card),
            None => panic!("no more cards left!"),
        };
    }
}

struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            hand: Vec::new(),
        }
    }

    fn display_player_hand(&self) {
        println!("{}'s hand: ", self.name);
        for (index, card) in self.hand.iter().enumerate() {
            println!("{} - {}", index, card.reveal());
        }
        println!(" ");
    }
}

struct GinGame {
    first_player: Player,
    second_player: Player,
    deck: Deck,
    discard_pile: DiscardPile,
    current_turn: String,
    knock_status: bool,
    gin_status: bool,
}

impl GinGame {
    fn new(first_player_name: String, second_player_name: String) -> Self {
        let deck = Deck::create();
        let discard_pile = DiscardPile::create();
        let first_player = Player::new(first_player_name);
        let second_player = Player::new(second_player_name);
        let current_turn = String::from("");
        GinGame {
            first_player,
            second_player,
            deck,
            discard_pile,
            current_turn,
            knock_status: false,
            gin_status: false,
        }
    }

    fn deal_starting_hands(&mut self) {
        self.deck.shuffle_deck();
        for _ in 0..10 {
            self.deck.draw_card(&mut self.first_player.hand);
            self.deck.draw_card(&mut self.second_player.hand);
        }
        self.deck.draw_card(&mut self.discard_pile.cards);
    }

    fn display_discard_pile(&self) {
        println!("Top card of discard pile: ");
        if self.discard_pile.cards.len() == 0 {
            println!("Discard pile is empty!");
        } else {
            println!(
                "{}",
                self.discard_pile.cards[self.discard_pile.cards.len() - 1].reveal()
            );
        }
        println!(" ");
    }

    fn get_current_turn(&mut self) -> &str {
        &self.current_turn
    }

    fn set_next_turn(&mut self) {
        if self.current_turn == self.first_player.name {
            self.current_turn = self.second_player.name.clone();
        } else {
            self.current_turn = self.first_player.name.clone();
        }
    }

    fn awaiting_draw(&mut self) {
        println!("either draw a card from the deck (d1) or draw a card from the discard pile (d2) awaiting input...");
        let mut input = String::new();

        // should the below be something to do with state?
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            // need to put in get current player logic
            match input.trim() {
                "d1" => {
                    if self.current_turn == self.first_player.name {
                        self.deck.draw_card(&mut self.first_player.hand);
                        self.first_player.display_player_hand();
                    } else {
                        self.deck.draw_card(&mut self.second_player.hand);
                        self.second_player.display_player_hand();
                    }
                    break;
                }
                "d2" => {
                    if self.current_turn == self.first_player.name {
                        self.discard_pile.draw_card(&mut self.first_player.hand);
                        self.first_player.display_player_hand();
                    } else {
                        self.discard_pile.draw_card(&mut self.second_player.hand);
                        self.second_player.display_player_hand();
                    }
                    break;
                }
                _ => println!("Invalid command."),
            }
        }
    }

    fn awaiting_discard(&mut self) {
        println!("decide which card you want to discard by typing \"d-N\" where is N is the number next to the card.");
        //show hand of player who has current turn
        let re = Regex::new(r"^d-\d{1,2}$").unwrap();

        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            if !re.is_match(&input.trim()) {
                println!("Invalid command. command is in wrong format");
                continue;
            }

            let number = input.trim().split("-").collect::<Vec<&str>>()[1];
            let number: usize = number.parse().unwrap();
            if number > 10 {
                println!("Invalid command. number too high");
                continue;
            }

            match number {
                number => {
                    let fp_name = self.first_player.name.clone();
                    if self.get_current_turn() == fp_name {
                        self.discard_pile
                            .discard_card(&mut self.first_player.hand, number);
                        self.display_discard_pile();
                        break;
                    } else {
                        self.discard_pile
                            .discard_card(&mut self.second_player.hand, number);
                        self.display_discard_pile();
                        break;
                    }
                }
            }
        }
    }

    fn awaiting_decision(&mut self) {
        println!("would you like to knock (K), call gin (G) or neither (N)?");

        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "K" => {
                    println!("player knocked");
                    self.knock_status = true;
                    break;
                }
                "G" => {
                    println!("player called gin");
                    self.gin_status = true;
                    break;
                }
                "N" => {
                    println!("player did not knock");
                    break;
                }
                _ => println!("invalid command."),
            }
        }
    }

    fn decide_first_turn(&mut self) {
        loop {
            self.deck.shuffle_deck();

            let (first_player_card, second_player_card) = self.deck.peek_two();

            println!(
                "{}'s card is {}",
                self.first_player.name,
                first_player_card.reveal()
            );
            println!(
                "{}'s card is {}",
                self.second_player.name,
                second_player_card.reveal()
            );

            if RANK_VALUES.get(&first_player_card.rank) == RANK_VALUES.get(&second_player_card.rank)
            {
                println!("Draw again!");
                println!(" ");
            }

            if RANK_VALUES.get(&first_player_card.rank) > RANK_VALUES.get(&second_player_card.rank)
            {
                println!("{} goes first.", self.first_player.name);
                self.current_turn = self.first_player.name.clone();
                break;
            }
            if RANK_VALUES.get(&first_player_card.rank) < RANK_VALUES.get(&second_player_card.rank)
            {
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

    while !game.knock_status && !game.gin_status {
        println!("{}", game.get_current_turn());
        game.awaiting_draw();
        game.awaiting_discard();
        game.awaiting_decision();
        game.set_next_turn();
    }

    // if gin, count deadwood of other player,
    // add up and score +20 
    //
    // if knock, deadwood is compared
    // if knocking player’s Deadwood value is equal to or greater than their 
    // opponent's Deadwood value, they have been Undercut, Opponent gets 
    // difference +10
    //
    // if deadwood is lower, knocking player gets difference +10
    //
    //
    // record scores, do a match check for a winner.
}
