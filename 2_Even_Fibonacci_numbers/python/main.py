# Not very nice
def fib_lessthan(n):
    """
    Returns the fibonacci sequence with numbers not
    exceeding n
    """
    arr = [1, 2]
    i = 1
    while True:
        if arr[i] + arr[i - 1] > n:
            return arr
        arr.append(arr[i] + arr[i - 1])
        i += 1

# I like the functional bit
if __name__ == "__main__":
    print(sum(filter(lambda x: x % 2 == 0, fib_lessthan(4_000_000))))