#[derive(Debug)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

fn print_suit(suit: &Suit) {
    match suit {
        Suit::Spades => println!("The suit is Spades"),
        Suit::Hearts => println!("The suit is Hearts"),
        Suit::Diamonds => println!("The suit is Diamonds"),
        Suit::Clubs => println!("The suit is Clubs"),
    }
}

enum Card {
    Ace(Suit),
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Number(u8, Suit),
}

impl Card {
    fn display(&self) -> String {
        match self {
            Card::Ace(suit) => format!("Ace of {:?}", suit),
            Card::King(suit) => format!("King of {:?}", suit),
            Card::Queen(suit) => format!("Queen of {:?}", suit),
            Card::Jack(suit) => format!("Jack of {:?}", suit),
            Card::Number(number, suit) => format!("{} of {:?}", number,
                                                  suit),
        }
    }
}

fn print_card(card: &Card){
    match card {
        Card::Ace(suit) => println!("The card is an Ace of {:?}", suit),
        Card::King(suit) => println!("The card is a King of {:?}", suit),
        Card::Queen(suit) => println!("The card is a Queen of {:?}", suit),
        Card::Jack(suit) => println!("The card is a Jack of {:?}", suit),
        Card::Number(number, suit) => println!("The card is a {} of {:?}", number, suit),
    }
}

fn main() {
    let my_suit: Suit = Suit::Spades;

    let my_card = Card::Ace(Suit::Spades);
    let another_card = Card::Number(7, Suit::Spades);

    print_suit(&my_suit);
    print_card(&my_card);

    let card_description = my_card.display();
    println!("{}", card_description);

    let r;
    let r =
    {
        let mut x = 5;
        r = &x;
        *r
    };
    println!("r: {}", r);
}
