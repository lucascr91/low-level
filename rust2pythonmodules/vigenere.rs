const LETTERS: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("{}", translate_message(String::from("PIZZA"), String::from("Lucas"), String::from("encrypt")))
}

fn translate_message(key: String, message: String, mode String) {
    let mut translate: Vec<String> = Vec::new();
    let mut keyIndex = 0;
    let mut key = key.make_ascii_uppercase();

    for symbol in message.chars() {
        assert!(LETTERS.find(symbol).is_some(), "Cannot find {}", symbol)
        let mut num = LETTERS.find(symbol).unwrap();
    }

}