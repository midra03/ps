n, k = map(int, input().split())

ok = k * (k + 1) // 2

if n < ok:
    print(-1)
elif (n - ok) % k == 0:
    print(k - 1)
else:
    print(k)