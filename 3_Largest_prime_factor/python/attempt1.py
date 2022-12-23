"""
This implementation is really slow :c there are better ways
of doing it but it does not looks as nicely functional and
I am too stubborn to copy one and then rewrite it in my own
way :(
"""

from math import sqrt

def is_prime(n: int) -> bool:
    if n % 2 == 0:
        return True
    
    for i in range(3, int(sqrt(n) + 1), 2):
        if n % i == 0:
            return False
    return True

def is_factor_of(n: int):
    def is_factor(i) -> bool:
        return n % i == 0
    return is_factor

def highest_prime_factor(n: int):
    return max(filter(is_prime, filter(is_factor_of(n), range(2, n//2))))

def main():
    print(f"{highest_prime_factor(600851475143)}")

if __name__ == "__main__":
    main()
    
