"""
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
                            a² + b² = c²

For example, 3² + 4² = 9 + 16 = 25 = 5².

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
"""

from math import sqrt
def triplet():
    for a in range(1,500):
        for b in range(a,500):
            if (sqrt(a ** 2 + b ** 2) % 1 == 0) and a + b + sqrt(a ** 2 + b ** 2) == 1000:
                return int(a * b * sqrt(a ** 2 + b ** 2))
                
if __name__ == '__main__':
    print(triplet())