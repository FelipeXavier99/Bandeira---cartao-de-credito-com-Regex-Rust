use std::io;

fn get_card_flag(card_number: &str) -> Option<&str> {
    let card_number = card_number.chars().filter(|c| c.is_numeric()).collect::<String>();

    let cards = [
        ("visa", r"^4[0-9]{12}(?:[0-9]{3})"),
        ("mastercard", r"^5[1-5][0-9]{14}"),
        ("diners", r"^3(?:0[0-5]|[68][0-9])[0-9]{11}"),
        ("amex", r"^3[47][0-9]{13}"),
        ("discover", r"^6(?:011|5[0-9]{2})[0-9]{12}$"),
        ("hipercard", r"^(606282\d{10}(\d{3})?)|(3841\d{15})"),
        ("elo", r"^((((636368)|(438935)|(504175)|(451416)|(636297))\d{0,10})|((5067)|(4576)|(4011))\d{0,12})"),
        ("jcb", r"^(?:2131|1800|35\d{3})\d{11}"),
        ("aura", r"^50[0-9]{14}$")
    ];

 

    for &(flag, regex) in cards.iter() {
        let re = regex::Regex::new(regex).unwrap();
        if re.is_match(&card_number) {
            println!("Card Flag: {}", flag);
            return Some(flag);
        }
    }

    println!("Card Flag not found");
    None
}

fn main() {
    println!("Enter your card number: ");
    let mut card_number = String::new();

    io::stdin()
        .read_line(&mut card_number)
        .expect("Failed to read input");

    if let Some(flag) = get_card_flag(&card_number) {
        
        println!("Card Flag: {}", flag);
    } else {
        println!("Card Flag not found");
    }
}
