use rand::rng; 
use rand::random;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;

// or we can also add like 
// use rand::{thread_rng, random, rngs::OsRng};


// A simple struct representing a deck of cards
// Derive Debug trait for easy printing, which creates 
// an implementation of the Debug trait for the Deck struct, 
// allowing us to print instances of Deck using the {:?} format specifier.
#[derive(Debug)] 
struct Deck {
    cards: Vec<String>,
}

// Implementing methods for the Deck struct
impl Deck {

    // Associated function to create a new deck of cards.
    // The 'new' function is an associated function (not a method) because it does not take a reference to self.
    fn new() -> Self {
        // List of 'suits' - 'hearts', 'diamonds', 'clubs', 'spades'
        // Mentioning the type of bindings explicitly is optional
        let suits = ["hearts", "diamonds", "clubs", "spades"];

        // List of 'values' - '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'
        let values = ["two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen", "king", "ace"];
    
        // In Rust, bindings are immutable by default, so we need to declare 'cards' 
        // as mutable using 'mut' to allow us to push new cards into the vector.
        let mut cards = vec![]; 
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        // Return a new instance of Deck with the generated cards.
        // It is called as implicit return, where the last expression in the function is returned without needing an explicit 'return' statement.
        // Important thing is that, it should not be followed by a semicolon, otherwise it will turn the expression into a statement and 
        // return unit type '()' instead of the Deck instance.
        Deck { cards }
    }

    // Method to shuffle the deck of cards, which will be implemented in the future.
    fn shuffle(&mut self){
        let mut rng = rng(); 
        // This method uses the `shuffle()` function from the `rand::seq::SliceRandom` trait, which provides
        // in-place shuffling for any slice type (including the `Vec` slice). The `SliceRandom` trait is imported
        // via `use rand::seq::SliceRandom`, making the `shuffle()` method available on vector slices.
        // The method uses `thread_rng()` as the random number generator source.
        self.cards.shuffle(&mut rng);
    }

    // Method to deal a specified number of cards from the deck, which will be implemented in the future.
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        if num_cards > self.cards.len() {
            panic!("Not enough cards in the deck to deal {} cards", num_cards);            
        }
        // This method uses the `split_off()` method of the `Vec` type to split the vector of cards into two parts:
        // The `split_off()` method takes an index as an argument and splits the vector at
        // that index, returning a new vector containing the elements from the split point to the end of the original vector.
        self.cards.split_off(self.cards.len() - num_cards)
    }
}


fn main() {
    // Create a new deck of cards using the Deck::new() method, which initializes the deck with all 52 standard playing cards.
    let mut deck = Deck::new();

    deck.shuffle();
    
    // Create a random number generator using the thread_rng function from the rand crate, which provides a random number generator 
    // that is local to the current thread and seeded by the operating system.
    // For external modules, we dont need to import headers like in C/C++. We can directly use the functions provided by the 
    // rand module after adding the dependency in Cargo.toml file. 
    // let rng = rand::thread_rng();

    let cards = deck.deal(5); // Dealing 5 cards from the deck using the deal method
    println!("Heres your deck: {:#?}", deck); // adding # to the debug format specifier will print the deck in a more readable, pretty-printed format.
}
