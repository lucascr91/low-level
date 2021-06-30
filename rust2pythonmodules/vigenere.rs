const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let message: String = String::from("We do not learn, and that what we call learning is only a process of recollection.");
    println!("{:?}", translate("PIZZA", &message, "encrypt"))
}

fn translate(key: &str, message: &str, mode: &str) -> String {
    let mut translate: Vec<char> = Vec::new();
    let mut key_index = 0;
    let key = key.to_lowercase();
    let message = message.to_lowercase();

    for symbol in message.chars() {
        let index_letter = LETTERS.find(symbol);
        match index_letter {
            Some(x) => {
                let mut num:i32 = x as i32;
                if mode == "encrypt" {
                    num+=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap() as i32;
                } else if mode == "decrypt" {
                    num-=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap() as i32;
                }
                num= (num).rem_euclid(LETTERS.chars().collect::<Vec<char>>().len() as i32);
                translate.push(LETTERS.chars().nth(num as usize).unwrap());
                key_index+=1;
                if key_index == (key.chars().collect::<Vec<char>>().len()) {
                    key_index = 0;
                }
            }
            None =>  {
                translate.push(symbol)
            }
        }
    }
    let result: String = translate.into_iter().collect();
    result
}