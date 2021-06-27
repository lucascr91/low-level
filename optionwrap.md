# O que significa unwrap em Rust?

26/06/2021

## Introdução

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

Um tipo de Enum muito importante em Rust é o `Option`. O `Option` é um Enum que está sempre disponível para o usuário e cobre a situação muito comum em que o objeto sendo manipulado pode ser de algum tipo conhecido (String, i32, etc) ou não ser nada. Essa é a assinatura do `Option`:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Onde T significa "um tipo qualquer de objeto conhecido". Note que também podemos usar o *match* para `Option`, já que ele é apenas um caso específico de Enum. Por exemplo, nós podemos criar uma função que tem como input uma `Option<&str>`, ou seja, uma Option com uma slice de string como tipo e retorna a versão capitalizada da string, caso uma string seja usada e `None` caso não tenha string:

```rust
fn name_upper(name:Option<&str>) -> Option<String> {
    match name {
        None => None,
        Some(i) => Some(i.to_uppercase()),
    }
}
//testando a funçao
fn main() {
    let first = Some("Lucas");
    println!("{:?}", name_upper(first))
}
```






