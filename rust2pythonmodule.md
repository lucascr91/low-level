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
| 10000            	| 7.68   	| 1.46   	|

Notas: Valores em segundos nas colunas 2 e 3. O código para replicação do experimento encontra-se no repositório do blog.

Claramente, a performance da função do `numpy` é muito melhor do que minha função. Por que isso acontece? Bom, esse não é um teste rigoroso e muitos fatores podem explicar a diferença de performance, mas seguramente o fato de muitas funções do módulo `numpy` serem escritas em C e minha função ter sido implementada em Python puro é uma das principais causas da diferença observada. Ou seja, é sempre possível melhorar a performance de uma função do Python se escrevermos ela em C. O conceito de "escrever uma função do Python em C" chama-se *extension* e é mais amplo do que a declaração anterior implica, já que podemos usar qualquer linguagem de baixo nível para fazer a mesma coisa, inclusive Rust.

Neste post, vou mostrar como escrever um módulo do Python em Rust. Além desta introdução, o post conta com mais três seções. Na seção seguinte, apresento o algoritmo da Cifra de Vigenère, que servirá de exemplo para nossa implementação. Primeiro escrevo o algoritmo em Python, depois escrevo o mesmo algoritmo em Rust. Com o código do algoritmo em Rust pronto, na terceira seção mostro o passo a passo para implementá-lo a partir de um módulo do Python. A última seção concluí o post e, no melhor espírito do blog, indico minhas dúvidas e questões sobre o tema.

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

### Continua ...