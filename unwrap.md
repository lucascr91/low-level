# O que significa unwrap em Rust?

26/06/2021

Quando comecei a aprender Rust uma das coisas que achava mais estranhas era o método `unwrap`. Eu via isso em todos os códigos e simplesmente não sabia o que significava. A traduação de "wrap" é embrulho, então "unwrap" é algo como "desembrulhar". Desembrulhar o que oras? Bom, para entender esse método é preciso entender o conceito de *error handling* in Rust, vamos fazer um tour agora sobre esse conceito e, ao final do post, a gente volta no `unwrap` para entender o que, afinal, estamos desembrulhando.

## Error handling

A primeira coisa a entender sobre erros em Rust é que existem dois tipos de erros: recuperáveis e não-recuperáveis. Segue uma tabela resumindo a diferença entre os dois:

|            	| Recuperável 	| Não-Recuperável 	|
|------------	|-------------	|-----------------	|
| tipo/macro 	| Result<T,E> 	|      panic!     	|
| Interrompe 	| run time    	| compile time    	|

O método `unwrap` é usado no contexto de erros recuperáveis, que aplicam o enumerável Result<T,E>. Enumeráveis são chamados simplesmente de `Enums` em Rust e são estruturas que permitem o usuário definir um tipo de objeto enumerando suas variantes. Por exemplo, podemos criar um Enum chamada State em que cada variante é um estado que já morei na vida:

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






