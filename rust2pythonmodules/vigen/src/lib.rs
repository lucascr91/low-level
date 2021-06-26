use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[pyfunction]
fn translate_message(key: &str, message: &str, mode: &str) -> PyResult<String> {
        let mut translate: Vec<char> = Vec::new();
        let mut key_index = 0;
        let key = key.to_uppercase();
    
        for symbol in message.chars() {
            let index_letter = LETTERS.find(symbol.to_uppercase().collect::<Vec<_>>()[0]);
            match index_letter {
                Some(mut num) => {
                    // num = num.wrap();
                    if mode == "encrypt" {
                        num+=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap();
                    } else if mode == "decrypt" {
                        num-=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap();
                    }
                    num%= LETTERS.chars().collect::<Vec<char>>().len();
        
                    if symbol.is_uppercase() {
                        translate.push(LETTERS.chars().nth(num).unwrap());
                    } else if symbol.is_lowercase() {
                        translate.push(LETTERS.chars().nth(num).unwrap().to_lowercase().collect::<Vec<char>>()[0])
                    }
        
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
        Ok(result)
}

#[pymodule]
fn vigen(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(translate_message, m)?)?;

    Ok(())
}