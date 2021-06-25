## Como escrever um módulo do Python em Rust

Você já deve ter percebido que muitas funções e métodos em Python têm uma performance muito melhor do que funções definidas pelo usuário. Um exemplo é o método `np.sum`, do módulo `numpy`. Veja como essa função se compara com `my_sum`, uma função definida por mim:


```python
def my_sum(numbers:list):
	if len(numbers)>1:
		return numbers[0]+my_sum(numbers[1:])
	else:
		return numbers[0]
```

Usando o módulo `timeit`, eu obtive os seguintes valores na minha máquina para listas com 50, 100 e 200 números:

### Diferença de performance entre np.sum e função definida pelo usuário
| Tamanho da lista 	| my_sum 	| np.sum 	|
|------------------	|--------	|--------	|
| 50               	| 1.07   	| 0.67   	|
| 100              	| 2.84   	| 0.93   	|
| 10000            	| 7.68   	| 1.46   	|
Notas: Valores em segundos nas colunas 2 e 3.

