def main():
    products = []
    for i in range(999, 99, -1):
        for j in range(999, 99, -1):
            if str(i * j) == str(i * j)[::-1]:
                products.append(i * j)
    return max(products)

if __name__ == "__main__":
    print(main())