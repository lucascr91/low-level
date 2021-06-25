## Como escrever um módulo do Python em Rust

Você já deve ter percebido que muitas funções e métodos em Python têm uma performance muito melhor do que funções definidas pelo usuário. Um exemplo é o método `np.sum`, do módulo `numpy`. Veja essa comparação entre a `np.sum` e a `my_sum`, uma função definida por mim:


```python
def my_sum(numbers:list):
	if len(numbers)>1:
		return numbers[0]+my_sum(numbers[1:])
	else:
		return numbers[0]
```
