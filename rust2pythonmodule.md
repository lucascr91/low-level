# Como escrever um módulo do Python em Rust

## Introdução

Você já deve ter percebido que muitas funções e métodos em Python têm uma performance muito melhor do que funções definidas pelo usuário. Um exemplo é o método `np.sum`, do módulo `numpy`. Veja como essa função se compara com `my_sum`, uma função definida por mim:

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

Neste post, vou mostrar como escrever um módulo do Python em Rust. Além desta introdução, o post conta com mais três seções. Na seção seguinte, apresento o algoritmo X, que servirá de exemplo para nossa implementação. Primeiro escrevo o algoritmo em Python, depois escrevo o mesmo algoritmo em Rust. Com o código do algoritmo em Rust pronto, na terceira seção mostro o passo a passo para implementá-lo a partir de um módulo do Python. A última seção concluí o post e, no melhor espírito do blog, indico minhas dúvidas e questões sobre o tema.

