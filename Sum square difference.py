"""
The sum of the squares of the first ten natural numbers is,
                1²+2²+...+10² = 385
The square of the sum of the first ten natural numbers is,
                (1+2...+10)² = 55² = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is :
                3025 - 385 = 2640
Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
"""
def sum_squares():
    a = (sum(range(1,101)))**2
    lst = [i for i in range(1,101)]
    for i, numbers in enumerate(lst):
        lst[i] = lst[i]**2
    return a-sum(lst)

if __name__ == '__main__':
    print(sum_squares())