const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("{:?}", translate_message("PIZZA", "Lucas", "encrypt"))
}

fn translate_message(key: &str, message: &str, mode: &str) -> String {
    let mut translate: Vec<char> = Vec::new();
    let mut key_index = 0;
    let key = key.to_uppercase();
    let message = message.to_uppercase();

    for symbol in message.chars() {
        let mut num = LETTERS.find(symbol).unwrap();
        if num>=0 {
            if mode == "encrypt" {
                num+=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap();
            } else if mode == "decrypt" {
                num-=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap();
            }

            if symbol.is_uppercase() {
                translate.push(LETTERS.chars().nth(num).unwrap());
            } else if symbol.is_lowercase() {
                translate.push(LETTERS.chars().nth(num).unwrap().to_lowercase().collect::<Vec<char>>()[0])
            }

            key_index+=1;
            if key_index == (key.chars().collect::<Vec<char>>().len()) {
                key_index = 0;
            }
        } else {
            translate.push(symbol)
        }
    }
    let result: String = translate.into_iter().collect();
    return result
}