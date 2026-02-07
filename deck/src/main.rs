
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
        Deck { cards }
    }

    // Method to shuffle the deck of cards, which will be implemented in the future.
    fn shuffle(&self){

    }
}


fn main() {
    // Create a new deck of cards using the Deck::new() method, which initializes the deck with all 52 standard playing cards.
    let deck = Deck::new();
    
    println!("Heres your deck: {:#?}", deck); // adding # to the debug format specifier will print the deck in a more readable, pretty-printed format.
}
