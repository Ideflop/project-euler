"""
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
"""
import math

def sm_divid_number():
    lst = [j for j in range(1,21)]
    while len(lst) > 1:
        lst[1] = abs(int(lst[0] * lst[1])) / math.gcd(int(lst[0]), int(lst[1]))
        lst.pop(0)
    return int(lst[0])

if __name__ == '__main__':
    print(sm_divid_number())