""" 
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10001st prime number?
"""

def Prime(n):
    if n in [2, 3]:
        return True
    if n%2==0 or n<2:
        return False
    return all(n%i != 0 for i in range(3, int(n**0.5)+1, 2))


def prime():
    prime = [n for n in range(2,200000) if Prime(n)]
    print(prime[10000])

if __name__ == '__main__':
    print(prime())