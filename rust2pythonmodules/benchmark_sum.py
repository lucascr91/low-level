from timeit import timeit
import numpy as np
import random
import sys

k=int(sys.argv[1])

lista= random.choices(range(10), k=k)


def my_sum(numbers:list):
	if len(numbers)>1:
		return numbers[0]+my_sum(numbers[1:])
	else:
		return numbers[0]

params = { 'number' : 100000, 'globals': globals() }
print("Sum timing using user defined function:\n",timeit('my_sum(lista)', **params))
print("Sum timing using numpy:\n",timeit('np.sum(lista)', **params))
