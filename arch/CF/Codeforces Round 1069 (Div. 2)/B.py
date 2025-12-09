def sol():
    n, l, r = map(int, input().split())
    b = []
    for i in range(n + 1):
        b.append(i + 1)
    b[l - 1] = b[r]
    a = [0 for _ in range(n + 1)]
    for i in range(1, n + 1):
        print(b[i - 1] ^ b[i], end=' ')
    print()

for _ in range(int(input())):
    sol()
