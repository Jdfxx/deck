
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

fn main() {
    
    //list of suits
    //list of values

    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["One", "Two", "Three", "Four"];
    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    
    let deck = Deck {cards};
    println!("Here is your deck: {:#?}", deck);
}
