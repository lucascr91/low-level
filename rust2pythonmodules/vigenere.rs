const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("{:?}", translate_message("PIZZA", "Lucas", "encrypt"))
}

fn translate_message(key: &str, message: &str, mode: &str) -> String {
    let mut translate: Vec<char> = Vec::new();
    let mut keyIndex = 0;
    let mut key = key.to_uppercase();

    for symbol in message.chars() {
        let mut num = LETTERS.find(symbol).unwrap();
        if num>=0 {
            if mode == "encrypt" {
                num+=LETTERS.find(key.chars().nth(keyIndex).unwrap()).unwrap();
            } else if mode == "decrypt" {
                num+=LETTERS.find(key.chars().nth(keyIndex).unwrap()).unwrap();
            }

            if symbol.is_uppercase() {
                translate.push(LETTERS.chars().nth(num).unwrap());
            } else if symbol.is_uppercase() {
                translate.push(LETTERS.chars().nth(num).unwrap().to_lowercase())
            }

            keyIndex+=1;
            if keyIndex == (key.chars().collect::<Vec<char>>().len()) {
                keyIndex = 0;
            }
        } else {
            translate.push(symbol)
        }
    }
    let result: String = translate.into_iter().collect();
    return result
}