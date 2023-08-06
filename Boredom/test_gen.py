#!/usr/bin/env python3
import sys
import random


def generate_random_numbers(n):
    return [random.randint(1, 10**5) for _ in range(n)]


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python program.py <n>")
    else:
        n = int(sys.argv[1])
        random_numbers = generate_random_numbers(n)
        print(n)
        print(" ".join(map(str, random_numbers)))
