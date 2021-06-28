# O que significa unwrap em Rust?

26/06/2021

Quando comecei a aprender Rust uma das coisas que achava mais estranhas era o método `unwrap`. Eu via isso em todos os códigos e simplesmente não sabia o que significava. A traduação de "wrap" é embrulho, então "unwrap" é algo como "desembrulhar". Desembrulhar o que, oras? Bom, para entender esse método é preciso entender os conceitos de `Enum` e *pattern matching* em Rust. Claro, um tratamento completo de cada um desses conceitos não caberia aqui. Por essa razão mostro apenas o que é necessário para entendermos o `unwrap`.

Vamos falar então breve e resumidamente sobre esses conceitos e, ao final do post, a gente volta no `unwrap` para entender o que, afinal, estamos desembrulhando.

## Enums e *pattern matching*

Enumeráveis são chamados simplesmente de `Enums` em Rust e são estruturas que permitem o usuário definir um tipo de objeto enumerando suas variantes. Por exemplo, podemos criar um Enum chamado State em que cada variante é um estado que já morei na vida:

```rust
enum State {
    minasgerais,
    saopaulo,
}
```

Enums têm muitas propriedades e um tratamento completo pode ser encontrado [aqui](https://doc.rust-lang.org/book/ch06-00-enums.html). Para os propósitos desse post o importante é entender como os Enums se relacionam com um *flow operator* do Rust chamado de *match*. No contexto de enumeráveis, o *match* roda um código com base em qual variante do Enum foi observada. Por exemplo, posso criar uma função que retorna quantos anos eu morei em cada um dos estados do Enum State:

```rust
fn years(state: State) -> i32 {
    match state {
        minasgerais => 26,
        saopaulo => 4
    }
}
```

Dois dos mais importantes eEnums em Rust são o `Result<T,E>` e o `Option<T>`. O `Result` tem as variantes `Ok(T)`, que é o resultado de uma função se ela roda com sucesso, e `Err(E)` que é um erro, ou seja, o resultado da função quando ela não tem sucesso. O `Option`, por sua vez, é um Enum que retorna o valor `Some(T)` se a função retorna um objeto e `None`, caso não retorne nada. Em ambos os casos, a letra `T` simboliza o tipo da variável que está retornando: `i32`, `f64`, `String`, etc.

Essa é a assinatura do Enum `Result`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

E essa é a assinatura do `Option`:

```
enum Option<T> {
    Some(T),
    None,
}
```

Note que, assim como em `State`, podemos usar o *match* para criar uma função que nos retorne um valor para cada variante de `Result` e outra função que retorne um valor para cada variante do `Option`. 

No caso do `Result`, se a função rodar corretamente, gostaríamos de obter o valor correspondente ao resultado e, caso contrário, gostaríamos de obter a mensagem de erro e interromper o programa. 

Vamos ilustar esse exemplo do `Result` com o método `from_utf8` que converte um vetor de bytes em uma `String`. Como pode ser visto na [documentação](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8), esse método retorna um Enum do tipo Result. Nossa função para usar o método `from_utf8` chama-se `get_value` e é definida abaixo:

```rust
// esse programa roda corretamente. Experimente alterar os valores do vetor para ver o que acontece
fn main() {
    let example = vec![240, 159, 146, 150];
    println!("{}", get_value(example))
}

fn get_value(emoji: Vec<u8>) -> String {
    let heart = String::from_utf8(emoji);
    match heart {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    }
}
```

Nossa função `get_value` irá retornar uma string quando o código do emoji corresponder a um caracter e um erro, caso contrário.

É possível fazer um exemplo análogo para o `Option`. Considere a função `from_u32` que, a partir de um valor numérico, retorna o caracter correspndente em Unicode. Como pode ser vistos na [documentação](https://docs.rs/rustc-std-workspace-std/1.0.1/std/char/fn.from_u32.html) essa função retorna

Ora, se uma função que retorna um Enum do tipo Result sempre precisa desse tipo de "tratamento" antes que a gente possa finalmente acessar o valor, não seria melhor ter isso pronto na forma de um método? Como você já deve ter advinhado, isso é exatamente o que o método `unwrap` faz. Usando o `unwrap`, o código anterior poder ser escrito de forma muito mais simples:

```rust
fn main() {
    let example = vec![240, 159, 146, 150];
    let heart = String::from_utf8(example).unwrap();
    prinln!("{}", heart)
}
```

## Conclusão

Então é isso: o `unwrap` é um método para desembrulhar um resultado que poderia ser um erro. E o que acontece quando por trás do embrulho existe um erro? Diferentemente da nossa função `get_value`, o `unwrap`, através da macro `panic!`, chama um erro irrecuperável quando encontra um erro e, assim, interrompe a compilação.



