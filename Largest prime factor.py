"""
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
"""

def largest_prime_factor():
    i = 2
    lst = []
    number = 600851475143
    while i != number:
        if number % i == 0:
            lst.append(i)
            number /= i
            i = 2
        else:
            i += 1
    
    lst.append(i)
    return lst[-1]
    
if __name__ == '__main__':
    print(largest_prime_factor())