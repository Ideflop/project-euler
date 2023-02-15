"""
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
"""

def Prime(n):
    if n in [2, 3]:
        return True
    if n%2==0 or n<2:
        return False
    return all(n%i != 0 for i in range(3, int(n**0.5)+1, 2))


def sum_primes():
    prime = [n for n in range(2,2000000) if Prime(n)]
    return sum(prime)

if __name__ == '__main__':
    print(sum_primes())