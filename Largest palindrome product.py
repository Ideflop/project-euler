"""
A palindromic number reads the same both ways. 
The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
"""

def largest_palindrome():
    i = 999
    j = 999
    lst = []
    while j != 99:
        k = str(i*j)
        if len(k) == 6:
            if k[0] == k[-1] and k[1] == k[-2] and k[2] == k[-3]:
                lst.append(i*j)
            if i == 100:
                j -= 1
                i = 999
            else:
                i -= 1
        elif len(k) == 5:
            if k[0] == k[-1] and k[1] == k[-2]:
                lst.append(i*j)
            if i == 100:
                j -= 1
                i = 999
            else:
                i -= 1
    return max(lst)

if __name__ == '__main__':
    print(largest_palindrome())