# O que significa ponto de interrogação em Rust?

05/07/2021

No post [anterior](https://lucascr91.github.io/crustacea/unwrap) vimos que o `unwrap` é um método utilizado quando uma função retorna um Enum do tipo `Result` ou `Option`. Você deve ter visto por aí um método similar, em que se usa o método `?`. Qual a diferença desse método para o `unwrap`? Em que contexto ele é usado? Neste post, tentamos responder essas duas perguntas.

## Duas diferenças fundamentais

O método `?` é um método de "propagação de erro", ou seja, em vez de lidar diretamente com o erro, ele relega essa tarefa para a função em que o próprio método é implementado. Ele se parece com o `unwrap` já que lida com os Enums `Result` e `Option`. No entanto, existem duas diferenças fundamentais:

1. O método `?` não chama a macro `panic!`, em caso de não sucesso da função.
2. O método `?` só funciona dentro de funções que retornam um Enum do tipo `Result`

Este é um exemplo do uso do método `?`:

```rust
fn main() {
    let example = vec![240, 159, 146, 150];
    println!("{:?}", get_value(example).unwrap())
}

fn get_value(emoji: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    let heart = String::from_utf8(emoji)?;
    return Ok(heart);
}
```
Note que, diferentemente da implementação do `unwrap` que vimos no post anterior, dessa vez o `unwrap` não é feito na função `String::from_utf8(emoji)`. Em vez disso, o erro é propagado por `?` e precisamos usar o `unwrap` só quando usamos a função `get_value`.

## Qual a vantagem do método de propagação de erro?

Não está claro para mim a resposta dessa pergunta. Ao que parece, a vantagem seria concentrar a gestão de erros em uma parte específica do código. Por exemplo, você cria várias funções utilizando a propagação de erro e usa métodos como `unwrap` e `expect` apenas quando de fato for usar essas funções.