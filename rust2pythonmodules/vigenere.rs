const LETTERS: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("{}", translate_message(String::from("PIZZA"), String::from("Lucas"), String::from("encrypt")))
}

fn translate_message(key: String, message: String, mode String) {
    let mut translate: Vec<String> = Vec::new();
    let mut keyIndex = 0;
    let mut key = key.make_ascii_uppercase();

    for symbol in message.chars() {
        let mut num = LETTERS.find(symbol);
        if num.is_some() {
            if mode == 'encrypt' {
                num+=LETTERS.find(key[keyIndex]).unwrap();
            } else if {
                num+=LETTERS.find(key[keyIndex]).unwrap();
            }
        }
        if 
    }

}