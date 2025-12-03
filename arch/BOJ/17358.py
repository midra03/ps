n = int(input())
ans = 1
mod = int(1e9) + 7
for i in range(1, n + 1, 2):
    ans *= i
    ans %= mod
print(ans)