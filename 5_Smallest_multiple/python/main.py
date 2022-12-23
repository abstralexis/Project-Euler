"""
This actually works, it is just really slow.
Like, 5:23.23... slow.
"""

def smallest_multiple(n: int) -> int:
    i: int = 0
    while True:
        i += 1
        results = filter(lambda x: i % x == 0, range(1, n+1))
        if len(list(results)) == n:
            return i

if __name__ == "__main__":
    print(smallest_multiple(20))