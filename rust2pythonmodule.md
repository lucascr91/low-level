# Como escrever um módulo do Python em Rust
25/06/2021

## Introdução

Você já deve ter percebido que muitas funções e métodos em Python têm uma performance muito melhor do que funções definidas pelo usuário. Um exemplo é a função `np.sum`, do módulo `numpy`. Veja como essa função se compara com `my_sum`, uma função definida por mim:

```python
def my_sum(numbers:list):
	if len(numbers)>1:
		return numbers[0]+my_sum(numbers[1:])
	else:
		return numbers[0]
```
Usando o módulo `timeit`, eu obtive os seguintes valores na minha máquina para listas com 50, 100 e 200 números:

#### Diferença de performance entre np.sum e função definida pelo usuário

| Tamanho da lista 	| my_sum 	| np.sum 	|
|------------------	|--------	|--------	|
| 50               	| 1.07   	| 0.67   	|
| 100              	| 2.84   	| 0.93   	|
| 200            	| 7.68   	| 1.46   	|

Notas: Valores em segundos nas colunas 2 e 3. O código para replicação do experimento encontra-se no repositório do blog.

Claramente, a performance da função do `numpy` é muito melhor do que minha função. Por que isso acontece? Bom, esse não é um teste rigoroso e muitos fatores podem explicar a diferença de performance, mas seguramente o fato de muitas funções do módulo `numpy` serem escritas em C e minha função ter sido implementada em Python puro é uma das principais causas da diferença observada. Ou seja, é sempre possível melhorar a performance de uma função do Python se escrevermos ela em C. O conceito de "escrever uma função do Python em C" chama-se *extension* e é mais amplo do que a declaração anterior implica, já que podemos usar qualquer linguagem de baixo nível para fazer a mesma coisa, inclusive Rust.

Neste post, vou mostrar como escrever um módulo do Python em Rust. Além desta introdução, o post conta com mais três seções. Na seção seguinte, apresento o algoritmo da Cifra de Vigenère, que servirá de exemplo para nossa implementação. Primeiro escrevo o algoritmo em Python, depois escrevo o mesmo algoritmo em Rust. Com o código do algoritmo em Rust pronto, na terceira seção mostro o passo a passo para implementá-lo como um módulo do Python. A última seção concluí o post e, no melhor espírito do blog, indico minhas dúvidas e questões sobre o tema.

## A cifra de Vigenère

A Cifra de Vigenère é um algoritmo de criptografia que consiste na generalização da Cifra de César. Vou explicar brevemente o conceito da cifra, sem me estender. Uma explicação completa você pode encontrar no capítulo 18 do livro [Cracking Codes with Python](http://inventwithpython.com/cracking/chapter18.html) de onde tirei o código em Python.

A Cifra de César é uma cifra de sustituição simples que consiste em substituir cada letra de uma mensagem por seu equivalente em um alfabeto deslocado por N casas, onde N é a chave da cifra. Por exemplo, meu nome "Lucas" na Cifra César com chave 5 fica "Qzhfx". A Cifra de Vigenère é similar à Chave de César, com a diferença que a chave de cada letra é uma variável dada pelo valor númerico da letra correspondente em uma palavra chave. Parece confuso, mas, novamente, um exemplo mostra como é simples. Meu nome na Cifra de Vigenère com chave PIZZA é "Acbzs". Note que A é a letra que encontramos depois de 15 deslocamentos a partir de "L". A chave 15 foi obtida pois "P" de "PIZZA" é 15ª letra do alfabeto.

Segue implementação do algoritmo de Vigenère em Python:

```python
LETTERS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
 
def translateMessage(key, message, mode):
    translated = []

    keyIndex = 0
    key = key.upper()

    for symbol in message:
        num = LETTERS.find(symbol.upper())
        if num != -1:
            if mode == 'encrypt':
                num += LETTERS.find(key[keyIndex])
            elif mode == 'decrypt':
                num -= LETTERS.find(key[keyIndex])
            num %= len(LETTERS)

            if symbol.isupper():
                translated.append(LETTERS[num])
            elif symbol.islower():
                translated.append(LETTERS[num].lower())

            keyIndex += 1
            if keyIndex == len(key):
                keyIndex = 0
        else:
            translated.append(symbol)

    return ''.join(translated)

myKey = 'PIZZA'
myMessage = "We do not learn, and that what we call learning is only a process of recollection."

myMode = 'encrypt' # Set to either 'encrypt' or 'decrypt'.

translated = translateMessage(myKey, myMessage, myMode)
print(translated)
```

E aqui o equivalente em Rust:

```rust
const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let message: String = String::from("We do not learn, and that what we call learning is only a process of recollection.");
    println!("{:?}", translate_message("PIZZA", &message, "encrypt"))
}

fn translate_message(key: &str, message: &str, mode: &str) -> String {
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
    return result
}
```

## Implementando o código em Rust como um módulo do Python

Para criar o módulo do Python em Rust é preciso primeiro criar uma lib no Rust. Para iniciar a lib, rode:

```bash
cargo new vigen --lib
```

Dei o nome de "vigen" ao nosso módulo, mas pode ter qualquer nome que você desejar.

No arquivo `Cargo.toml` escrevemos:

```
[package]
name = "vigen"
version = "0.1.0"
edition = "2018"

[lib]
name = "vigen"
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.13.2"
features = ["extension-module"]
```

Agora implementamos o código da Cifra de Vigenère em Rust no arquivo `lib.rs`. Mas note que agora o código tem algumas alterações para assegurar que as funções ppossam lidar com input e output do Python:

```rust
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
```

Agora basta navegar até `vigen/` e rodar:

```bash
cargo build --release
```

Para testar nosso módulo basta trocar o nome do arquivo `libvigen.so` para `vigen.so` na pasta `vigen/target/release`, navegar até essa pasta e abrir uma REPL do Python:

```python
from vigen import *
message = "We do not learn, and that what we call learning is only a process of recollection."
translate_message("PIZZA", message, "encrypt")
'Lm cn ndb kdagv, zmd ipzs wwis ve rikk ltiqmico hr octx z pgwbdsh we qerwkkerbhnn.'
```

Funciona perfeitamente!

Uma breve comparação da diferença de performance entre a função usando Python puro e Rust pode ser feita usando o módulo `timeit` novamente. Para isso, crie uma cópia de `vigenere.py` em `vigen/target/release`, remova o que não se relaciona diretamente coma definição da função e rode no REPL:

```python
import vigenere
import vigen
from timeit import timeit
message = "We do not learn, and that what we call learning is only a process of recollection."
params = { 'number' : 100000, 'globals': globals() }
timeit( 'vigen.translate_message("PIZZA", message, "encrypt")', **params)
2.6793487409995578
timeit( 'vigenere.translateMessage("PIZZA", message, "encrypt")', **params)
4.19886307699926
```

Aparentemente, nossa função em Rust é mais de 50% mais rápida que nossa função escrita em Python puro.

## Conclusão

Bom, achei muito divertido escrever esse post e aprendi bastante. Escrever módulos do Python com performance melhor foi uma das principais razões que me incentivaram a estudar *system languages*. Parece que a diversão de verdade ocorre em baixo-nível. Naturalmente, fiquei com muitas dúvidas já que o procedimento como um todo envolve muitos conceitos diferentes. Por exemplo, como faço para usar o pacote fora da pasta `release`? A resposta para essa pergunta parece ser "só colocar o arquivo `vigen.so` na pasta onde quer importar o arquivo". Pode ser, mas parece pouco prático. Outra dúvida é o termo `#[pyfunction]` no código do arquivo `lib.rs`, isto é decorator?

Por hoje é isso. Obrigado pela leitura.




