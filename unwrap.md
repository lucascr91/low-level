# O que significa unwrap em Rust?

26/06/2021

Quando comecei a aprender Rust uma das coisas que achava mais estranhas era o método `unwrap`. Eu via isso em todos os códigos e simplesmente não sabia o que significava. A traduação de "wrap" é embrulho, então "unwrap" é algo como "desembrulhar". Desembrulhar o que oras? Bom, para entender esse método é preciso entender o conceito de *error handling* in Rust, vamos fazer um tour agora sobre esse conceito e, ao final do post, a gente volta no `unwrap` para entender o que, afinal, estamos desembrulhando.

## Error handling

A primeira coisa a entender sobre erros em Rust é que existem dois tipos de erros: recuperáveis e não-recuperáveis. Segue uma tabela resumindo a diferença entre os dois:

|            	| Recuperável 	| Não-Recuperável 	|
|------------	|-------------	|-----------------	|
| tipo/macro 	| Result<T,E> 	|      panic!     	|
| Interrompe 	| run time    	| compile time    	|

O método `unwrap` é usado no contexto de erros recuperáveis, que aplicam o enumerável `Result<T,E>`. Enumeráveis são chamados simplesmente de `Enums` em Rust e são estruturas que permitem o usuário definir um tipo de objeto enumerando suas variantes. Por exemplo, podemos criar um Enum chamada State em que cada variante é um estado que já morei na vida:

```rust
enum State {
    minasgerais,
    saopaulo,
}
```

Enums têm muitas propriedades e um tratamento completo pode ser encontrado [aqui](https://doc.rust-lang.org/book/ch06-00-enums.html). Para os propósitos desse post o importante é entender como os Enums se relacionam com um *flow operator* do Rust chamado de *match*. O *match* basicamente roda um código com base em qual variante do Enum foi observada. Por exemplo, posso criar uma função que retorna quantos anos eu morei em cada um dos estados do Enum State:

```rust
fn years(state: State) -> i32 {
    match state {
        minasgerais => 26,
        saopaulo => 4
    }
}
```

O `Result` é um enumerável como o `State` e suas variantes são `Ok(T)` e `Err(E)` em que `T` é o tipo da variável que retorna quando a função roda sem erros e `E` é o erro que retorna caso a função não tenha sucesso. Essa é a assinatura do Enum `Result`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Note que, assim como em State, podemos usar o *match* para criar uma função que nos retorne um valor para cada variante de `Result`. Em particular, se a função rodar corretamente, gostaríamos de ter obtero valor correspondente ao resultado e, caso contrário, gostaríamos de interromper a compilação. 

Vamos ilustar esse exemplo com o método `from_utf8` que converte um vetor de bytes em uma `String`. Como pode ser visto na [documentação](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8), esse método retorna um Enum do tipo Result. Nossa função para usar o método `from_utf8`, tem a seguinte forma:

```rust
fn get_value(emoji: Vec<i32>) -> String {
    let heart = String::from_utf8(emoji);
    match heart {
        Ok(value) => value,
        Err(error) => error,
    }
}
```






